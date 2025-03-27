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
    pub fn new(text: &str) -> Say {
        Say {
            text: text.to_string(),
            say_loop: Some(1),
            synthesizer: None,
            early_media: Some(false),
        }
    }
    
    pub fn synthesize(&mut self, synthesizer: Synthesizer) -> &mut Say {
        self.synthesizer = Some(synthesizer);
        self
    }
    
    pub fn early_media(&mut self, early_media: bool) -> &mut Say {
        self.early_media = Some(early_media);
        self
    }
    
    pub fn say_loop(&mut self, say_loop: u8) -> &mut Say {
        self.say_loop = Some(say_loop);
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


