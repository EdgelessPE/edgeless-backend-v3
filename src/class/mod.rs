mod alpha;
mod common;
mod external;
mod hub;
mod internal;
mod mirror;
mod plugins;
mod ventoy;

use serde::{Deserialize, Serialize};

pub use alpha::{AlphaResponse,AlphaCover};
pub use common::{FileNode,ServiceNodeConfig,ServiceNodePublic};
pub use external::{AlphaCoverJson,HubExtendedJson};
pub use hub::{HubLatest,HubNotice,HubPackages,HubResponse,HubUpdate};
pub use internal::{TokenRequiredQueryStruct,LazyDeleteNode,FileType};
pub use mirror::MirrorProperty;
pub use plugins::{PluginsResponse,EptFileNode};
pub use ventoy::EdgelessVentoy;

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
    pub ventoy: EdgelessVentoy,
    pub hub: HubResponse,
}