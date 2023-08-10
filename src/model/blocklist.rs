use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct Item {
    pub mid: String,
    pub uname: String,
    pub face: String,
    pub space_url: String,
}
