mod scanner;
mod class;
mod hash;

use actix_web::{get, App, HttpServer, Responder, HttpResponse};

#[get("/v3/ept")]
async fn ept() -> impl Responder {
    let dir_res = scanner::scan_plugins(String::from("D:/Download/HubCache/Burn"));
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