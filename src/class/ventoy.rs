use super::FileNode;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VentoyResponse {
    pub windows: FileNode,
    pub linux: FileNode,
    pub plugin: FileNode,
}
