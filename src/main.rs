mod class;
mod config;
mod constant;
mod daemon;
mod hash_service;
mod response_collector;
mod scanner;
mod utils;

#[cfg(test)]
mod test;

use std::{sync::mpsc::channel, thread::spawn};

use crate::config::{read_config, Config};
use actix_web::{get, web, App, HttpResponse, HttpServer};
use casual_logger::Log;
use class::TokenRequiredQueryStruct;
use lazy_static::lazy_static;
use response_collector::ResponseCollector;
use std::sync::{Arc, Mutex};
use utils::get_service;

lazy_static! {
    // 全局变量
    static ref COLLECTOR: Arc<Mutex<Option<ResponseCollector>>> = Arc::new(Mutex::new(None));
    static ref CONFIG:Arc<Mutex<Option<Config>>> = Arc::new(Mutex::new(None));
}

#[get("/api/v3/hello")]
async fn ept_hello_handler() -> HttpResponse {
    match COLLECTOR.lock().unwrap().as_mut().map(|v| v.hello()) {
        Some(Ok(res)) => HttpResponse::Ok().json(res),
        Some(Err(e)) => {
            Log::error(&format!("Can't collect ept response {:?}", e));
            Log::flush();
            HttpResponse::InternalServerError().body("Can't collect ept response")
        }
        None => {
            Log::error("Can't collect ept response: Uninit");
            Log::flush();
            HttpResponse::InternalServerError().body("Can't collect ept response")
        }
    }
}

#[get("/api/v3/ept/refresh")]
async fn ept_refresh_handler(info: web::Query<TokenRequiredQueryStruct>) -> HttpResponse {
    let config_guard = CONFIG.lock().unwrap();
    let config = config_guard.as_ref().unwrap();
    let mut collector_guard = COLLECTOR.lock().unwrap();
    let collector = collector_guard.as_mut().unwrap();
    if info.token == config.token.alpha {
        collector.ept_refresh(false);
        return HttpResponse::Ok().body("Requested refresh");
    } else if info.token == config.token.super_user {
        collector.ept_refresh(true);
        return HttpResponse::Ok().body("Requested force refresh");
    }
    HttpResponse::BadRequest().body("Invalid token")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    Log::remove_old_logs();

    let cfg = read_config().unwrap();

    let (result_sender, result_receiver) = channel();
    let (cmd_sender, cmd_receiver) = channel();
    let mut daemon = daemon::Daemon::new(
        cmd_receiver,
        result_sender,
        get_service(&cfg.mirror.services, String::from("plugins"))
            .unwrap()
            .local,
    );

    // 初始化全局变量
    *(COLLECTOR.lock().unwrap()) = Some(ResponseCollector::new(
        result_receiver,
        cmd_sender,
        cfg.clone(),
    ));

    *(CONFIG.lock().unwrap()) = Some(cfg);

    //启动 daemon 服务
    spawn(move || {
        daemon.request(true, true);
        daemon.serve();
    });

    HttpServer::new(|| {
        App::new()
            .service(ept_hello_handler)
            .service(ept_refresh_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
