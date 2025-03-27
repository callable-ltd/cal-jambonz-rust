use serde::{Deserialize, Serialize};
use crate::recognizer::Recognizer;
use crate::verb::Verb;

#[derive(Serialize, Deserialize, Clone)]
pub struct Transcribe {
    #[serde(rename = "transcriptionHook")]
    pub transcription_hook: String,
    pub recognizer: Recognizer,
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
