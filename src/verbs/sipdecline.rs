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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub headers: Option<HashMap<String, String>>,
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
        pub fn custom(status: SIPStatus, reason: String, x_reason: String) -> SipDecline {
            let mut map = HashMap::new();
            map.insert("X-Reason".to_string(), x_reason);
            SipDecline {
                status: Some(status),
                reason: Some(reason),
                headers: Some(map),
            }
        }

        pub fn server_error(reason: String, x_reason: String) -> SipDecline {
            let mut map = HashMap::new();
            map.insert("X-Reason".to_string(), x_reason);
            SipDecline {
                status: Some(SIPStatus::InternalServerError),
                reason: Some(reason),
                headers: Some(map),
            }
        }

        pub fn unauthorised(reason: String, x_reason: String) -> SipDecline {
            let mut map = HashMap::new();
            map.insert("X-Reason".to_string(), x_reason);
            SipDecline {
                status: Some(SIPStatus::Unauthorized),
                reason: Some(reason),
                headers: Some(map),
            }
        }

        pub fn not_found(reason: String, x_reason: String) -> SipDecline {
            let mut map = HashMap::new();
            map.insert("X-Reason".to_string(), x_reason);
            SipDecline {
                status: Some(SIPStatus::Decline),
                reason: Some(reason),
                headers: Some(map),
            }
        }

        pub fn not_implemented(reason: String) -> SipDecline {
            let mut map = HashMap::new();
            map.insert("X-Reason".to_string(), "Not Implemented".to_string());
            SipDecline {
                status: Some(SIPStatus::NotImplemented),
                reason: Some(reason),
                headers: Some(map),
            }
        }
    }
