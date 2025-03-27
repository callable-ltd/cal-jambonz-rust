use crate::verbs::synthesizer::Synthesizer;
use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DialogFlow {
    pub project: String,

    pub lang: String,

    pub credentials: String,

    pub welcome_event: Option<String>,

    pub welcome_event_params: Option<HashMap<String, String>>,

    pub no_input_timeout: Option<u8>,

    pub no_input_event: Option<String>,

    pub pass_dtmf_as_text_input: Option<String>,

    pub thinking_music: Option<String>,

    pub action_hook: Option<String>,

    pub event_hook: Option<String>,

    pub tts: Synthesizer,

    pub baregin: Option<bool>,
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
