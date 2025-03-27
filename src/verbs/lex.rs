use crate::verbs::synthesizer::Synthesizer;
use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Lex {
        pub bot_id: String,
        pub bot_alias: String,
        pub credentials: LexAWSCredentials,
        pub region: String,
        pub locale: LexLocale,
        pub event_hook: Option<String>,
        pub intent: Option<String>,
        pub welcome_message: Option<String>,
        pub no_input_timeout: Option<u8>,
        pub tts: Option<Synthesizer>,
        pub metadata: Option<LexMeta>,
    }

    impl Into<Verb> for Lex {
        fn into(self) -> Verb {
            Verb::Lex(self)
        }
    }

    impl Into<Vec<Verb>> for Lex {
        fn into(self) -> Vec<Verb> {
            vec![self.into()]
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct LexMeta {
        pub slots: Option<HashMap<String, String>>,
        pub context: Option<HashMap<String, String>>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub enum LexLocale {
        #[serde(rename = "en_AU")]
        EnglishAU,
        #[serde(rename = "en_GB")]
        EnglishGB,
        #[serde(rename = "en_US")]
        EnglishUS,
        #[serde(rename = "fr_CA")]
        FrenchCA,
        #[serde(rename = "fr_FR")]
        FrenchFR,
        #[serde(rename = "es_ES")]
        FrenchES,
        #[serde(rename = "es_US")]
        SpanishUS,
        #[serde(rename = "it_IT")]
        ItalianIT,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct LexAWSCredentials {
        pub access_key: String,
        pub secret_access_key: String,
    }
