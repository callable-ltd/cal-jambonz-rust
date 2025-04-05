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
    pub fn new(project: String, lang: String, credentials: String) -> DialogFlow {
        DialogFlow {
            credentials,
            lang,
            project,
            tts: None,
            action_hook: None,
            baregin: None,
            event_hook: None,
            no_input_event: None,
            no_input_timeout: None,
            pass_dtmf_as_text_input: None,
            thinking_music: None,
            welcome_event: None,
            welcome_event_params: None,
        }
    }

    pub fn action_hook(&mut self, action_hook: Option<String>) -> &mut DialogFlow {
        self.action_hook = action_hook;
        self
    }

    pub fn baregin(&mut self, baregin: Option<bool>) -> &mut DialogFlow {
        self.baregin = baregin;
        self
    }

    pub fn event_hook(&mut self, event_hook: Option<String>) -> &mut DialogFlow {
        self.event_hook = event_hook;
        self
    }

    pub fn no_input_event(&mut self, no_input_event: Option<String>) -> &mut DialogFlow {
        self.no_input_event = no_input_event;
        self
    }

    pub fn no_input_timeout(&mut self, no_input_timeout: Option<u8>) -> &mut DialogFlow {
        self.no_input_timeout = no_input_timeout;
        self
    }

    pub fn pass_dtmf(&mut self, pass_dtmf: Option<String>) -> &mut DialogFlow {
        self.pass_dtmf_as_text_input = pass_dtmf;
        self
    }

    pub fn welcome_event(&mut self, welcome_event: Option<String>) -> &mut DialogFlow {
        self.welcome_event = welcome_event;
        self
    }

    pub fn welcome_event_params(
        &mut self,
        welcome_event_params: Option<HashMap<String, String>>,
    ) -> &mut DialogFlow {
        self.welcome_event_params = welcome_event_params;
        self
    }

    pub fn thinking_music(&mut self, thinking_music: Option<String>) -> &mut DialogFlow {
        self.thinking_music = thinking_music;
        self
    }

    pub fn tt(&mut self, tts: Option<DialogFlowSynthesizer>) -> &mut DialogFlow {
        self.tts = tts;
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
    pub fn new(language: String) -> DialogFlowSynthesizer {
        DialogFlowSynthesizer {
            language,
            vendor: None,
            gender: None,
            voice: None,
        }
    }

    pub fn vendor(&mut self, vendor: Option<String>) -> &mut DialogFlowSynthesizer {
        self.vendor = vendor;
        self
    }

    pub fn gender(&mut self, gender: Option<String>) -> &mut DialogFlowSynthesizer {
        self.gender = gender;
        self
    }

    pub fn voice(&mut self, vendor: Option<String>) -> &mut DialogFlowSynthesizer {
        self.voice = vendor;
        self
    }
}
