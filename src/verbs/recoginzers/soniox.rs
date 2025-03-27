use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SonioxOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<SonioxModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profanity_filter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<SonioxStorage>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SonioxStorage {
    pub id: String,
    pub title: String,
    pub disable_store_audio: bool,
    pub disable_store_transcript: bool,
    pub disable_search: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SonioxModel {
    PrecisionIvr,
}