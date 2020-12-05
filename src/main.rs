use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use log::{info, warn};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Webserver running!");
    HttpServer::new(|| App::new().service(hello).service(echo))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
