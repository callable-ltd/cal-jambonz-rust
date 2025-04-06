use crate::verbs::amd::Amd;
use crate::verbs::constants::{DEFAULT_CALL_TIME, DEFAULT_RING_TIME, HANG_UP_CONNECT, RING_TONE};
use crate::verbs::dub::DubData;
use crate::verbs::recognizer::Recognizer;
use crate::verbs::target::Target;
use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::listen::Listen;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dial {
    /// Array of up to 10 destinations to simultaneously dial.
    /// The first person (or entity) to answer the call will
    /// be connected to the caller, and the rest of the called
    /// numbers will be hung up.
    pub target: Vec<Target>,

    /// Webhook to invoke when the call ends.
    /// The webhook will include properties
    /// describing the outcome of the call attempt
    #[serde(rename = "actionHook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_hook: Option<String>,

    /// Enable answering machine detection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amd: Option<Amd>,

    /// If set to true, the inbound call will ring until the number
    /// that was dialed answers the call, and at that point a 200
    /// OK will be sent on the inbound leg.
    ///
    /// If false, the inbound call will be answered immediately as
    /// the outbound call is placed.
    ///
    /// Defaults to false.
    #[serde(rename = "answerOnBridge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_on_bridge: Option<bool>,

    /// The inbound caller’s phone number, which is displayed
    /// to the number that was dialed.
    ///
    /// The caller ID must be a valid E.164 number.
    ///
    /// Defaults to caller ID on inbound call.
    #[serde(rename = "callerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_id: Option<String>,

    /// Webhook for an application to run on the callee’s end after
    /// the dialed number answers but before the call is connected.
    ///
    /// This allows the caller to provide information to the dialed number,
    /// giving them the opportunity to decline the call before they answer.
    ///
    /// Note that if you want to run different applications on specific destinations,
    /// you can specify the ‘url’ property on the nested target object.
    #[serde(rename = "confirmHook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_hook: Option<String>,

    /// Url that specifies a .wav or .mp3 audio file of custom audio or ringback
    /// to play to the caller while the outbound call is ringing.
    #[serde(rename = "dialMusic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial_music: Option<String>,

    /// An array of strings that represent DTMF sequences which, when detected,
    /// will trigger a mid-call notification to the application
    /// via the configured dtmfHook.
    #[serde(rename = "dtmfCapture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtmf_capture: Option<Vec<String>>,

    /// A webhook to call when a dtmfCapture entry is matched.
    ///
    /// This is a notification only — no response is expected,
    /// and any desired actions must be carried out via
    /// the REST updateCall API.
    #[serde(rename = "dtmfHook")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtmf_hook: Option<String>,

    /// An object containing arbitrary SIP headers
    /// to apply to the outbound call attempt(s).
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, String>,

    /// A nested listen action, which will cause audio from the call
    /// to be streamed to a remote server over a websocket connection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen: Option<Listen>,

    /// Webhook to invoke when an incoming SIP REFER is received on a dialed call.
    ///
    /// If the application wishes to accept and process the REFER,
    /// the webhook application should return an HTTP status code 200 with no body,
    /// and jambonz will send a SIP 202 Accepted.
    // Otherwise, any HTTP non-success status will cause jambonz to send a SIP response to the REFER with the same status code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refer_hook: Option<String>,

    #[serde(rename = "timeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_limit: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcribe: Option<TranscribeDial>,

    /// if true, jambonz will attempt to re-invite itself completely
    /// out of the media path for the call.
    ///
    /// Defaults to false.
    #[serde(rename = "exitMediaPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_media_path: Option<bool>,

    /// A nested dub verb to add additional audio
    /// tracks into the outbound call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dub: Option<DubData>,

    /// If true, jambonz will not release the media
    /// from feature server for the bridged call
    /// Default: false
    #[serde(rename = "anchorMedia")]
    #[serde(skip_serializing_if = "Option::is_none")]
    anchor_media: Option<bool>,
}

impl Into<Verb> for Dial {
    fn into(self) -> Verb {
        Verb::Dial(self)
    }
}

impl Into<Vec<Verb>> for Dial {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}

