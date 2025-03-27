    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct BargeIn {
        pub input: Vec<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub enable: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub sticky: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub action_hook: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub finish_on_key: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub num_digits: Option<u8>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub min_digits: Option<u8>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub max_digits: Option<u8>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub inter_digit_timeout: Option<u8>,
    }
