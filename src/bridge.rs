use std::{
    sync::mpsc::{Receiver, Sender},
    time::SystemTime,
};

use crate::{
    class::{AlphaResponse, HelloResponse},
    constant::{CHECK_UPDATE_INTERVAL, SU_REQUEST},
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

    fn update_cache(&mut self, block: bool) {
        self.commander.send(String::from(SU_REQUEST)).unwrap();
        //尝试获取通道中的内容
        loop {
            let try_receive = self.result_receiver.try_recv();
            if let Ok((hello_res, alpha_res)) = try_receive {
                self.hello_response = Some(hello_res);
                self.alpha_response = Some(alpha_res);
                println!("Cache updated");
                if block {
                    break;
                }
            } else {
                if !block {
                    break;
                }
            }
        }
    }

    fn is_expired(&self) -> bool {
        SystemTime::now()
            .duration_since(self.recent_update)
            .unwrap()
            .as_secs()
            > CHECK_UPDATE_INTERVAL
    }

    pub fn hello(&self) -> anyhow::Result<HelloResponse> {
        let is_none = self.hello_response.is_none();
        if is_none || self.is_expired() {
            self.update_cache(is_none);
        }
        Ok(self.hello_response.unwrap())
    }

    pub fn alpha(&self) -> anyhow::Result<AlphaResponse> {
        let is_none = self.alpha_response.is_none();
        if is_none || self.is_expired() {
            self.update_cache(is_none);
        }
        Ok(self.alpha_response.unwrap())
    }
}
