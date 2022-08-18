use crate::class::{EptFileNode, HelloResponse, ServiceNodePublic, FileNode};
use crate::config::Config;
use crate::utils::{file_selector, get_service, version_extractor};
use std::collections::HashMap;
use std::io;
use std::ops::Add;
use std::sync::mpsc::{Receiver, Sender};

use crate::constant::{CMD_REQUEST, PROTOCOL, SU_REQUEST};

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

    pub fn hello(&mut self) -> io::Result<HelloResponse> {
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

        //过滤 services 中的 local 字段
        let pub_services: Vec<ServiceNodePublic> = c
            .mirror
            .services
            .clone()
            .into_iter()
            .map(|node| ServiceNodePublic {
                name: node.name,
                path: node.path,
            })
            .collect();

        //筛选 iso
        let iso_service=get_service(&c.mirror.services, String::from("iso")).unwrap();
        let selected_iso=file_selector(iso_service.local, String::from("^Edgeless.*iso$"), 2).unwrap();
        let iso_version=version_extractor(selected_iso.clone(), 2).unwrap();

        Ok(HelloResponse {
            name: c.mirror.name,
            description: c.mirror.description,
            protocol: String::from(PROTOCOL),
            root: c.mirror.root.clone(),
            property: c.property,
            services: pub_services,
            plugins: self.packages_tree.clone(),
            iso:FileNode{
                version:iso_version,
                file_name:selected_iso.clone(),
                url:c.mirror.root.clone().add(&selected_iso),
            }
        })
    }

    pub fn ept_refresh(&mut self, super_user: bool) {
        if super_user {
            self.commander.send(String::from(SU_REQUEST)).unwrap();
        } else {
            self.commander.send(String::from(CMD_REQUEST)).unwrap();
        }
    }
}
