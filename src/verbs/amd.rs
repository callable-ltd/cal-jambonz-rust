
use crate::verbs::recognizer::Recognizer;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Amd {
    #[serde(rename = "actionHook")]
    pub action_hook: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognizer: Option<Recognizer>,
    #[serde(rename = "thresholdWordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_word_count: Option<u8>,
    #[serde(rename = "digitCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digit_count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timers: Option<Timers>,
}

impl Amd {
    pub fn new(action_hook: &str) -> Amd {
        Amd {
            action_hook: action_hook.to_string(),
            recognizer: None,
            threshold_word_count: None,
            digit_count: None,
            timers: None,
        }
    }

    pub fn recognizer(&mut self, recognizer: Recognizer) -> &mut Amd {
        self.recognizer = Some(recognizer);
        self
    }

    pub fn threshold_word_count(&mut self, threshold_word_count: u8) -> &mut Amd {
        self.threshold_word_count = Some(threshold_word_count);
        self
    }

    pub fn digit_count(&mut self, digit_count: u8) -> &mut Amd {
        self.digit_count = Some(digit_count);
        self
    }

    pub fn timers(&mut self, timers: Timers) -> &mut Amd {
        self.timers = Some(timers);
        self
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Timers {
    #[serde(rename = "decisionTimeoutMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    decision_timeout_ms: Option<u16>,
    #[serde(rename = "greetingCompletionTimeoutMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    greeting_completion_timeout_ms: Option<u16>,
    #[serde(rename = "noSpeechTimeoutMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    no_speech_timeout_ms: Option<u16>,
    #[serde(rename = "toneTimeoutMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    tone_timeout_ms: Option<u16>,
}

impl Timers {
    pub fn new() -> Timers {
        Timers {
            decision_timeout_ms: None,
            greeting_completion_timeout_ms: None,
            no_speech_timeout_ms: None,
            tone_timeout_ms: None,
        }
    }

    pub fn decision_timeout_ms(&mut self, decision_timeout_ms: u16) -> &mut Timers {
        self.decision_timeout_ms = Some(decision_timeout_ms);
        self
    }

    pub fn greeting_completion_timeout_ms(
        &mut self,
        greeting_completion_timeout_ms: u16,
    ) -> &mut Timers {
        self.greeting_completion_timeout_ms = Some(greeting_completion_timeout_ms);
        self
    }

    pub fn no_speech_timeout_ms(&mut self, no_speech_timeout_ms: u16) -> &mut Timers {
        self.no_speech_timeout_ms = Some(no_speech_timeout_ms);
        self
    }

    pub fn tone_timeout_ms(&mut self, tone_timeout_ms: u16) -> &mut Timers {
        self.tone_timeout_ms = Some(tone_timeout_ms);
        self
    }
}
