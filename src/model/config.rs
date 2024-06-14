use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cookie {
    pub sessdata: String,
    pub bili_jct: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub sleep_ms: u64,
    pub cookie_list: Vec<Cookie>,
}
