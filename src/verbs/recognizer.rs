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
