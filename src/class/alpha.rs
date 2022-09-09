use serde::{Deserialize, Serialize};
use super::{FileNode};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AlphaResponse {
    pub wim: FileNode,
    pub cover: AlphaCover,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AlphaCover {
    pub lower_than: String,
    pub url: String,
}