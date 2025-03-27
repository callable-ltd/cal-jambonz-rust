use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NvidiaOptions {
    pub riva_uri: String,
    pub max_alternatives: u8,
    pub profanity_filter: bool,
    pub punctuation: bool,
    pub word_time_offsets: bool,
    pub verbatim_transcripts: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_configuration: Option<HashMap<String, String>>,
}