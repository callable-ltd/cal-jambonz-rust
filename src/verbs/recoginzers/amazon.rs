use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AWSFilterMethod {
    Remove,
    Mask,
    Tag,
}