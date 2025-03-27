use crate::verb::Verb;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub data: HashMap<String, String>,
}

impl Tag {
    pub fn new() -> Tag {
        Tag {
            data: HashMap::new(),
        }
    }
    pub fn insert(&mut self, key: String, val: String) -> &mut Tag {
        self.data.insert(key, val);
        self
    }

    pub fn extend(&mut self, data: HashMap<String, String>) -> &mut Tag {
        self.data.extend(data);
        self
    }

    pub fn replace(&mut self, data: HashMap<String, String>) -> &mut Tag {
        self.data = data;
        self
    }
}
impl Into<Verb> for Tag {
    fn into(self) -> Verb {
        Verb::Tag(self)
    }
}

impl Into<Vec<Verb>> for Tag {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}
