use crate::verb::Verb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Redirect {
    pub action_hook: String,
}

impl Redirect {
    pub fn new(action_hook: String) -> Redirect {
        Redirect { action_hook }
    }
}

impl Into<Verb> for Redirect {
    fn into(self) -> Verb {
        Verb::Redirect(self)
    }
}

impl Into<Vec<Verb>> for Redirect {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}
