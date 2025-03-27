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

    pub fn amd(&mut self, amd: bool) -> &mut Config {
        self.amd = Some(amd);
        self
    }

    pub fn barge_in(&mut self, barge_in: BargeIn) -> &mut Config {
        self.barge_in = Some(barge_in);
        self
    }

    pub fn boost_audio_signal(&mut self, boost_audio_signal: &str) -> &mut Config {
        self.boost_audio_signal = Some(boost_audio_signal.to_string());
        self
    }

    pub fn filler_noise(&mut self, filler_noise: FillerNoise) -> &mut Config {
        self.filler_noise = Some(filler_noise);
        self
    }

    pub fn listen(&mut self, listen: Listen) -> &mut Config {
        self.listen = Some(listen);
        self
    }

    pub fn notify_events(&mut self, notify_events: bool) -> &mut Config {
        self.notify_events = Some(notify_events);
        self
    }

    pub fn on_hold_music(&mut self, on_hold_music: &str) -> &mut Config {
        self.on_hold_music = Some(on_hold_music.to_string());
        self
    }

    pub fn recognizer(&mut self, recognizer: Recognizer) -> &mut Config {
        self.recognizer = Some(recognizer);
        self
    }

    pub fn reset(&mut self, reset: Vec<String>) -> &mut Config {
        self.reset = Some(reset);
        self
    }

    pub fn record(&mut self, record: SipRec) -> &mut Config {
        self.record = Some(record);
        self
    }

    pub fn transcribe(&mut self, transcribe: TranscribeConfig) -> &mut Config {
        self.transcribe = Some(transcribe);
        self
    }

    pub fn synthesizer(&mut self, synthesizer: Synthesizer) -> &mut Config {
        self.synthesizer = Some(synthesizer);
        self
    }

    pub fn sip_request_within_dialog_hook(
        &mut self,
        sip_request_within_dialog_hook: &str,
    ) -> &mut Config {
        self.sip_request_within_dialog_hook = Some(sip_request_within_dialog_hook.to_string());
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
    fn new(enable: bool, transcription_hook: &str) -> TranscribeConfig {
        TranscribeConfig {
            enable,
            transcription_hook: transcription_hook.to_string(),
            recognizer: None,
        }
    }

    pub fn recognizer(&mut self, recognizer: Recognizer) -> &mut TranscribeConfig {
        self.recognizer = Some(recognizer);
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
    pub fn new(enabled: bool, url: &str) -> FillerNoise {
        FillerNoise {
            enabled,
            url: url.to_string(),
            start_delay_secs: None,
        }
    }

    pub fn start_delay(&mut self, start_delay_secs: u8) -> &mut FillerNoise {
        self.start_delay_secs = Some(start_delay_secs);
        self
    }
}
