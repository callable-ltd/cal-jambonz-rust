use serde::{Deserialize, Serialize};
use crate::verbs::recognizer::Recognizer;
use crate::verbs::synthesizer::Synthesizer;
use crate::verbs::verb::Verb;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Rasa {
    pub url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_hook: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_hook: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<Synthesizer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognizer: Option<Recognizer>,
}

impl Into<Verb> for Rasa {
    fn into(self) -> Verb {
        Verb::Rasa(self)
    }
}

impl Into<Vec<Verb>> for Rasa {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}
