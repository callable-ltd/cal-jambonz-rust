use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Vad {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_ms: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<VadMode>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum VadMode {
    M0 = 0,
    M1 = 1,
    M2 = 2,
    M3 = 3,
}
