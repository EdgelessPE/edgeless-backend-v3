use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MirrorProperty {
    pub domestic_server: bool,
    pub upload_bandwidth: u64,
    pub sync_interval: u64,
    pub official_maintained: bool,
}
