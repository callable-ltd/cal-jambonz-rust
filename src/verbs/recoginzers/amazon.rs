use serde::{Deserialize, Serialize};
use crate::recoginzers::vad::Vad;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AWSRecognizer {

    pub transcription_hook: String,

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
    pub identify_channels: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_method: Option<AWSFilterMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AWSFilterMethod {
    Remove,
    Mask,
    Tag,
}