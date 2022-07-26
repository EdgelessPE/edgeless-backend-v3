use std::{
    sync::mpsc::{Receiver, Sender},
    time::SystemTime,
};

use crate::{
    class::{AlphaResponse, HelloResponse},
    constant::{CHECK_UPDATE_INTERVAL, SU_REQUEST, CMD_REQUEST},
};

pub struct Bridge {
    result_receiver: Receiver<(HelloResponse, AlphaResponse)>,
    commander: Sender<String>,

    hello_response: Option<HelloResponse>,
    alpha_response: Option<AlphaResponse>,
    recent_update: SystemTime,
}

impl Bridge {
    pub fn new(
        result_receiver: Receiver<(HelloResponse, AlphaResponse)>,
        commander: Sender<String>,
    ) -> Self {
        Bridge {
            result_receiver,
            commander,
            hello_response: None,
            alpha_response: None,
            recent_update: SystemTime::UNIX_EPOCH,
        }
    }

    pub fn update_cache(&mut self, block: bool,force:bool) {
        let cmd=if force {
            String::from(SU_REQUEST)
        }else{
            String::from(CMD_REQUEST)
        };
        self.commander.send(cmd).unwrap();
        //尝试获取通道中的内容
        loop {
            let try_receive = self.result_receiver.try_recv();
            if let Ok((hello_res, alpha_res)) = try_receive {
                self.hello_response = Some(hello_res);
                self.alpha_response = Some(alpha_res);
                println!("Info:Cache updated");
                if block {
                    break;
                }
            } else {
                if !block {
                    break;
                }
            }
        }
        self.recent_update = SystemTime::now();
    }

    fn is_expired(&self) -> bool {
        SystemTime::now()
            .duration_since(self.recent_update)
            .unwrap()
            .as_secs()
            > CHECK_UPDATE_INTERVAL
    }

    pub fn hello(&mut self) -> anyhow::Result<HelloResponse> {
        let is_none = self.hello_response.is_none();
        if is_none || self.is_expired() {
            self.update_cache(is_none,false);
        }
        Ok(self.hello_response.as_ref().unwrap().clone())
    }

    pub fn alpha(&mut self) -> anyhow::Result<AlphaResponse> {
        let is_none = self.alpha_response.is_none();
        if is_none || self.is_expired() {
            self.update_cache(is_none,false);
        }
        Ok(self.alpha_response.as_ref().unwrap().clone())
    }
}
