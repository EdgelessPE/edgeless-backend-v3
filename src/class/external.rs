use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AlphaCoverJson {
    pub lower_than: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HubExtendedJson {
    pub download_page: String,
    pub allow_normal_since: String,
    pub force_update_until: String,
    pub wide_gaps: Vec<String>,
}
