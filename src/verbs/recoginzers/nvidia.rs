use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::verbs::recoginzers::vad::Vad;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NvidiaRecognizer {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vad: Option<Vad>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interim: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_languages: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asr_dtmf_termination_digit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asr_timeout: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub separate_recognition_per_channel: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints_boost: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nvidia_options: Option<NvidiaOptions>,
}


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