use serde::{Deserialize, Serialize};
use super::{FileNode};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VentoyResponse {
    pub windows: FileNode,
    pub linux:FileNode,
    pub plugin: FileNode,
}
