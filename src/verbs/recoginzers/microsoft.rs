use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MSOutputFormat {
    Simple,
    Detailed,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MSProfanityOption {
    Masked,
    Removed,
    Raw,
}
