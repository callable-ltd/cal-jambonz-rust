use crate::verbs::recoginzers::vad::Vad;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MSRecognizer {

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
    pub azure_service_endpoint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_options: Option<AzureOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_speech_timeout_ms: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profanity_filter: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profanity_option: Option<MSProfanityOption>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<MSOutputFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_snr: Option<bool>,
}


#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MSOutputFormat {
    Simple,
    Detailed,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MSProfanityOption {
    Masked,
    Removed,
    Raw,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AzureOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_segmentation_silence_timeout_ms: Option<u16>,
}