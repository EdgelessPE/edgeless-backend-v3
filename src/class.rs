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
struct HubLatest {
    version: String,
    page: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct HubUpdate {
    allow_normal_since: String,
    force_update_until: String,
    wide_gaps: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct HubNotice {
    id: String,
    level: String,
    message: String,
    description: String,
    close_text: String,
    lower_than: String,
    repeat_after: u8,
}

#[derive(Serialize, Deserialize, Clone)]
struct HubPackages {
    update: String,
    extended_update: String,
    full: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct FileNode {
    version: String,
    file_name: String,
    url: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct EdgelessVentoy {
    version: String,
    file_name: String,
    url: String,
    plugin_url: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct AlphaCover {
    lower_than: String,
    url: String,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct EptResponse {
    name: String,
    protocol: String,
    root: String,
    sync_interval: i8,
    tree: HashMap<String, EptFileNode>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HubResponse {
    latest: HubLatest,
    update: HubUpdate,
    notice: HubNotice,
    packages: HubPackages,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EdgelessResponse {
    iso: FileNode,
    ventoy: EdgelessVentoy,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AlphaResponse {
    wim: FileNode,
    cover: AlphaCover,
}