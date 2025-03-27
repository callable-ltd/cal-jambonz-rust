use serde::{Deserialize, Serialize};
use crate::verbs::verb::Verb;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pause {
    pub length: u8,
}

impl Into<Verb> for Pause {
    fn into(self) -> Verb {
        Verb::Pause(self)
    }
}

impl Into<Vec<Verb>> for Pause {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}
