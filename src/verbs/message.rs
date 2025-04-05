use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub from: String,

    pub to: String,

    pub text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_hook: Option<String>,
}

impl Message {
    pub fn new(from: String, to: String, text: String) -> Message {
        Message {
            from,
            to,
            text,
            carrier: None,
            action_hook: None,
        }
    }
    
    pub fn with_action_hook(&mut self, action_hook: Option<String>) -> &mut Message {
        self.action_hook = action_hook;
        self
    }
    
    pub fn with_carrier(&mut self, carrier: Option<String>) -> &mut Message {
        self.carrier = carrier;
        self
    }
    
    pub fn with_action(&mut self, action_hook: Option<String>) -> &mut Message {
        self.action_hook = action_hook;
        self
    }
    
}
impl Into<Verb> for Message {
    fn into(self) -> Verb {
        Verb::Message(self)
    }
}

impl Into<Vec<Verb>> for Message {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}
