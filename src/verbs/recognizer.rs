
use serde::{Deserialize, Serialize};
use crate::recoginzers::amazon::AWSFilterMethod;
use crate::recoginzers::azure::AzureOptions;
use crate::recoginzers::deepgram::DeepgramOptions;
use crate::recoginzers::ibm::IBMOptions;
use crate::recoginzers::microsoft::{MSOutputFormat, MSProfanityOption};
use crate::recoginzers::nuance::NuanceOptions;
use crate::recoginzers::nvidia::NvidiaOptions;
use crate::recoginzers::soniox::SonioxOptions;
use crate::verbs::recoginzers::google::{GoogleInteractionType, GoogleSpeechModel};
use crate::verbs::recoginzers::vad::Vad;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Recognizer {
    pub vad: Vad,

    pub alt_languages: Vec<String>,

    pub transcription_hook: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interim: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints_boost: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profanity_filter: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_utterance: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub separate_recognition_per_channel: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub punctuation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<GoogleSpeechModel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_model: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diarization: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diarization_min_speakers: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diarization_max_speakers: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_type: Option<GoogleInteractionType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub naics_code: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_method: Option<AWSFilterMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identify_channels: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub profanity_option: Option<MSProfanityOption>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<MSOutputFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_snr: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_speech_timeout_ms: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asr_timeout: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asr_dtmf_termination_digit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_service_endpoint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_options: Option<AzureOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deepgram_options: Option<DeepgramOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ibm_options: Option<IBMOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nuance_options: Option<NuanceOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nvidia_options: Option<NvidiaOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub soniox_options: Option<SonioxOptions>,
}
