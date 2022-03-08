use std::env;
use std::fs;

use actix_files::Files;
use actix_web::{get, web, App, HttpServer, Responder};
use log::debug;
use log::{info, warn};

use handlebars::{Handlebars};

use models::awards::Awards;
use models::categories::Categories;
use models::projects::Projects;

mod util;
mod scss;
mod models;
mod render;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

extern crate yaml_rust;

extern crate notify;
use notify::{RecursiveMode, Watcher};
use std::sync::{Arc, Mutex};

use crate::models::awards::fetch_awards;
use crate::models::categories::Category;
use crate::models::categories::get_categories;
use crate::models::project::Project;
use crate::render::render_template;
use crate::scss::watch_css;

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

pub struct AppState<'a> {
	hb: web::Data<Handlebars<'a>>,
	projects: Arc<Mutex<Projects>>,
	categories: Arc<Categories>,
	awards: Arc<Awards>,
}

#[get("/")] // TODO: actually learn about lifetime specifiers
async fn index(data: web::Data<AppState<'_>>) -> impl Responder {
	render_template(String::from("index"), data, None)
}

#[get("/awards")]
async fn get_awards(data: web::Data<AppState<'_>>) -> impl Responder {
	let awards = data.awards.clone();

	render_template(
		format!("awards"),
		data,
		Some(json!({
			"awards": *awards,
		})),
	)
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

	let awards = fetch_awards("./awards/").unwrap();

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
	let awards = Arc::new(awards);

	actix_web::rt::spawn(watch_css("./src/styles"));

	info!("Webserver running!");
	HttpServer::new(move || {
		App::new()
			.data(AppState {
				hb: handlebars_ref.clone(),
				projects: projects_ref.clone(),
				categories: categories_ref.clone(),
				awards: awards.clone(),
			})
			.service(Files::new("/static", "./static"))
			.service(index)
			.service(get_awards)
			.service(get_projects)
			.service(get_project)
			.service(get_page)
	})
	.bind("0.0.0.0:9090")?
	.run()
	.await
}
