use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Leave {}

impl Leave {
    pub fn new() -> Leave {
        Leave {}
    }
}

impl Into<Verb> for Leave {
    fn into(self) -> Verb {
        Verb::Leave(self)
    }
}

impl Into<Vec<Verb>> for Leave {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}
