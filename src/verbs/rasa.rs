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
    pub fn new(url: &str) -> Rasa {
        Rasa {
            url: url.to_string(),
            prompt: None,
            event_hook: None,
            action_hook: None,
            tts: None,
            recognizer: None,
        }
    }

    pub fn prompt(&mut self, prompt: &str) -> &mut Rasa {
        self.prompt = Some(prompt.to_string());
        self
    }

    pub fn event_hook(&mut self, event_hook: &str) -> &mut Rasa {
        self.event_hook = Some(event_hook.to_string());
        self
    }

    pub fn action_hook(&mut self, action_hook: &str) -> &mut Rasa {
        self.action_hook = Some(action_hook.to_string());
        self
    }

    pub fn tts(&mut self, tts: Synthesizer) -> &mut Rasa {
        self.tts = Some(tts);
        self
    }

    pub fn recognizer(&mut self, recognizer: Recognizer) -> &mut Rasa {
        self.recognizer = Some(recognizer);
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
