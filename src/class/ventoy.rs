use serde::{Deserialize, Serialize};
use super::{FileNode};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EdgelessVentoy {
    pub windows: FileNode,
    pub plugin: FileNode,
}
