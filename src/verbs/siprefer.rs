use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SipRefer {
    pub refer_to: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_hook: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_hook: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    referred_by: Option<String>,

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, String>,
}

impl SipRefer {
    pub fn new(refer_to: &str) -> SipRefer {
        SipRefer {
            refer_to: refer_to.to_string(),
            action_hook: None,
            event_hook: None,
            headers: HashMap::new(),
            referred_by: None,
        }
    }

    pub fn action_hook(&mut self, hook: &str) -> &mut SipRefer {
        self.action_hook = Some(hook.to_string());
        self
    }

    pub fn event_hook(&mut self, hook: &str) -> &mut SipRefer {
        self.event_hook = Some(hook.to_string());
        self
    }

    pub fn referred_by(&mut self, referred_by: &str) -> &mut SipRefer {
        self.referred_by = Some(referred_by.to_string());
        self
    }

    pub fn add_headers(&mut self, headers: HashMap<String, String>) -> &mut SipRefer {
        self.headers.extend(headers);
        self
    }

    pub fn replace_headers(&mut self, headers: HashMap<String, String>) -> &mut SipRefer {
        self.headers = headers;
        self
    }

    pub fn add_header(&mut self, key: &str, value: &str) -> &mut SipRefer {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
    
    
}
impl Into<Verb> for SipRefer {
    fn into(self) -> Verb {
        Verb::SipRefer(self)
    }
}

impl Into<Vec<Verb>> for SipRefer {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}
