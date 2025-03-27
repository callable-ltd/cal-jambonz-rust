    use serde::{Deserialize, Serialize};
    use crate::verb::Verb;

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Redirect {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub action_hook: Option<String>,
    }

    impl Into<Verb> for Redirect {
        fn into(self) -> Verb {
            Verb::Redirect(self)
        }
    }

    impl Into<Vec<Verb>> for Redirect {
        fn into(self) -> Vec<Verb> {
            vec![self.into()]
        }
    }
