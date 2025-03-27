use serde::{Deserialize, Serialize};
use crate::recoginzers::vad::Vad;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IbmRecognizer {

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
    pub ibm_options: Option<IBMOptions>,

}


#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IBMOptions {
    pub stt_api_key: String,
    pub stt_region: String,
    pub instance_id: String,
    pub model: String,
    pub language_customization_id: String,
    pub acoustic_customization_id: String,
    pub base_model_version: String,
    pub watson_metadata: String,
    pub watson_learning_opt_out: bool,
}