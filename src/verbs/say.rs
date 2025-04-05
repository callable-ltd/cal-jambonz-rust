use crate::verbs::synthesizer::Synthesizer;
use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};

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

impl Say {
    pub fn new(text: String) -> Say {
        Say {
            text,
            say_loop: Some(1),
            synthesizer: None,
            early_media: Some(false),
        }
    }
    
    pub fn synthesize(&mut self, synthesizer: Option<Synthesizer>) -> &mut Say {
        self.synthesizer = synthesizer;
        self
    }
    
    pub fn early_media(&mut self, early_media: Option<bool>) -> &mut Say {
        self.early_media = early_media;
        self
    }
    
    pub fn say_loop(&mut self, say_loop: Option<u8>) -> &mut Say {
        self.say_loop = say_loop;
        self
    }
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


