use crate::verbs::recognizer::Recognizer;
use crate::verbs::synthesizer::Synthesizer;
use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};

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

impl Rasa {
    pub fn new(url: String) -> Rasa {
        Rasa {
            url,
            prompt: None,
            event_hook: None,
            action_hook: None,
            tts: None,
            recognizer: None,
        }
    }

    pub fn prompt(&mut self, prompt: Option<String>) -> &mut Rasa {
        self.prompt = prompt;
        self
    }

    pub fn event_hook(&mut self, event_hook: Option<String>) -> &mut Rasa {
        self.event_hook = event_hook;
        self
    }

    pub fn action_hook(&mut self, action_hook: Option<String>) -> &mut Rasa {
        self.action_hook = action_hook;
        self
    }

    pub fn tts(&mut self, tts: Option<Synthesizer>) -> &mut Rasa {
        self.tts = tts;
        self
    }

    pub fn recognizer(&mut self, recognizer: Option<Recognizer>) -> &mut Rasa {
        self.recognizer = recognizer;
        self
    }
    
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
