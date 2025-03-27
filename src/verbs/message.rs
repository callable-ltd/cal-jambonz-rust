    use serde::{Deserialize, Serialize};
    use crate::verbs::verb::Verb;

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
