use serde::{Deserialize, Serialize};
use crate::verbs::recoginzers::vad::Vad;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GoogleRecognizer {

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
    pub diarization: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diarization_min_speakers: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diarization_max_speakers: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_model: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints_boost: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_type: Option<GoogleInteractionType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<GoogleSpeechModel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub naics_code: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub punctuation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_utterance: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub separate_recognition_per_channel: Option<bool>,

}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GoogleSpeechModel {
    PhoneCall,
    Telephony,
    TelephonyShort,
    MedicalDictation,
    MedialConversation,
    LatestShort,
    LatestLong,
    CommandAndSearch,
    Default,
    Video,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GoogleInteractionType {
    Discussion,
    Presentation,
    PhoneCall,
    VoiceMail,
    ProfessionallyProduced,
    VoiceSearch,
    VoiceCommand,
    Dictation,
}