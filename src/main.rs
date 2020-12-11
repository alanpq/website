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

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

// thanks rust-lang git repo for the sass compile stuff

#[derive(Clone, Serialize)]
struct CSSFiles {
    app: String,
    fonts: String,
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
                fonts: fonts_css_file,
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
    assets: &'a AssetFiles,
}

#[get("/")] // TODO: actually learn about lifetime specifiers
async fn index(data: web::Data<AppState<'_>>) -> impl Responder {
    render_template(String::from("index"), data)
}

#[get("/{page}")]
async fn get_page(
    web::Path(page): web::Path<String>,
    data: web::Data<AppState<'_>>,
) -> impl Responder {
    println!("page {}", page);
    render_template(page, data)
}

fn render_fail_wrapper(
    res: Result<String, RenderError>,
) -> actix_web::web::HttpResponse<actix_web::dev::Body> {
    match res {
        Ok(content) => HttpResponse::Ok().body(content),
        Err(_) => HttpResponse::Ok().body("<h1>404</h1>"),
    }
}

fn render_template(
    page: String,
    data: web::Data<AppState<'_>>,
) -> actix_web::web::HttpResponse<actix_web::dev::Body> {
    println!("Template request for '{}'", page);
    let d = json!({
        "page": page,
        "app_css": compileOrFetch! (data, "app", css.app, compile_sass),
    });
    if CONFIG.dev {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_templates_directory(".handlebars", "./src/templates")
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
        .register_templates_directory(".handlebars", "./src/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    println!("Webserver running!");
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                hb: handlebars_ref.clone(),
                assets: &ASSETS,
            })
            .service(Files::new("/static", "./static"))
            .service(index)
            .service(get_page)
    })
    .bind("0.0.0.0:8080")?
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

    assert_eq!(path::Path::new(&scss_file).exists(), true);

    let css = compile_file(&scss_file, Options::default())
        .unwrap_or_else(|_| panic!("couldn't compile sass: {}", &scss_file));

    let css_sha = format!("{}_{}", filename, hash_css(&css));
    let css_file = format!("./static/styles/{}.css", css_sha);

    fs::write(&css_file, css.into_bytes())
        .unwrap_or_else(|_| panic!("couldn't write css file: {}", &css_file));

    String::from(&css_file[1..])
}
