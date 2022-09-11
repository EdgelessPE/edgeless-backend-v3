use super::FileNode;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AlphaResponse {
    pub kernel_wim: FileNode,
    pub cover: AlphaCover,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AlphaCover {
    pub lower_than: String,
    pub file: FileNode,
}
