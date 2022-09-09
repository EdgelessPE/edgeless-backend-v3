use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::Integrity;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PluginsResponse {
    pub tree: HashMap<String, Vec<EptFileNode>>,
    pub path: String,
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EptFileNode {
    pub name: String,
    pub size: u64,
    pub timestamp: u64,
    pub integrity: Integrity
}