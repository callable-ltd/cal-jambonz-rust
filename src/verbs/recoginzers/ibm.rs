use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IBMOptions {
    pub stt_api_key: String,
    pub stt_region: String,
    pub instance_id: String,
    pub model: String,
    pub language_customization_id: String,
    pub acoustic_customization_id: String,
    pub base_model_version: String,
    pub watson_metadata: String,
    pub watson_learning_opt_out: bool,
}