impl Dial {
    pub fn new(caller_id: &str, targets: Vec<Target>) -> Dial {
        Dial {
            action_hook: Some(HANG_UP_CONNECT.to_string()),
            caller_id: Some(caller_id.to_string()),
            dial_music: Some(RING_TONE.to_string()),
            time_limit: Some(DEFAULT_CALL_TIME),
            timeout: Some(DEFAULT_RING_TIME),
            headers: HashMap::new(),
            target: targets,
            amd: None,
            answer_on_bridge: None,
            confirm_hook: None,
            dtmf_capture: None,
            dtmf_hook: None,
            listen: None,
            refer_hook: None,
            transcribe: None,
            exit_media_path: None,
            dub: None,
            anchor_media: None,
        }
    }

    pub fn action_hook(&mut self, action_hook: Option<String>) -> &mut Dial {
        self.action_hook = action_hook;
        self
    }
    pub fn caller_id(&mut self, caller_id: Option<String>) -> &mut Dial {
        self.caller_id = caller_id;
        self
    }

    pub fn dial_music(&mut self, str: Option<String>) -> &mut Dial {
        self.dial_music = str;
        self
    }

    pub fn timeout(&mut self, limit: Option<u8>) -> &mut Dial {
        self.timeout = limit;
        self
    }

    pub fn target(&mut self, target: Target) -> &mut Dial {
        self.target.push(target);
        self
    }

    pub fn replace_targets(&mut self, targets: Vec<Target>) -> &mut Dial {
        self.target = targets;
        self
    }

    pub fn add_targets(&mut self, targets: Vec<Target>) -> &mut Dial {
        targets.into_iter().for_each(|target| {
            self.target.push(target);
        });
        self
    }

    pub fn time_limit(&mut self, limit: Option<u16>) -> &mut Dial {
        self.time_limit = limit;
        self
    }

    pub fn amd(&mut self, amd: Option<Amd>) -> &mut Dial {
        self.amd = amd;
        self
    }

    pub fn answer_on_bridge(&mut self, answer_on_bridge: Option<bool>) -> &mut Dial {
        self.answer_on_bridge = answer_on_bridge;
        self
    }

    pub fn confirm_hook(&mut self, confirm_hook: Option<String>) -> &mut Dial {
        self.confirm_hook = confirm_hook;
        self
    }

    pub fn dtmf_capture(&mut self, dtmf_capture: Option<Vec<String>>) -> &mut Dial {
        self.dtmf_capture = dtmf_capture;
        self
    }

    pub fn dtmf_hook(&mut self, dtmf_hook: Option<String>) -> &mut Dial {
        self.dtmf_hook = dtmf_hook;
        self
    }

    pub fn header(&mut self, key: &str, value: &str) -> &mut Dial {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn replace_headers(&mut self, headers: HashMap<String, String>) -> &mut Dial {
        self.headers = headers;
        self
    }

    pub fn add_headers(&mut self, headers: HashMap<String, String>) -> &mut Dial {
        self.headers.extend(headers);
        self
    }

    pub fn listen(&mut self, listen: Option<Listen>) -> &mut Dial {
        self.listen = listen;
        self
    }

    pub fn refer_hook(&mut self, refer_hook: Option<String>) -> &mut Dial {
        self.refer_hook = refer_hook;
        self
    }

    pub fn transcribe(&mut self, transcribe: Option<TranscribeDial>) -> &mut Dial {
        self.transcribe = transcribe;
        self
    }

    pub fn exit_media_path(&mut self, exit_media_path: Option<bool>) -> &mut Dial {
        self.exit_media_path = exit_media_path;
        self
    }

    pub fn dub(&mut self, dub: Option<DubData>) -> &mut Dial {
        self.dub = dub;
        self
    }

    pub fn anchor_media(&mut self, anchor_media: Option<bool>) -> &mut Dial {
        self.anchor_media = anchor_media;
        self
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TranscribeDial {
    #[serde(rename = "transcriptionHook")]
    pub transcription_hook: String,
    #[serde(flatten)]
    pub recognizer: Recognizer,
}

impl TranscribeDial {
    pub fn new(transcription_hook: String, recognizer: Recognizer) -> TranscribeDial {
        TranscribeDial {
            transcription_hook,
            recognizer,
        }
    }
}
