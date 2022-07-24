use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub mirror: Mirror,
    pub position: Position,
    pub url: Url,
    pub config: SubConfig,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Mirror {
    pub name: String,
    pub description: String,
    pub native_server: bool,
    pub upload_bandwidth: u64,
    pub sync_interval: u64,
    pub official_maintained: bool,
    pub services: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Position {
    pub plugins: String,
    pub iso: String,
    pub alpha: String,
    pub ventoy: String,
    pub hub: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Url {
    pub domain: String,

    pub ventoy_plugin: String,

    pub plugins: String,
    pub iso: String,
    pub alpha: String,
    pub alpha_cover: String,
    pub ventoy: String,
    pub hub: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SubConfig {
    pub hub: String,
    pub notice: String,
}