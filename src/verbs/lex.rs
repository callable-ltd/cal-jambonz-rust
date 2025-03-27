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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_hook: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub welcome_message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_input_timeout: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<Synthesizer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LexMeta>,
}

impl Lex {
    pub fn new(
        bot_id: &str,
        bot_alias: &str,
        access_key: &str,
        secret_access_key: &str,
        region: &str,
    ) -> Lex {
        Lex {
            bot_id: bot_id.to_string(),
            bot_alias: bot_alias.to_string(),
            credentials: LexAWSCredentials {
                access_key: access_key.to_string(),
                secret_access_key: secret_access_key.to_string(),
            },
            region: region.to_string(),
            locale: LexLocale::EnglishGB,
            event_hook: None,
            intent: None,
            welcome_message: None,
            no_input_timeout: None,
            tts: None,
            metadata: None,
        }
    }

    pub fn bot_alias(&mut self, bot_alias: &str) -> &mut Lex {
        self.bot_alias = bot_alias.to_string();
        self
    }

    pub fn credentials(&mut self, credentials: LexAWSCredentials) -> &mut Lex {
        self.credentials = credentials;
        self
    }

    pub fn region(&mut self, region: &str) -> &mut Lex {
        self.region = region.to_string();
        self
    }

    pub fn locale(&mut self, locale: LexLocale) -> &mut Lex {
        self.locale = locale;
        self
    }

    pub fn event_hook(&mut self, event_hook: &str) -> &mut Lex {
        self.event_hook = Some(event_hook.to_string());
        self
    }

    pub fn intent(&mut self, intent: &str) -> &mut Lex {
        self.intent = Some(intent.to_string());
        self
    }

    pub fn welcome_message(&mut self, welcome_message: &str) -> &mut Lex {
        self.welcome_message = Some(welcome_message.to_string());
        self
    }

    pub fn no_input_timeout(&mut self, no_input_timeout: u8) -> &mut Lex {
        self.no_input_timeout = Some(no_input_timeout);
        self
    }

    pub fn tts(&mut self, tts: Synthesizer) -> &mut Lex {
        self.tts = Some(tts);
        self
    }

    pub fn metadata(&mut self, metadata: LexMeta) -> &mut Lex {
        self.metadata = Some(metadata);
        self
    }
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

impl LexMeta {
    pub fn new() -> LexMeta {
        LexMeta {
            slots: None,
            context: None,
        }
    }

    pub fn slots(&mut self, slots: HashMap<String, String>) -> &mut LexMeta {
        self.slots = Some(slots);
        self
    }

    pub fn context(&mut self, context: HashMap<String, String>) -> &mut LexMeta {
        self.context = Some(context);
        self
    }
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

impl LexAWSCredentials {
    pub fn new(access_key: &str, secret_access_key: &str) -> LexAWSCredentials {
        LexAWSCredentials {
            access_key: access_key.to_string(),
            secret_access_key: secret_access_key.to_string(),
        }
    }
}
