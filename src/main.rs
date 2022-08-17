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

use std::{ sync::mpsc::channel, thread::spawn};

use crate::config::read_config;
use actix_web::{web, App, HttpResponse, HttpServer};
use response_collector::ResponseCollector;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

lazy_static! {
    // 全局 ResponseCollector
    static ref COLLECTOR: Arc<Mutex<Option<ResponseCollector>>> = Arc::new(Mutex::new(None));
}

async fn handler() -> HttpResponse {
    match COLLECTOR.lock().unwrap().as_mut().map(|v| v.ept()) {
        Some(Ok(res)) => HttpResponse::Ok().json(res),
        Some(Err(e)) =>  {
            println!("Error:Can't collect ept response {:?}",e);
            HttpResponse::InternalServerError().body("Can't collect ept response")
        },
        None => {
            println!("Error:Can't collect ept response: Uninit");
            HttpResponse::InternalServerError().body("Can't collect ept response")
        }
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let cfg = read_config().unwrap();

    let (result_sender, result_receiver) = channel();
    let (cmd_sender, cmd_receiver) = channel();
    let mut daemon = daemon::Daemon::new(
        cmd_receiver,
        result_sender,
        cfg.position.plugins.clone(),
    );

    // 初始化全局 ResponseCollector
    *(COLLECTOR.lock().unwrap()) = Some(ResponseCollector::new(result_receiver, cmd_sender, cfg.clone()));

    //启动 daemon 服务
    spawn(move || {
        daemon.request(true);
        daemon.serve();
    });

    HttpServer::new(|| {
        App::new().route("/v3/ept", web::get().to(handler))
        // .service(ept)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
