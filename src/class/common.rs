use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileNode {
    pub name: String,
    pub version: String,
    pub size: u64,
    pub timestamp: u64,
    pub hash: String, //SHA256
    pub url: String,
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