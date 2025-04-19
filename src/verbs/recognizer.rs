use std::fmt::{Debug, Formatter};
use crate::vendors::amazon::AWSRecognizer;
use crate::verbs::vendors::deepgram::DeepgramRecognizer;
use crate::verbs::vendors::google::GoogleRecognizer;
use crate::verbs::vendors::ibm::IbmRecognizer;
use crate::verbs::vendors::microsoft::MSRecognizer;
use crate::verbs::vendors::nuance::NuanceRecognizer;
use crate::verbs::vendors::nvidia::NvidiaRecognizer;
use crate::verbs::vendors::soniox::SonioxRecognizer;
use serde::{Deserialize, Serialize};
use crate::vendors::openai::OpenaiRecognizer;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "vendor")]
pub enum Recognizer {
    Aws(AWSRecognizer),
    Deepgram(DeepgramRecognizer),
    Google(GoogleRecognizer),
    Ibm(IbmRecognizer),
    Microsoft(MSRecognizer),
    Nuance(NuanceRecognizer),
    Nvidia(NvidiaRecognizer),
    Soniox(SonioxRecognizer),
    Openai(OpenaiRecognizer)
}

// Recognizer Debug Implementation
impl Debug for Recognizer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Recognizer::Aws(_) => write!(f, "Recognizer::Aws"),
            Recognizer::Deepgram(_) => write!(f, "Recognizer::Deepgram"),
            Recognizer::Google(_) => write!(f, "Recognizer::Google"),
            Recognizer::Ibm(_) => write!(f, "Recognizer::Ibm"),
            Recognizer::Microsoft(_) => write!(f, "Recognizer::Microsoft"),
            Recognizer::Nuance(_) => write!(f, "Recognizer::Nuance"),
            Recognizer::Nvidia(_) => write!(f, "Recognizer::Nvidia"),
            Recognizer::Soniox(_) => write!(f, "Recognizer::Soniox"),
            Recognizer::Openai(_) => write!(f, "Recognizer::Openai"),
        }
    }
}