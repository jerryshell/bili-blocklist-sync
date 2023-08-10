use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub code: Option<i64>,
    pub message: Option<String>,
    pub ttl: Option<i64>,
    pub data: Option<Data>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    pub list: Option<Vec<List>>,
    pub re_version: Option<i64>,
    pub total: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct List {
    pub mid: Option<i64>,
    pub attribute: Option<i64>,
    pub mtime: Option<i64>,
    pub tag: Option<serde_json::Value>,
    pub special: Option<i64>,
    pub uname: Option<String>,
    pub face: Option<String>,
    pub sign: Option<String>,
    pub face_nft: Option<i64>,
    pub official_verify: Option<OfficialVerify>,
    pub vip: Option<Vip>,
    pub nft_icon: Option<String>,
    pub rec_reason: Option<String>,
    pub track_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OfficialVerify {
    #[serde(rename = "type")]
    pub official_verify_type: Option<i64>,
    pub desc: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vip {
    pub vip_type: Option<i64>,
    pub vip_due_date: Option<i64>,
    pub due_remark: Option<String>,
    pub access_status: Option<i64>,
    pub vip_status: Option<i64>,
    pub vip_status_warn: Option<String>,
    pub theme_type: Option<i64>,
    pub label: Option<Label>,
    #[serde(rename = "avatar_subscript")]
    pub avatar_subscript: Option<i64>,
    #[serde(rename = "nickname_color")]
    pub nickname_color: Option<String>,
    #[serde(rename = "avatar_subscript_url")]
    pub avatar_subscript_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Label {
    pub path: Option<String>,
    pub text: Option<String>,
    pub label_theme: Option<String>,
    pub text_color: Option<String>,
    pub bg_style: Option<i64>,
    pub bg_color: Option<String>,
    pub border_color: Option<String>,
}
