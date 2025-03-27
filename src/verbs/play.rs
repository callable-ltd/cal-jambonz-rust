    use serde::{Deserialize, Serialize};
    use crate::verbs::verb::Verb;
    
    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Play {
        pub url: String,
        #[serde(rename = "loop")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub play_loop: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub early_media: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub timeout_secs: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub seek_offset: Option<u16>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub action_hook: Option<String>,
    }

    impl Into<Verb> for Play {
        fn into(self) -> Verb {
            Verb::Play(self)
        }
    }

    impl Into<Vec<Verb>> for Play {
        fn into(self) -> Vec<Verb> {
            vec![self.into()]
        }
    }

    impl Play {
        pub fn new(url: &str, action_hook: Option<String>) -> Self {
            Play {
                url: url.to_string(),
                action_hook,
                play_loop: None,
                early_media: None,
                timeout_secs: None,
                seek_offset: None,
            }
        }
    }

