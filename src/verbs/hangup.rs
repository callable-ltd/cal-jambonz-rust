use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Hangup {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, String>,
}

impl Into<Verb> for Hangup {
    fn into(self) -> Verb {
        Verb::Hangup(self)
    }
}

impl Into<Vec<Verb>> for Hangup {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}

impl Hangup {
    pub fn hangup() -> Hangup {
        Hangup {
            headers: HashMap::new(),
        }
    }

    pub fn hangup_with_reason(x_reason: String) -> Hangup {
        let mut map = HashMap::new();
        map.insert("X-Reason".to_string(), x_reason);
        Hangup { headers: map }
    }

    pub fn reason(&mut self, str: String) -> &mut Hangup {
        self.headers.insert("X-Reason".to_string(), str);
        self
    }

    pub fn add_header(&mut self, key: &str, value: String) -> &mut Hangup {
        self.headers.insert(key.to_string(), value);
        self
    }

    pub fn add_headers(&mut self, headers: HashMap<String, String>) -> &mut Hangup {
        self.headers.extend(headers);
        self
    }

    pub fn replace_headers(&mut self, headers: HashMap<String, String>) -> &mut Hangup {
        self.headers = headers;
        self
    }
}
