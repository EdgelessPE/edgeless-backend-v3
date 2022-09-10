mod alpha;
mod common;
mod external;
mod hash;
mod hub;
mod internal;
mod mirror;
mod plugins;
mod ventoy;

use serde::{Deserialize, Serialize};

pub use alpha::{AlphaResponse,AlphaCover};
pub use common::{FileNode,ServiceNodeConfig,ServiceNodePublic};
pub use external::{AlphaCoverJson,HubExtendedJson};
pub use hash::{Integrity,IntegrityMethod};
pub use hub::{HubLatest,HubNotice,HubPackages,HubResponse,HubUpdate};
pub use internal::{TokenRequiredQueryStruct,LazyDeleteNode,FileType};
pub use mirror::MirrorProperty;
pub use plugins::{PluginsResponse,EptFileNode};
pub use ventoy::VentoyResponse;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HelloResponse {
    pub name: String,
    pub description: String,
    pub protocol: String,
    pub root: String,
    pub property: MirrorProperty,
    pub services: Vec<ServiceNodePublic>,

    pub plugins: PluginsResponse,
    pub kernel: FileNode,
    pub ventoy: VentoyResponse,
    pub hub: HubResponse,
}