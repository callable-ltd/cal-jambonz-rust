use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Synthesizer {
    //todo change to enum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<String>,

    //todo change to enum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    //todo change to enum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    //todo change to enum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
}
