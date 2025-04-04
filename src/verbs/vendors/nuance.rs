use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::verbs::vendors::vad::Vad;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NuanceRecognizer {

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
    pub nuance_options: Option<NuanceOptions>,
}


#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NuanceOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub krypton_endpoint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub utterance_detection_mode: Option<NuanceUtteranceDetectionMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub punctuation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_tokenization: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discard_speaker_adaptation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_call_recording: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_load_failures: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_initial_capitalization: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_zero_base_lm_weight: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_wakeup_word: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type: Option<NuanceResultType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_input_timeout_ms: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognition_timeout_ms: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub utterance_end_silence_ms: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_hypotheses: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_domain: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_detection_sensitivity: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_data: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatting: Option<NuanceFormatting>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resource: Vec<NuanceResource>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NuanceResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_wordset: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub builtin: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_grammar: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wakeup_word: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_name: Option<NuanceWeightName>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_value: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reuse: Option<NuanceReusePolicy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<NuanceExternalReference>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NuanceExternalReference {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_type: Option<NuanceReferenceType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_load_failures: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_timeout_ms: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NuanceFormatting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NuanceReferenceType {
    UndefinedResourceType,
    Wordset,
    CompiledWordset,
    DomainLm,
    SpeakerProfile,
    Grammer,
    Settings,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NuanceReusePolicy {
    UndefinedReuse,
    LowReuse,
    HighReuse,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NuanceWeightName {
    DefaultWeight,
    Lowest,
    Low,
    Medium,
    High,
    Highest,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NuanceResultType {
    Final,
    Partial,
    ImmutablePartial,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NuanceUtteranceDetectionMode {
    Single,
    Multiple,
    Disabled,
}
