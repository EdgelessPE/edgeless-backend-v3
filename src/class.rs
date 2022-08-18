use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EptFileNode {
    pub name: String,
    pub size: u64,
    pub timestamp: u64,
    pub hash: String, //SHA256
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HubLatest {
    pub version: String,
    pub page: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HubUpdate {
    pub allow_normal_since: String,
    pub force_update_until: String,
    pub wide_gaps: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HubNotice {
    pub id: String,
    pub level: String,
    pub message: String,
    pub description: String,
    pub close_text: String,
    pub lower_than: String,
    pub repeat_after: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HubPackages {
    pub update: String,
    pub extended_update: String,
    pub full: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileNode {
    pub version: String,
    pub file_name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EdgelessVentoy {
    pub version: String,
    pub file_name: String,
    pub url: String,
    pub plugin_url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AlphaCover {
    pub lower_than: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServiceNodeConfig {
    pub name: String,
    pub path: String,
    pub local: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ServiceNodePublic {
    pub name: String,
    pub path: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HelloResponse {
    pub name: String,
    pub description: String,
    pub protocol: String,
    pub root: String,
    pub property: MirrorProperty,
    pub services: Vec<ServiceNodePublic>,

    pub plugins: PluginsResponse,
    pub iso: FileNode,
    pub alpha: AlphaResponse,
    pub ventoy: FileNode,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PluginsResponse {
    pub tree: HashMap<String, Vec<EptFileNode>>,
    pub path: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MirrorProperty {
    pub native_server: bool,
    pub upload_bandwidth: u64,
    pub sync_interval: u64,
    pub official_maintained: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HubResponse {
    pub latest: HubLatest,
    pub update: HubUpdate,
    pub notice: HubNotice,
    pub packages: HubPackages,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EdgelessResponse {
    pub iso: FileNode,
    pub ventoy: EdgelessVentoy,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AlphaResponse {
    pub wim: FileNode,
    pub cover: AlphaCover,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LazyDeleteNode {
    pub path: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenRequiredQueryStruct {
    pub token: String,
}
