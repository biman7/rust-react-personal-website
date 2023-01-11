use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use serde::{Serialize};
use std::env;

mod utils;
mod config;

#[derive(Serialize)]
struct HomePageProps {
    message: String
}

#[get("/")]
async fn show_home() -> impl Responder {
    let component = String::from("Home");
    let props = HomePageProps {
        message: String::from("Hello World!"),
    };
    HttpResponse::Ok().body(utils::renderer::render_with_props(component, props))
}
    
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init();
    HttpServer::new(|| {
        App::new()
        .service(show_home)
        .service(fs::Files::new("/", "./public").show_files_listing())
    })
    .bind((env::var("HOST").unwrap(), env::var("PORT").map_or(8080, |p| p.parse::<u16>().unwrap())))?
    .run()
    .await
}