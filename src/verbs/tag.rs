use crate::verb::Verb;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Tag {
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        pub data: HashMap<String, String>,
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
