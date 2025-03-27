use crate::verbs::play::Play;
use crate::verbs::recognizer::Recognizer;
use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};
use crate::verbs::say::Say;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Gather {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_hook: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bargein: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtmf_bargein: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_on_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inter_digit_timeout: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_during_prompt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_bargein_word_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_digits: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_digits: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_digits: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_result_hook: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play: Option<Play>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub say: Option<Say>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognizer: Option<Recognizer>,
}

impl Into<Verb> for Gather {
    fn into(self) -> Verb {
        Verb::Gather(self)
    }
}

impl Into<Vec<Verb>> for Gather {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}
