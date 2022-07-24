mod scanner;
mod class;

use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

#[get("/v3/ept")]
async fn ept() -> impl Responder {
    let dir_res = scanner::scan_plugins(String::from("I:/Edgeless/OneDrive - 洛阳科技职业学院/插件包"));
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