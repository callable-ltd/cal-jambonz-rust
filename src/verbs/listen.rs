use crate::verbs::auth::WSAuth;
use crate::verbs::transcribe::Transcribe;
use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Listen {
    pub url: String,

    pub action_hook: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<SampleRate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u8>,

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

impl Listen {
    pub fn new(url: String, action_hook: String) -> Listen {
        Listen {
            url: url.to_string(),
            action_hook: action_hook.to_string(),
            sample_rate: Some(SampleRate::SR8000),
            timeout: None,
            finish_on_key: None,
            max_length: None,
            metadata: None,
            mix_type: None,
            pass_dtmf: None,
            play_beep: None,
            transcribe: None,
            ws_auth: None,
        }
    }
    
    pub fn timeout(&mut self, timeout: Option<u8>) -> &mut Listen {
        self.timeout = timeout;
        self
    }
    
    pub fn finish_on_key(&mut self, key: Option<String>) -> &mut Listen {
        self.finish_on_key = key;
        self
    }
    
    pub fn max_length(&mut self, length: Option<u16>) -> &mut Listen {
        self.max_length = length;
        self
    }
    
    pub fn metadata(&mut self, metadata: Option<HashMap<String, String>>) -> &mut Listen {
        self.metadata = metadata;
        self
    }
    
    pub fn mix_type(&mut self, mix_type: Option<MixType>) -> &mut Listen {
        self.mix_type = mix_type;
        self
    }
    
    pub fn pass_dtmf(&mut self, pass_dtmf: Option<bool>) -> &mut Listen {
        self.pass_dtmf = pass_dtmf;
        self
    }
    
    pub fn play_beep(&mut self, play_beep: Option<bool>) -> &mut Listen {
        self.play_beep = play_beep;
        self
    }
    
    pub fn transcribe(&mut self, transcribe: Option<Transcribe>) -> &mut Listen {
        self.transcribe = transcribe;
        self
    }
    
    pub fn ws_auth(&mut self, ws_auth: Option<WSAuth>) -> &mut Listen {
        self.ws_auth = ws_auth;
        self
    }
    
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

#[derive(Serialize_repr, Deserialize_repr, Clone)]
#[repr(u16)]
pub enum SampleRate {
    SR8000 = 8000,
    SR16000 = 16000,
    SR24000 = 24000,
    SR48000 = 48000,
    SR64000 = 64000,
}

// SampleRate Debug Implementation
impl Debug for SampleRate {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SampleRate::SR8000 => write!(f, "8000Hz"),
            SampleRate::SR16000 => write!(f, "16000Hz"),
            SampleRate::SR24000 => write!(f, "24000Hz"),
            SampleRate::SR48000 => write!(f, "48000Hz"),
            &SampleRate::SR64000 => write!(f, "64000Hz"),

        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MixType {
    Mono,
    Stereo,
    Mixed,
}


// MixType Debug Implementation
impl Debug for MixType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            MixType::Mono => write!(f, "Mono"),
            MixType::Stereo => write!(f, "Stereo"),
            MixType::Mixed => write!(f, "Mixed"),
        }
    }
}
