mod scanner;
mod class;
mod hash_service;
mod config;
mod utils;
mod collector;
mod daemon;
#[cfg(test)]
mod test;

use actix_web::{get, App, HttpServer, Responder, HttpResponse};
use crate::config::read_config;

#[get("/v3/ept")]
async fn ept() -> impl Responder {
    let config = read_config().unwrap();
    let dir_res = collector::ept(&config).unwrap();
    HttpResponse::Ok().json(dir_res)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(ept)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}