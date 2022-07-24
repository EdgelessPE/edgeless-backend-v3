use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct EptFileNode {
    pub name: String,
    pub size: u64,
    pub timestamp: u64,
    pub hash: String, //SHA256
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HubLatest {
    pub version: String,
    pub page: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HubUpdate {
    pub allow_normal_since: String,
    pub force_update_until: String,
    pub wide_gaps: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HubNotice {
    pub id: String,
    pub level: String,
    pub message: String,
    pub description: String,
    pub close_text: String,
    pub lower_than: String,
    pub repeat_after: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HubPackages {
    pub update: String,
    pub extended_update: String,
    pub full: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileNode {
    pub version: String,
    pub file_name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EdgelessVentoy {
    pub version: String,
    pub file_name: String,
    pub url: String,
    pub plugin_url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AlphaCover {
    pub lower_than: String,
    pub url: String,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct EptResponse {
    pub name: String,
    pub description: String,
    pub native_server: bool,
    pub upload_bandwidth: u64,
    pub protocol: String,
    pub root: String,
    pub sync_interval: u64,
    pub official_maintained: bool,
    pub services: Vec<String>,

    pub tree: HashMap<String, Vec<EptFileNode>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HubResponse {
    pub latest: HubLatest,
    pub update: HubUpdate,
    pub notice: HubNotice,
    pub packages: HubPackages,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EdgelessResponse {
    pub iso: FileNode,
    pub ventoy: EdgelessVentoy,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AlphaResponse {
    pub wim: FileNode,
    pub cover: AlphaCover,
}