use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::path;
use std::hash::Hasher;

use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use log::{info, warn};

use handlebars::{Handlebars, RenderError};
use sass_rs::{compile_file, Options};

use serde::Serialize;

mod project;
mod projects;
use project::{Project};
use projects::{Projects};

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

extern crate yaml_rust;
use yaml_rust::{Yaml, YamlEmitter, YamlLoader};

extern crate notify;
use notify::{watcher, DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::iter::FromIterator;
// thanks rust-lang git repo for the sass compile stuff

#[derive(Clone, Serialize)]
struct CSSFiles {
    app: String,
    fonts: String,
    // pages: HashMap<String, String>
    //vendor: String,
}
#[derive(Clone, Serialize)]
struct JSFiles {
    app: String,
}
#[derive(Clone, Serialize)]
struct AssetFiles {
    css: CSSFiles,
    //js: JSFiles,
}

struct ServerConfig {
    dev: bool,
}

lazy_static! {
    static ref ASSETS: AssetFiles = {
        let app_css_file = compile_sass("app");
        let fonts_css_file = compile_sass("fonts");
        //let vendor_css_file = concat_vendor_css(vec!["tachyons"]);
        //let app_js_file = concat_app_js(vec!["tools-install"]);

        AssetFiles {
            css: CSSFiles {
                app: app_css_file,
                fonts: fonts_css_file
            //    vendor: vendor_css_file,
            },
            //js: JSFiles { app: app_js_file },
        }
    };

    static ref CONFIG: ServerConfig = {
        let dev = match env::var("DEV") {
            Ok(_val) => true,
            Err(_e) => false,
        };
        ServerConfig {
            dev: dev
        }
    };
}

macro_rules! compileOrFetch {
    ($data:expr, $name:literal, $assetA:tt.$assetB:tt, $compileFunc:tt) => {
        if CONFIG.dev {
            $compileFunc($name)
        } else {
            String::from(&$data.assets.$assetA.$assetB)
        }
    };
}

struct AppState<'a> {
    hb: web::Data<Handlebars<'a>>,
    projects: Arc<Mutex<Projects>>,
    assets: &'a AssetFiles,
}

#[get("/")] // TODO: actually learn about lifetime specifiers
async fn index(data: web::Data<AppState<'_>>) -> impl Responder {
    render_template(String::from("index"), data, None)
}

#[get("/projects")]
async fn get_projects(data: web::Data<AppState<'_>>) -> impl Responder {
    let all = &data.projects.lock().unwrap().value();
    let all = all.lock().unwrap();
    let categories = &data.projects.lock().unwrap().categories();
    let categories = categories.lock().unwrap();

    let map: HashMap<String, Vec<Project>> = categories.iter().map(|(k,v)| {
        let projects: Vec<Project> = v.iter().map(|id| {
            all.get(id).unwrap().clone()
        }).collect();
        return (k.clone(), projects)
    }).collect();

    render_template(String::from("projects"), data, Some(json!({
        "categories": map,
    })))
}

// FIXME: project url with trailing '/' does not render
#[get("/projects/{id}")]
async fn get_project(
    web::Path(id): web::Path<String>,
    data: web::Data<AppState<'_>>,
) -> impl Responder {
    let proj = &data.projects.lock().unwrap().value();
    let map = proj.lock().unwrap();
    println!("project {}", id);
    render_template(String::from("project"), data, Some(json!({
        "project": map.get(&id)
    })))
}

#[get("/{page}")]
async fn get_page(
    web::Path(page): web::Path<String>,
    data: web::Data<AppState<'_>>,
) -> impl Responder {
    println!("page {}", page);
    render_template(page, data, None)
}

fn render_fail_wrapper(
    res: Result<String, RenderError>,
) -> actix_web::web::HttpResponse<actix_web::dev::Body> {
    match res {
        Ok(content) => HttpResponse::Ok().body(content),
        Err(_) => HttpResponse::Ok().body("<h1>404</h1>"),
    }
}

/*

render(data) {
    const final_data = {
        app_css:
        ...data
    }
    // do render things with final_data
}

*/

fn render_template(
    page: String,
    data: web::Data<AppState<'_>>,
    json: Option<serde_json::Value>
) -> actix_web::web::HttpResponse<actix_web::dev::Body> {
    println!("Template request for '{}'", page);
    let mut dm = json!({
        "page": page,
        "app_css": compileOrFetch! (data, "app", css.app, compile_sass),
    });

    let d = dm.as_object_mut().unwrap();

    match json {
        Some(j) => {
            j.as_object().unwrap().iter().for_each(|(k,v)| {
                d.insert(k.to_string(),v.clone());
            });
        },
        None => {}
    }

    let d = json!(d);
    
    if CONFIG.dev {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_templates_directory(".hbs", "./src/templates")
            .unwrap();
        render_fail_wrapper(handlebars.render(page.as_str(), &d))
    } else {
        render_fail_wrapper((&data.hb).render(page.as_str(), &d))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    lazy_static::initialize(&ASSETS);
    // Handlebars uses a repository for the compiled templates. This object must be
    // shared between the application threads, and is therefore passed to the
    // Application Builder as an atomic reference-counted pointer.
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", "./src/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);
    let mut projects = Projects::new();
    projects
        .watcher
        .watch("./src/projects/", RecursiveMode::Recursive)
        .unwrap();

    let paths = fs::read_dir("./src/projects/").unwrap();
    for path in paths {
        projects.process(path.unwrap().path());
    }

    let projects_ref = Arc::new(Mutex::new(projects));

    println!("Webserver running!");
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                hb: handlebars_ref.clone(),
                assets: &ASSETS,
                projects: projects_ref.clone(),
            })
            .service(Files::new("/static", "./static"))
            .service(index)
            .service(get_projects)
            .service(get_project)
            .service(get_page)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}

fn hash_css(css: &str) -> String {
    let mut hasher = DefaultHasher::new();
    hasher.write(css.as_bytes());
    hasher.finish().to_string()
}

fn compile_sass(filename: &str) -> String {
    println!("Compiling '{}.css'...", filename);
    let scss_file = format!("./src/styles/{}.scss", filename);

    if !path::Path::new(&scss_file).exists() {
        let path = std::env::current_dir().unwrap();
        println!("The current directory is {}", path.display());
        panic!("file not found: {}", scss_file);
    }

    let css = compile_file(&scss_file, Options::default())
        .unwrap_or_else(|_| panic!("couldn't compile sass: {}", &scss_file));

    // let css_sha = format!("{}_{}", filename, hash_css(&css));
    let css_file = format!("./static/styles/{}.css", filename);

    fs::write(&css_file, css.into_bytes())
        .unwrap_or_else(|_| panic!("couldn't write css file: {}", &css_file));

    String::from(&css_file[1..])
}
