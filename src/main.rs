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
	path_root: String,
}

lazy_static! {
	static ref CONFIG: ServerConfig = {
		let dev = match env::var("DEV") {
			Ok(_val) => true,
			Err(_e) => false,
		};
		ServerConfig { dev, path_root: env::var("PATH_ROOT").unwrap_or(".".to_string()) }
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
		"awards".to_string(),
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
			Some((k.clone(), projects))
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
		.register_templates_directory(".hbs", format!("{}/src/templates", CONFIG.path_root) )
		.unwrap();
	let handlebars_ref = web::Data::new(handlebars);

	let categories = get_categories(format!("{}/projects/categories.yml", CONFIG.path_root)).unwrap();

	let awards = fetch_awards(format!("{}/awards/", CONFIG.path_root)).unwrap();

	let mut projects = Projects::new();
	projects
		.watcher
		.watch(format!("{}/projects/", CONFIG.path_root), RecursiveMode::Recursive)
		.unwrap();

	let mut paths: Vec<std::path::PathBuf> = fs::read_dir(format!("{}/projects/", CONFIG.path_root)).unwrap().filter_map(|p| p.ok()).map(|p| p.path()).collect();
	paths.sort_by_cached_key(|path| path.file_name()
		.and_then(|s| s.to_str())
		.and_then(|s| s.split('_').next())
		.and_then(|s| s.parse::<u32>().ok())
		.unwrap_or(50)
	);
	for path in paths {
		debug!("found project {}", &path.display());
		projects.process(path);
	}

	let projects_ref = Arc::new(Mutex::new(projects));
	let categories_ref = Arc::new(categories);
	let awards = Arc::new(awards);

	actix_web::rt::spawn(watch_css(format!("{}/src/styles", CONFIG.path_root)));

  let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
  let port = std::env::var("PORT").unwrap_or_else(|_| "9090".to_string());

	info!("Webserver running!");
	HttpServer::new(move || {
		App::new()
			.data(AppState {
				hb: handlebars_ref.clone(),
				projects: projects_ref.clone(),
				categories: categories_ref.clone(),
				awards: awards.clone(),
			})
			.service(Files::new("/static", format!("{}/static", CONFIG.path_root)))
			.service(index)
			.service(get_awards)
			.service(get_projects)
			.service(get_project)
			.service(get_page)
	})
	.bind(format!("{host}:{port}"))?
	.run()
	.await
}
