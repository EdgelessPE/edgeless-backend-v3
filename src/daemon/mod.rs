use std::fs;
use std::sync::mpsc::Sender;
use std::time::SystemTime;
use std::path::Path;

use crate::class::EptResponse;

struct Daemon {
    timestamp_recent_finish: SystemTime,
    status_running: bool,
    list_lazy_delete: Vec<String>,

    sender: Sender<EptResponse>,
    dir_packages:String
}
impl Daemon {
    pub fn new(sender: Sender<EptResponse>,dir_packages:String) -> Self {
        Daemon {
            timestamp_recent_finish: SystemTime::UNIX_EPOCH,
            status_running: false,
            list_lazy_delete: vec![],
            sender,
            dir_packages,
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
        for file in &self.list_lazy_delete {
            let file_path=Path::new(&self.dir_packages).join(&file);
            if file_path.exists(){
                if let Err(err)= fs::remove_file(&file_path){
                    println!("Fatal:Can't delete {}, io error : {}",file_path.to_string_lossy(),err);
                }
            }else{
                println!("Warning:Can't delete {}, file not exist",file_path.to_string_lossy());
            }
        }

        //生成
        

        Ok(())
    }
}
