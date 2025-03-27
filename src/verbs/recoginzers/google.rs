use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GoogleSpeechModel {
    PhoneCall,
    Telephony,
    TelephonyShort,
    MedicalDictation,
    MedialConversation,
    LatestShort,
    LatestLong,
    CommandAndSearch,
    Default,
    Video,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GoogleInteractionType {
    Discussion,
    Presentation,
    PhoneCall,
    VoiceMail,
    ProfessionallyProduced,
    VoiceSearch,
    VoiceCommand,
    Dictation,
}