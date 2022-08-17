mod class;
mod config;
mod daemon;
mod hash_service;
mod response_collector;
mod scanner;
mod utils;

#[cfg(test)]
mod test;

use std::{sync::mpsc::channel, collections::HashMap, thread::spawn};

use crate::config::read_config;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let cfg=crate::config::read_config().unwrap();

    let hash_map=HashMap::new();

    let (result_sender,result_receiver)=channel();
    let (cmd_sender,cmd_receiver)=channel();
    let mut daemon=daemon::Daemon::new(cmd_receiver,result_sender,hash_map,cfg.position.plugins.clone());

    //启动 daemon 服务
    spawn(move ||{
        daemon.serve();
    });

    HttpServer::new(|| {
        App::new()
        // .service(ept)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
