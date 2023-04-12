use actix_web::{web, HttpResponse};
use handlebars::RenderError;
use log::info;

use crate::AppState;

pub fn render_fail_wrapper(
	res: Result<String, RenderError>,
) -> actix_web::web::HttpResponse<actix_web::dev::Body> {
	match res {
		Ok(content) => HttpResponse::Ok().body(content),
		Err(_) => HttpResponse::Ok().body("<h1>404</h1>"),
	}
}

pub fn render_template(
	page: String,
	data: web::Data<AppState<'_>>,
	json: Option<serde_json::Value>,
) -> actix_web::web::HttpResponse<actix_web::dev::Body> {
	info!("Template request for '{}'", page);
	let mut dm = json!({
		"page": page,
	});

	let d = dm.as_object_mut().unwrap();

	if let Some(j) = json {
		j.as_object().unwrap().iter().for_each(|(k, v)| {
			d.insert(k.to_string(), v.clone());
		});
	}

	let d = json!(d);
	render_fail_wrapper(data.hb.render(page.as_str(), &d))
}