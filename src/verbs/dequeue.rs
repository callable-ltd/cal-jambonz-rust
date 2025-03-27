
    use serde::{Deserialize, Serialize};
    use crate::verbs::verb::Verb;

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Dequeue {
        pub name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub action_hook: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub beep: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timeout: Option<u8>,
    }

    impl Into<Verb> for Dequeue {
        fn into(self) -> Verb {
            Verb::Dequeue(self)
        }
    }

    impl Into<Vec<Verb>> for Dequeue {
        fn into(self) -> Vec<Verb> {
            vec![self.into()]
        }
    }

    impl Dequeue {
        pub fn new(name: &str) -> Dequeue {
            Dequeue {
                name: name.to_string(),
                action_hook: None,
                beep: None,
                timeout: None,
            }
        }

        pub fn action_hook(&mut self, action: &str) -> &mut Dequeue {
            self.action_hook = Some(action.to_string());
            self
        }

        pub fn beep(&mut self, beep: bool) -> &mut Dequeue {
            self.beep = Some(beep);
            self
        }

        pub fn timeout(&mut self, timeout: u8) -> &mut Dequeue {
            self.timeout = Some(timeout);
            self
        }
    }

