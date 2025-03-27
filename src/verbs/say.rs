use serde::{Deserialize, Serialize};
use crate::verbs::synthesizer::Synthesizer;
use crate::verbs::verb::Verb;

#[derive(Serialize, Deserialize, Clone)]
pub struct Say {
    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthesizer: Option<Synthesizer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "loop")]
    pub say_loop: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "earlyMedia")]
    pub early_media: Option<bool>,
}

impl Into<Verb> for Say {
    fn into(self) -> Verb {
        Verb::Say(self)
    }
}

impl Into<Vec<Verb>> for Say {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}

impl Say {
    pub fn new(text: &str) -> Say {
        Say {
            text: text.to_string(),
            say_loop: Some(1),
            synthesizer: None,
            early_media: Some(false),
        }
    }
}

