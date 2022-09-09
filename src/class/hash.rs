// use serde::{Deserialize, Serialize};
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct Integrity {
//     pub method: IntegrityMethod,
//     pub value: String,
// }
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub enum IntegrityMethod {
//     SHA256,
//     BLAKE3,
// }

pub use crate::hash2::{Integrity, IntegrityMethod};