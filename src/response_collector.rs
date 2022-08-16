use crate::class::{EptFileNode, EptResponse};
use crate::config::Config;
use std::collections::HashMap;
use std::io;
use std::ops::Add;
use std::sync::mpsc::Receiver;

const PROTOCOL: &str = "3.0.0";

pub struct ResponseCollector {
    packages_receiver: Receiver<HashMap<String, Vec<EptFileNode>>>,
    packages_tree: HashMap<String, Vec<EptFileNode>>,
    config: Config,
}

impl ResponseCollector {
    pub fn new(
        packages_receiver: Receiver<HashMap<String, Vec<EptFileNode>>>,
        config: Config,
    ) -> Self {
        ResponseCollector {
            packages_receiver,
            packages_tree: HashMap::new(),
            config,
        }
    }

    pub fn ept(&mut self) -> io::Result<EptResponse> {
        let c = self.config.to_owned();

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
