use super::FileNode;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AlphaResponse {
    pub kernel_wim: Option<FileNode>,
    pub cover: Option<AlphaCover>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AlphaCover {
    pub lower_than: String,
    pub file: FileNode,
}
