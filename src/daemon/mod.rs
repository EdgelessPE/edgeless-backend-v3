use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::time::SystemTime;

use crate::class::{LazyDeleteNode, EptFileNode};
use crate::scanner::Scanner;

struct Daemon {
    timestamp_recent_finish: SystemTime,   //上次扫描结束时的时间戳
    status_running: bool,                  //是否有一个扫描任务正在进行中
    list_lazy_delete: Vec<LazyDeleteNode>, //懒删除文件列表

    sender: Sender<HashMap<String, Vec<EptFileNode>>>, //结果发送channel
    scanner: Scanner,            //扫描器实例
    dir_packages: String,        //插件包所在目录
}
impl Daemon {
    pub fn new(sender: Sender<HashMap<String, Vec<EptFileNode>>>, scanner: Scanner, dir_packages: String) -> Self {
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
    pub fn request(&mut self) {
        //判断是否使能扫描
        if !self.status_running
            && SystemTime::now()
                .duration_since(self.timestamp_recent_finish)
                .unwrap()
                .as_secs()
                > 5 * 60
        {
            self.status_running = true;
            let update_res=self.update();
            if let Err(err)=update_res{
                println!("Error:Can't update packages : {:?}",err);
            }
            self.timestamp_recent_finish = SystemTime::now();
            self.status_running = false;
        }
    }

    //执行一次更新
    fn update(&mut self) -> std::io::Result<()> {
        println!("Info:Start updating");

        //懒删除
        for node in &self.list_lazy_delete {
            self.scanner
                .delete_file(node.path.to_owned(), node.key.to_owned())
        }

        //生成新的扫描结果和懒删除列表
        let (result,lazy_delete_list)=self.scanner.scan_packages(self.dir_packages.clone())?;
        
        //发送结果
        self.sender.send(result);

        //更新懒删除列表
        self.list_lazy_delete=lazy_delete_list;

        Ok(())
    }
}
