mod scanner;
mod class;
mod hash;
mod config;

use actix_web::{get, App, HttpServer, Responder, HttpResponse};

#[get("/v3/ept")]
async fn ept() -> impl Responder {
    let dir_res = config::read_config();
    HttpResponse::Ok().json(dir_res.unwrap())
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