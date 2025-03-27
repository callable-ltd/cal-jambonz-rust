use serde::{Deserialize, Serialize};
use crate::verbs::verb::Verb;

#[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Enqueue {
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub priority: Option<u16>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub action_hook: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub wait_hook: Option<String>,
    }

    impl Into<Verb> for Enqueue {
        fn into(self) -> Verb {
            Verb::Enqueue(self)
        }
    }

    impl Into<Vec<Verb>> for Enqueue {
        fn into(self) -> Vec<Verb> {
            vec![self.into()]
        }
    }
