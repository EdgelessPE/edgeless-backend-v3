use std::fs;
use std::sync::mpsc::Sender;
use std::time::SystemTime;
use std::path::Path;

use crate::class::EptResponse;
use crate::scanner::Scanner;

struct LazyDeleteNode {
    path:String,
    key:String
}
struct Daemon {
    timestamp_recent_finish: SystemTime, //上次扫描结束时的时间戳
    status_running: bool, //是否有一个扫描任务正在进行中
    list_lazy_delete: Vec<LazyDeleteNode>, //懒删除文件列表

    sender: Sender<EptResponse>, //结果发送channel
    scanner:Scanner, //扫描器实例
    dir_packages:String, //插件包所在目录
}
impl Daemon {
    pub fn new(sender: Sender<EptResponse>,dir_packages:String,scanner:Scanner) -> Self {
        Daemon {
            timestamp_recent_finish: SystemTime::UNIX_EPOCH,
            status_running: false,
            list_lazy_delete: vec![],
            sender,
            dir_packages,
            scanner,
        }
    }

    //由外部调用，要求安排一次扫描更新
    pub fn request(&mut self){
        //判断是否使能扫描
        if !self.status_running && SystemTime::now().duration_since(self.timestamp_recent_finish).unwrap().as_secs() > 5*60 {
            self.status_running=true;
            self.update();
            self.timestamp_recent_finish=SystemTime::now();
            self.status_running=false;
        }
    }

    //执行一次更新
    fn update(&mut self)-> std::io::Result<()>{
        println!("Info:Start updating");

        //懒删除
        for node in &self.list_lazy_delete {
            self.scanner.delete_file(node.path.to_owned(), node.key.to_owned())
        }

        //生成


        Ok(())
    }
}
