use std::env;
use std::fs;

use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use log::debug;
use log::{info, warn};

use handlebars::{Handlebars, RenderError};

use serde::Serialize;

mod categories;
mod scss;

mod project;
mod projects;
use project::Project;
use projects::Projects;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

extern crate yaml_rust;

extern crate notify;
use notify::{RecursiveMode, Watcher};
use std::sync::{Arc, Mutex};

use crate::categories::Categories;
use crate::categories::Category;
use crate::categories::get_categories;
use crate::scss::watch_css;
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
	static ref CONFIG: ServerConfig = {
		let dev = match env::var("DEV") {
			Ok(_val) => true,
			Err(_e) => false,
		};
		ServerConfig { dev: dev }
	};
}

struct AppState<'a> {
	hb: web::Data<Handlebars<'a>>,
	projects: Arc<Mutex<Projects>>,
	categories: Arc<Categories>,
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

	let order = data.categories.to_owned();

	let map: Vec<(Category, Vec<Project>)> = order
		.iter()
		.filter_map(|k| {
			let projects: Vec<Project> = categories.get(&k.name)?.iter().map(|id| all.get(id).unwrap().clone()).collect();
			return Some((k.clone(), projects));
		})
		.collect();

	debug!("{:?}", order);

	render_template(
		String::from("projects"),
		data,
		Some(json!({
			"categories": map,
			"order": *order,
		})),
	)
}

// FIXME: project url with trailing '/' does not render
#[get("/projects/{id}")]
async fn get_project(
	web::Path(id): web::Path<String>,
	data: web::Data<AppState<'_>>,
) -> impl Responder {
	let proj = &data.projects.lock().unwrap().value();
	let map = proj.lock().unwrap();
	debug!("project {}", id);
	render_template(
		String::from("project"),
		data,
		Some(json!({
			"project": map.get(&id)
		})),
	)
}

#[get("/{page}")]
async fn get_page(
	web::Path(page): web::Path<String>,
	data: web::Data<AppState<'_>>,
) -> impl Responder {
	debug!("page {}", page);
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

fn render_template(
	page: String,
	data: web::Data<AppState<'_>>,
	json: Option<serde_json::Value>,
) -> actix_web::web::HttpResponse<actix_web::dev::Body> {
	info!("Template request for '{}'", page);
	let mut dm = json!({
		"page": page,
	});

	let d = dm.as_object_mut().unwrap();

	match json {
		Some(j) => {
			j.as_object().unwrap().iter().for_each(|(k, v)| {
				d.insert(k.to_string(), v.clone());
			});
		}
		None => {}
	}

	let d = json!(d);
	render_fail_wrapper((&data.hb).render(page.as_str(), &d))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	simple_logger::init_with_env().unwrap();
	// Handlebars uses a repository for the compiled templates. This object must be
	// shared between the application threads, and is therefore passed to the
	// Application Builder as an atomic reference-counted pointer.
	let mut handlebars = Handlebars::new();
	handlebars.set_dev_mode(CONFIG.dev);

	if CONFIG.dev {
		warn!("Running in development mode! This may affect performance.");
	}

	handlebars
		.register_templates_directory(".hbs", "./src/templates")
		.unwrap();
	let handlebars_ref = web::Data::new(handlebars);

	let categories = get_categories("./projects/categories.yml").unwrap();

	let mut projects = Projects::new();
	projects
		.watcher
		.watch("./projects/", RecursiveMode::Recursive)
		.unwrap();

	let paths = fs::read_dir("./projects/").unwrap();
	for path in paths {
		projects.process(path.unwrap().path());
	}

	let projects_ref = Arc::new(Mutex::new(projects));
	let categories_ref = Arc::new(categories);

	actix_web::rt::spawn(watch_css("./src/styles"));

	info!("Webserver running!");
	HttpServer::new(move || {
		App::new()
			.data(AppState {
				hb: handlebars_ref.clone(),
				projects: projects_ref.clone(),
				categories: categories_ref.clone(),
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
