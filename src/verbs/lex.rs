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
        bot_id: String,
        bot_alias: String,
        access_key: String,
        secret_access_key: String,
        region: String,
    ) -> Lex {
        Lex {
            bot_id,
            bot_alias,
            region,
            locale: LexLocale::EnglishGB,
            event_hook: None,
            intent: None,
            welcome_message: None,
            no_input_timeout: None,
            tts: None,
            metadata: None,
            credentials: LexAWSCredentials {
                access_key,
                secret_access_key,
            },
        }
    }

    pub fn bot_alias(&mut self, bot_alias: String) -> &mut Lex {
        self.bot_alias = bot_alias;
        self
    }

    pub fn credentials(&mut self, credentials: LexAWSCredentials) -> &mut Lex {
        self.credentials = credentials;
        self
    }

    pub fn region(&mut self, region: String) -> &mut Lex {
        self.region = region;
        self
    }

    pub fn locale(&mut self, locale: LexLocale) -> &mut Lex {
        self.locale = locale;
        self
    }

    pub fn event_hook(&mut self, event_hook:  Option<String>) -> &mut Lex {
        self.event_hook = event_hook;
        self
    }

    pub fn intent(&mut self, intent: Option<String>) -> &mut Lex {
        self.intent = intent;
        self
    }

    pub fn welcome_message(&mut self, welcome_message: Option<String>) -> &mut Lex {
        self.welcome_message = welcome_message;
        self
    }

    pub fn no_input_timeout(&mut self, no_input_timeout: Option<u8>) -> &mut Lex {
        self.no_input_timeout = no_input_timeout;
        self
    }

    pub fn tts(&mut self, tts: Option<Synthesizer>) -> &mut Lex {
        self.tts = tts;
        self
    }

    pub fn metadata(&mut self, metadata: Option<LexMeta>) -> &mut Lex {
        self.metadata = metadata;
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
    pub fn new(access_key: String, secret_access_key: String) -> LexAWSCredentials {
        LexAWSCredentials {
            access_key,
            secret_access_key,
        }
    }
}
