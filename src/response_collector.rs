use crate::class::{
    AlphaCover, AlphaResponse, EptFileNode, FileNode, HelloResponse, PluginsResponse,
    ServiceNodePublic,
};
use crate::config::Config;
use crate::utils::{file_selector, get_json, get_service, version_extractor};
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

        //获取插件信息
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
        //创建响应结构体
        let plugins_service = get_service(&c.mirror.services, String::from("plugins")).unwrap();
        let plugins_response = PluginsResponse {
            tree: self.packages_tree.clone(),
            path: plugins_service.path,
        };

        //筛选 iso
        let iso_service = get_service(&c.mirror.services, String::from("iso")).unwrap();
        let selected_iso =
            file_selector(iso_service.local, String::from("^Edgeless.*iso$"), 2).unwrap();
        let iso_version = version_extractor(selected_iso.clone(), 2).unwrap();

        //筛选 alpha
        let alpha_service = get_service(&c.mirror.services, String::from("alpha")).unwrap();
        let selected_alpha_wim =
            file_selector(alpha_service.local, String::from("^Edgeless.*wim$"), 2).unwrap();
        let alpha_version = version_extractor(selected_alpha_wim.clone(), 2).unwrap();
        let extended_alpha_config = get_json(c.config.alpha_cover.clone()).unwrap();
        let alpha_cover = AlphaCover {
            lower_than: extended_alpha_config["lower_than"].to_string(),
            url: c
                .mirror
                .root
                .clone()
                .add(&alpha_service.path)
                .add("cover.7z"),
        };
        let alpha_wim = FileNode {
            version: alpha_version,
            file_name: selected_alpha_wim.clone(),
            url: c
                .mirror
                .root
                .clone()
                .add(&alpha_service.path)
                .add(&selected_alpha_wim),
        };

        //筛选 ventoy
        let ventoy_service = get_service(&c.mirror.services, String::from("ventoy")).unwrap();
        let selected_ventoy = file_selector(
            ventoy_service.local,
            String::from("^ventoy-.*-windows.zip$"),
            1,
        )
        .unwrap();
        let ventoy_version = version_extractor(selected_ventoy.clone(), 1).unwrap();

        Ok(HelloResponse {
            name: c.mirror.name,
            description: c.mirror.description,
            protocol: String::from(PROTOCOL),
            root: c.mirror.root.clone(),
            property: c.property,
            services: pub_services,
            plugins: plugins_response,
            iso: FileNode {
                version: iso_version,
                file_name: selected_iso.clone(),
                url: c
                    .mirror
                    .root
                    .clone()
                    .add(&iso_service.path)
                    .add(&selected_iso),
            },
            alpha: AlphaResponse {
                wim: alpha_wim,
                cover: alpha_cover,
            },
            ventoy: FileNode {
                version: ventoy_version,
                file_name: selected_ventoy.clone(),
                url: c
                    .mirror
                    .root
                    .clone()
                    .add(&ventoy_service.path)
                    .add(&selected_ventoy),
            },
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
