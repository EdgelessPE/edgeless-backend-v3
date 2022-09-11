use super::hash::Integrity;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileNode {
    pub name: String,
    pub version: String,
    pub url: String,
    pub size: u64,
    pub timestamp: u64,
    pub integrity: Integrity,
}
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
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
