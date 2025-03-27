use crate::verbs::synthesizer::Synthesizer;
use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DialogFlow {
    pub credentials: String,

    pub lang: String,

    pub project: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_hook: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub baregin: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_hook: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_input_event: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_input_timeout: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_dtmf_as_text_input: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_music: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<DialogFlowSynthesizer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub welcome_event: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub welcome_event_params: Option<HashMap<String, String>>,
}

impl Into<Verb> for DialogFlow {
    fn into(self) -> Verb {
        Verb::DialogFlow(self)
    }
}

impl Into<Vec<Verb>> for DialogFlow {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}

impl DialogFlow {
    pub fn new(project: &str, lang: &str, credentials: &str, tts: Synthesizer) -> DialogFlow {
        DialogFlow {
            credentials: credentials.to_string(),
            lang: lang.to_string(),
            project: project.to_string(),
            action_hook: None,
            baregin: None,
            event_hook: None,
            no_input_event: None,
            no_input_timeout: None,
            pass_dtmf_as_text_input: None,
            thinking_music: None,
            tts: None,
            welcome_event: None,
            welcome_event_params: None,
        }
    }
    
    pub fn action_hook(&mut self, action_hook: &str) -> &mut DialogFlow {
        self.action_hook = Some(action_hook.to_string());
        self
    }
    
    pub fn baregin(&mut self, baregin: bool) -> &mut DialogFlow {
        self.baregin = Some(baregin);
        self
    }
    
    pub fn event_hook(&mut self, event_hook: &str) -> &mut DialogFlow {
        self.event_hook = Some(event_hook.to_string());
        self
    }
    
    pub fn no_input_event(&mut self, no_input_event: &str) -> &mut DialogFlow {
        self.no_input_event = Some(no_input_event.to_string());
        self
    }
    
    pub fn no_input_timeout(&mut self, no_input_timeout: u8) -> &mut DialogFlow {
        self.no_input_timeout = Some(no_input_timeout);
        self
    }
    
    pub fn pass_dtmf(&mut self, pass_dtmf: &str) -> &mut DialogFlow {
        self.pass_dtmf_as_text_input = Some(pass_dtmf.to_string());
        self
    }
    
    pub fn welcome_event(&mut self, welcome_event: &str) -> &mut DialogFlow {
        self.welcome_event = Some(welcome_event.to_string());
        self
    }
    
    pub fn welcome_event_params(&mut self, welcome_event_params: HashMap<String, String>) -> &mut DialogFlow {
        self.welcome_event_params = Some(welcome_event_params);
        self
    }
    
    
    pub fn  thinking_music(&mut self, thinking_music: &str) -> &mut DialogFlow {
        self.thinking_music = Some(thinking_music.to_string());
        self
    }
    
    pub fn tt(&mut self, tts: DialogFlowSynthesizer) -> &mut DialogFlow {
        self.tts = Some(tts);
        self
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DialogFlowSynthesizer {
    pub language: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
}

impl DialogFlowSynthesizer {
    pub fn new(language: &str) -> DialogFlowSynthesizer {
        DialogFlowSynthesizer {
            language: language.to_string(),
            vendor: None,
            gender: None,
            voice: None,
        }
    }
    
    pub fn vendor(&mut self, vendor: &str) -> &mut DialogFlowSynthesizer {
        self.vendor = Some(vendor.to_string());
        self
    }
    
    pub fn gender(&mut self, gender: &str) -> &mut DialogFlowSynthesizer {
        self.gender = Some(gender.to_string());
        self
    }
    
    pub fn voice(&mut self, vendor: &str) -> &mut DialogFlowSynthesizer {
        self.voice = Some(vendor.to_string());
        self
    }
}
