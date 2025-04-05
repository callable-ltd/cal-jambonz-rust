use crate::verbs::recognizer::Recognizer;
use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Transcribe {
    #[serde(rename = "transcriptionHook")]
    pub transcription_hook: String,
    pub recognizer: Recognizer,
}

impl Transcribe {
    pub fn new(transcription_hook: String, recognizer: Recognizer) -> Transcribe {
        Transcribe {
            transcription_hook,
            recognizer,
        }
    }
}

impl Into<Verb> for Transcribe {
    fn into(self) -> Verb {
        Verb::Transcribe(self)
    }
}

impl Into<Vec<Verb>> for Transcribe {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}
