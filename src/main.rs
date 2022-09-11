mod bridge;
mod class;
mod config;
mod constant;
mod daemon;
mod utils;

#[cfg(test)]
mod test;

use std::{sync::mpsc::channel, thread::spawn};

use crate::{
    bridge::Bridge,
    config::{read_config, Config},
};
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer};
use casual_logger::{Log, Opt};
use class::TokenRequiredQueryStruct;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    // 全局变量
    static ref BRIDGE: Arc<Mutex<Option<Bridge>>> = Arc::new(Mutex::new(None));
    static ref CONFIG:Arc<Mutex<Option<Config>>> = Arc::new(Mutex::new(None));
}

#[get("/api/v3/hello")]
async fn ept_hello_handler() -> HttpResponse {
    match BRIDGE.lock().unwrap().as_mut().map(|v| v.hello()) {
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

#[get("/api/v3/alpha")]
async fn ept_alpha_handler(info: web::Query<TokenRequiredQueryStruct>) -> HttpResponse {
    let config_guard = CONFIG.lock().unwrap();
    let config = config_guard.as_ref().unwrap();
    if info.token == config.token.alpha {
        return match BRIDGE.lock().unwrap().as_mut().map(|v| v.alpha()) {
            Some(Ok(res)) => HttpResponse::Ok().json(res),
            Some(Err(e)) => {
                Log::error(&format!("Can't collect alpha response {:?}", e));
                Log::flush();
                HttpResponse::InternalServerError().body("Can't collect alpha response")
            }
            None => {
                Log::error("Can't collect alpha response: Uninit");
                Log::flush();
                HttpResponse::InternalServerError().body("Can't collect alpha response")
            }
        };
    }
    HttpResponse::BadRequest().body("Invalid token")
}

#[get("/api/v3/refresh")]
async fn ept_refresh_handler(info: web::Query<TokenRequiredQueryStruct>) -> HttpResponse {
    let config_guard = CONFIG.lock().unwrap();
    let config = config_guard.as_ref().unwrap();
    let mut collector_guard = BRIDGE.lock().unwrap();
    let collector = collector_guard.as_mut().unwrap();
    if info.token == config.token.super_user {
        collector.update_cache(false);
        return HttpResponse::Ok().body("Requested refresh");
    }
    HttpResponse::BadRequest().body("Invalid token")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    Log::set_opt(Opt::Release);
    Log::remove_old_logs();

    let cfg = read_config().unwrap();

    let (result_sender, result_receiver) = channel();
    let (cmd_sender, cmd_receiver) = channel();
    let mut daemon = daemon::Daemon::new(cmd_receiver, result_sender, cfg.clone());

    // 初始化全局变量
    *(BRIDGE.lock().unwrap()) = Some(Bridge::new(result_receiver, cmd_sender));
    *(CONFIG.lock().unwrap()) = Some(cfg);

    //启动 daemon 服务
    spawn(move || {
        daemon.request(true, true);
        daemon.serve();
    });

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(ept_hello_handler)
            .service(ept_refresh_handler)
            .service(ept_alpha_handler)
    })
    .bind(("127.0.0.1", 8383))?
    .run()
    .await
}
