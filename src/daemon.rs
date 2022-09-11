use std::sync::mpsc::{Receiver, Sender};
use std::time::SystemTime;

use casual_logger::Log;

use crate::assembly_factory::get_general_response;
use crate::class::{AlphaResponse, HelloResponse, LazyDeleteNode};
use crate::config::Config;
use crate::constant::{CALC_HASH_INTERVAL, CMD_REQUEST, SU_REQUEST};
use crate::hash2::IntegrityCache;
use crate::scanner::Scanner;

pub struct Daemon {
    timestamp_recent_finish: SystemTime,   //上次扫描结束时的时间戳
    list_lazy_delete: Vec<LazyDeleteNode>, //懒删除文件列表

    commander: Receiver<String>, //更新请求接收器
    result_sender: Sender<(HelloResponse, AlphaResponse)>, //结果发送channel
    scanner: Scanner,            //扫描器实例
    config: Config,              //配置
}
impl Daemon {
    pub fn new(
        commander: Receiver<String>,
        result_sender: Sender<(HelloResponse, AlphaResponse)>,
        config: Config,
    ) -> Self {
        let hash_service = IntegrityCache::new();
        let scanner = Scanner::new(hash_service);
        Daemon {
            timestamp_recent_finish: SystemTime::UNIX_EPOCH,
            list_lazy_delete: vec![],
            result_sender,
            config,
            scanner,
            commander,
        }
    }

    pub fn serve(&mut self) {
        let cmd_request = String::from(CMD_REQUEST);
        let su_request = String::from(SU_REQUEST);
        while let Ok(cmd) = self.commander.recv() {
            // println!("Daemon Info:Get cmd : {}", &cmd);
            if cmd == cmd_request {
                self.request(false, false);
            } else if cmd == su_request {
                self.request(false, true);
            }
        }
    }

    //请求安排一次扫描更新
    pub fn request(&mut self, clear_hash_map: bool, force: bool) {
        //判断是否使能扫描
        if force
            || SystemTime::now()
                .duration_since(self.timestamp_recent_finish)
                .unwrap()
                .as_secs()
                > CALC_HASH_INTERVAL
        {
            let update_res = self.update(clear_hash_map);
            if let Err(err) = update_res {
                Log::error(&format!("Can't update packages : {:?}", err));
            }
            self.timestamp_recent_finish = SystemTime::now();
        }
    }

    //执行一次更新
    fn update(&mut self, _clear_hash_map: bool) -> std::io::Result<()> {
        Log::info("Start updating");
        println!("Info:Start updating");

        //插件包懒删除
        for node in &self.list_lazy_delete {
            self.scanner
                .delete_file(node.path.to_owned(), node.key.to_owned())
        }

        //获取扫描结果
        let (hello_response, alpha_response, lazy_delete_list) =
            get_general_response(&mut self.scanner, &self.config).unwrap();

        //发送结果
        self.result_sender
            .send((hello_response, alpha_response))
            .unwrap();

        //更新懒删除列表
        self.list_lazy_delete = lazy_delete_list;

        Log::info("Finish updating");
        println!("Info:Finish updating");

        Log::remove_old_logs();
        Log::flush();
        Ok(())
    }
}
