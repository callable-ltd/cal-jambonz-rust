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
    pub fn new(from: &str, to: &str, text: &str) -> Message {
        Message {
            from: from.to_string(),
            to: to.to_string(),
            text: text.to_string(),
            carrier: None,
            action_hook: None,
        }
    }
    
    pub fn with_action_hook(&mut self, action_hook: &str) -> &mut Message {
        self.action_hook = Some(action_hook.to_string());
        self
    }
    
    pub fn with_carrier(&mut self, carrier: &str) -> &mut Message {
        self.carrier = Some(carrier.to_string());
        self
    }
    
    pub fn with_action(&mut self, action_hook: &str) -> &mut Message {
        self.action_hook = Some(action_hook.to_string());
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
