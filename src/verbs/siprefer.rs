use crate::verb::Verb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct SipRefer {
        pub refer_to: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub action_hook: Option<String>,
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
