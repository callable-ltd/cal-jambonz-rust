use crate::verbs::recoginzers::deepgram::DeepgramRecognizer;
use crate::verbs::recoginzers::google::GoogleRecognizer;
use crate::verbs::recoginzers::ibm::IbmRecognizer;
use crate::verbs::recoginzers::microsoft::MSRecognizer;
use crate::verbs::recoginzers::nuance::NuanceRecognizer;
use crate::verbs::recoginzers::nvidia::NvidiaRecognizer;
use crate::verbs::recoginzers::soniox::SonioxRecognizer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "vendor")]
pub enum Recognizer {
    Deepgram(DeepgramRecognizer),
    Google(GoogleRecognizer),
    Ibm(IbmRecognizer),
    Microsoft(MSRecognizer),
    Nuance(NuanceRecognizer),
    Nvidia(NvidiaRecognizer),
    Soniox(SonioxRecognizer),
}
