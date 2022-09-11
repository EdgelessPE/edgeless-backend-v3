use super::FileNode;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HubResponse {
    pub latest: HubLatest,
    pub update: HubUpdate,
    pub notices: Vec<HubNotice>,
    pub packages: HubPackages,
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
    pub channel: String,
    pub level: String,
    pub message: String,
    pub description: String,
    pub close_text: String,
    pub lower_than: String,
    pub repeat_after: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HubPackages {
    pub update: FileNode,
    pub extended_update: FileNode,
    pub full: FileNode,
}
