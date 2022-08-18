use crate::class::{MirrorProperty, ServiceNodeConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub mirror: Mirror,
    pub property: MirrorProperty,
    pub config: ExtendedConfig,
    pub token: Token,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Mirror {
    pub name: String,
    pub description: String,
    pub root: String,
    pub services: Vec<ServiceNodeConfig>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExtendedConfig {
    pub hub: String,
    pub notice: String,
    pub alpha_cover: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Token {
    pub alpha: String,
    pub super_user: String,
}
