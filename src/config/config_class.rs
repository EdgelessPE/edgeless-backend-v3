use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    mirror: Mirror,
    position: Position,
    url: Url,
    config: SubConfig,
}

#[derive(Serialize, Deserialize, Clone)]
struct Mirror {
    name: String,
    description: String,
    native_server: bool,
    upload_bandwidth: u64,
    sync_interval: u64,
    official_maintained: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct Position {
    plugins: String,
    iso: String,
    alpha: String,
    alpha_cover: String,
    ventoy: String,
    hub: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Url {
    domain: String,

    ventoy_plugin: String,

    plugins: String,
    iso: String,
    alpha: String,
    alpha_cover: String,
    ventoy: String,
    hub: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct SubConfig {
    hub: String,
    notice: String,
}