use crate::class::{EptFileNode, EptResponse};
use crate::config::Config;
use std::collections::HashMap;
use std::io;
use std::ops::Add;
use std::sync::mpsc::{Receiver, Sender};

use crate::constant::{PROTOCOL,CMD_REQUEST};

pub struct ResponseCollector {
    packages_receiver: Receiver<HashMap<String, Vec<EptFileNode>>>,
    commander: Sender<String>,
    packages_tree: HashMap<String, Vec<EptFileNode>>,
    config: Config,
}

impl ResponseCollector {
    pub fn new(
        packages_receiver: Receiver<HashMap<String, Vec<EptFileNode>>>,
        commander: Sender<String>,
        config: Config,
    ) -> Self {
        ResponseCollector {
            packages_receiver,
            commander,
            packages_tree: HashMap::new(),
            config,
        }
    }

    pub fn ept(&mut self) -> io::Result<EptResponse> {
        let c = self.config.to_owned();

        //发送更新请求
        self.commander.send(String::from(CMD_REQUEST)).unwrap();

        //尝试获取通道中的内容
        loop {
            let try_receive = self.packages_receiver.try_recv();
            if let Ok(val) = try_receive {
                self.packages_tree = val;
            } else {
                break;
            }
        }

        Ok(EptResponse {
            name: c.mirror.name,
            description: c.mirror.description,
            native_server: c.mirror.native_server,
            upload_bandwidth: c.mirror.upload_bandwidth,
            protocol: String::from(PROTOCOL),
            root: c.url.domain.add(&c.url.plugins),
            sync_interval: c.mirror.sync_interval,
            official_maintained: c.mirror.official_maintained,
            services: c.mirror.services,
            tree: self.packages_tree.clone(),
        })
    }
}
