use crate::recoginzers::deepgram::DeepgramRecognizer;
use crate::recoginzers::google::GoogleRecognizer;
use crate::recoginzers::ibm::IbmRecognizer;
use crate::recoginzers::microsoft::MSRecognizer;
use crate::recoginzers::nuance::NuanceRecognizer;
use crate::recoginzers::nvidia::NvidiaRecognizer;
use crate::recoginzers::soniox::SonioxRecognizer;
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
