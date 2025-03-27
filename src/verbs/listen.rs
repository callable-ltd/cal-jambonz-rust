use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::verbs::auth::WSAuth;
use crate::verbs::transcribe::Transcribe;
use crate::verbs::verb::Verb;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "verb")]
pub enum Listen {
    Listen(ListenStruct)
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListenStruct {
    pub verb: String,

    pub url: String,

    pub action_hook: String,

    pub sample_rate: SampleRate,

    pub timeout: u8,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_on_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mix_type: Option<MixType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_dtmf: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_beep: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcribe: Option<Transcribe>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ws_auth: Option<WSAuth>,
}

impl Into<Verb> for Listen {
    fn into(self) -> Verb {
        Verb::Listen(self)
    }
}

impl Into<Vec<Verb>> for Listen {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SampleRate {
    SR8000 = 8000,
    SR16000 = 16000,
    SR24000 = 24000,
    SR48000 = 48000,
    SR64000 = 64000,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MixType {
    Mono,
    Stereo,
    Mixed,
}
