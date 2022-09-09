use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LazyDeleteNode {
    pub path: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenRequiredQueryStruct {
    pub token: String,
}


#[derive(PartialEq)]
pub enum FileType {
    Dir,
    File,
}
