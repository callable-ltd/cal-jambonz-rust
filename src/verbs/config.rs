use crate::verbs::bargein::BargeIn;
use crate::verbs::listen::Listen;
use crate::verbs::recognizer::Recognizer;
use crate::verbs::siprec::SipRec;
use crate::verbs::synthesizer::Synthesizer;
use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amd: Option<bool>,

    #[serde(rename = "bargeIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub barge_in: Option<BargeIn>,

    #[serde(rename = "boostAudioSignal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_audio_signal: Option<String>,

    #[serde(rename = "fillerNoise")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filler_noise: Option<FillerNoise>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen: Option<Listen>,

    #[serde(rename = "notifyEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_events: Option<bool>,

    #[serde(rename = "onHoldMusic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_hold_music: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognizer: Option<Recognizer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<SipRec>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcribe: Option<TranscribeConfig>,

    #[serde(rename = "sipRequestWithinDialogHook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sip_request_within_dialog_hook: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthesizer: Option<Synthesizer>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            amd: None,
            barge_in: None,
            boost_audio_signal: None,
            filler_noise: None,
            listen: None,
            notify_events: None,
            on_hold_music: None,
            recognizer: None,
            reset: None,
            record: None,
            sip_request_within_dialog_hook: None,
            transcribe: None,
            synthesizer: None,
        }
    }

    pub fn amd(&mut self, amd: Option<bool>) -> &mut Config {
        self.amd = amd;
        self
    }

    pub fn barge_in(&mut self, barge_in: Option<BargeIn>) -> &mut Config {
        self.barge_in = barge_in;
        self
    }

    pub fn boost_audio_signal(&mut self, boost_audio_signal: Option<String>) -> &mut Config {
        self.boost_audio_signal = boost_audio_signal;
        self
    }

    pub fn filler_noise(&mut self, filler_noise: Option<FillerNoise>) -> &mut Config {
        self.filler_noise = filler_noise;
        self
    }

    pub fn listen(&mut self, listen: Option<Listen>) -> &mut Config {
        self.listen = listen;
        self
    }

    pub fn notify_events(&mut self, notify_events: Option<bool>) -> &mut Config {
        self.notify_events = notify_events;
        self
    }

    pub fn on_hold_music(&mut self, on_hold_music: Option<String>) -> &mut Config {
        self.on_hold_music = on_hold_music;
        self
    }

    pub fn recognizer(&mut self, recognizer: Option<Recognizer>) -> &mut Config {
        self.recognizer = recognizer;
        self
    }

    pub fn reset(&mut self, reset: Option<Vec<String>>) -> &mut Config {
        self.reset = reset;
        self
    }

    pub fn record(&mut self, record: Option<SipRec>) -> &mut Config {
        self.record = record;
        self
    }

    pub fn transcribe(&mut self, transcribe: Option<TranscribeConfig>) -> &mut Config {
        self.transcribe = transcribe;
        self
    }

    pub fn synthesizer(&mut self, synthesizer: Option<Synthesizer>) -> &mut Config {
        self.synthesizer = synthesizer;
        self
    }

    pub fn sip_request_within_dialog_hook(
        &mut self,
        sip_request_within_dialog_hook: Option<String>,
    ) -> &mut Config {
        self.sip_request_within_dialog_hook = sip_request_within_dialog_hook;
        self
    }
}
impl Into<Verb> for Config {
    fn into(self) -> Verb {
        Verb::Config(self)
    }
}

impl Into<Vec<Verb>> for Config {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TranscribeConfig {
    pub enable: bool,

    #[serde(rename = "transcriptionHook")]
    pub transcription_hook: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognizer: Option<Recognizer>,
}

impl TranscribeConfig {
    fn new(enable: bool, transcription_hook: String) -> TranscribeConfig {
        TranscribeConfig {
            enable,
            transcription_hook,
            recognizer: None,
        }
    }

    pub fn recognizer(&mut self, recognizer: Option<Recognizer>) -> &mut TranscribeConfig {
        self.recognizer = recognizer;
        self
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FillerNoise {
    pub enabled: bool,

    pub url: String,

    #[serde(rename = "startDelaySecs")]
    pub start_delay_secs: Option<u8>,
}

impl FillerNoise {
    pub fn new(enabled: bool, url: String) -> FillerNoise {
        FillerNoise {
            enabled,
            url,
            start_delay_secs: None,
        }
    }

    pub fn start_delay(&mut self, start_delay_secs: Option<u8>) -> &mut FillerNoise {
        self.start_delay_secs = start_delay_secs;
        self
    }
}
