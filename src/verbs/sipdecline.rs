use crate::shared::shared::SIPStatus;
use crate::verb::Verb;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SipDecline {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SIPStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, String>,
}

impl Into<Verb> for SipDecline {
    fn into(self) -> Verb {
        Verb::SipDecline(self)
    }
}

impl Into<Vec<Verb>> for SipDecline {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}

impl SipDecline {
    pub fn new(status: SIPStatus) -> SipDecline {
        SipDecline {
            status: Some(status),
            reason: None,
            headers: HashMap::new(),
        }
    }

    pub fn server_error(reason: String, x_reason: String) -> SipDecline {
        let mut map = HashMap::new();
        map.insert("X-Reason".to_string(), x_reason);
        SipDecline {
            status: Some(SIPStatus::InternalServerError),
            reason: Some(reason),
            headers: map,
        }
    }

    pub fn unauthorised(reason: String, x_reason: String) -> SipDecline {
        let mut map = HashMap::new();
        map.insert("X-Reason".to_string(), x_reason);
        SipDecline {
            status: Some(SIPStatus::Unauthorized),
            reason: Some(reason),
            headers: map,
        }
    }

    pub fn not_found(reason: String, x_reason: String) -> SipDecline {
        let mut map = HashMap::new();
        map.insert("X-Reason".to_string(), x_reason);
        SipDecline {
            status: Some(SIPStatus::Decline),
            reason: Some(reason),
            headers: map,
        }
    }

    pub fn not_implemented(reason: String) -> SipDecline {
        let mut map = HashMap::new();
        map.insert("X-Reason".to_string(), "Not Implemented".to_string());
        SipDecline {
            status: Some(SIPStatus::NotImplemented),
            reason: Some(reason),
            headers: map,
        }
    }

    pub fn reason(&mut self, reason: &str) -> &mut SipDecline {
        self.reason = Some(reason.to_string());
        self.headers.insert("X-Reason".to_string(), reason.to_string());
        self
    }

    pub fn add_header(&mut self, key: &str, value: &str) -> &mut SipDecline {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn add_headers(&mut self, headers: HashMap<String, String>) -> &mut SipDecline {
        self.headers.extend(headers);
        self
    }
    
    pub fn replace_headers(&mut self, headers: HashMap<String, String>) -> &mut SipDecline {
        self.headers = headers;
        self
    }
}
