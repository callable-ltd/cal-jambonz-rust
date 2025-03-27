use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pause {
    pub length: u8,
}

impl Pause {
    pub fn new(length: u8) -> Pause {
        Pause { length }
    }
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
