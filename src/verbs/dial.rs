    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use crate::verbs::amd::Amd;
    use crate::verbs::constants::{DEFAULT_CALL_TIME, DEFAULT_RING_TIME, HANG_UP_CONNECT, RING_TONE};
    use crate::verbs::dub::DubData;
    use crate::verbs::listen::Listen;
    use crate::verbs::recognizer::Recognizer;
    use crate::verbs::target::Target;
    use crate::verbs::verb::Verb;

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Dial {
        pub target: Vec<Target>,

        #[serde(rename = "actionHook")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub action_hook: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub amd: Option<Amd>,

        #[serde(rename = "answerOnBridge")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub answer_on_bridge: Option<bool>,

        #[serde(rename = "callerId")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub caller_id: Option<String>,

        #[serde(rename = "confirmHook")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub confirm_hook: Option<String>,

        #[serde(rename = "dialMusic")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dial_music: Option<String>,

        #[serde(rename = "dtmfCapture")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dtmf_capture: Option<Vec<String>>,

        #[serde(rename = "dtmfHook")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dtmf_hook: Option<String>,

        #[serde(skip_serializing_if = "HashMap::is_empty")]
        pub headers: HashMap<String, String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub listen: Option<Listen>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub refer_hook: Option<String>,

        #[serde(rename = "timeLimit")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub time_limit: Option<u16>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub timeout: Option<u8>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub transcribe: Option<TranscribeDial>,

        #[serde(rename = "exitMediaPath")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub exit_media_path: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub dub: Option<DubData>,

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

        pub fn action_hook(&mut self, str: &str) -> &mut Dial {
            self.action_hook = Some(str.to_string());
            self
        }
        pub fn caller_id(&mut self, str: &str) -> &mut Dial {
            self.caller_id = Some(str.to_string());
            self
        }

        pub fn dial_music(&mut self, str: &str) -> &mut Dial {
            self.dial_music = Some(str.to_string());
            self
        }

        pub fn timeout(&mut self, limit: u8) -> &mut Dial {
            self.timeout = Some(limit);
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

        pub fn time_limit(&mut self, limit: u16) -> &mut Dial {
            self.time_limit = Some(limit);
            self
        }

        pub fn amd(&mut self, amd: Amd) -> &mut Dial {
            self.amd = Some(amd);
            self
        }

        pub fn answer_on_bridge(&mut self, answer_on_bridge: bool) -> &mut Dial {
            self.answer_on_bridge = Some(answer_on_bridge);
            self
        }

        pub fn confirm_hook(&mut self, confirm_hook: &str) -> &mut Dial {
            self.confirm_hook = Some(confirm_hook.to_string());
            self
        }

        pub fn dtmf_capture(&mut self, dtmf_capture: Vec<&str>) -> &mut Dial {
            let vec = dtmf_capture
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            self.dtmf_capture = Some(vec);
            self
        }

        pub fn dtmf_hook(&mut self, dtmf_hook: &str) -> &mut Dial {
            self.dtmf_hook = Some(dtmf_hook.to_string());
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

        pub fn listen(&mut self, listen: Listen) -> &mut Dial {
            self.listen = Some(listen);
            self
        }

        pub fn refer_hook(&mut self, refer_hook: &str) -> &mut Dial {
            self.refer_hook = Some(refer_hook.to_string());
            self
        }

        pub fn transcribe(&mut self, transcribe: TranscribeDial) -> &mut Dial {
            self.transcribe = Some(transcribe);
            self
        }

        pub fn exit_media_path(&mut self, exit_media_path: bool) -> &mut Dial {
            self.exit_media_path = Some(exit_media_path);
            self
        }

        pub fn dub(&mut self, dub: DubData) -> &mut Dial {
            self.dub = Some(dub);
            self
        }

        pub fn anchor_media(&mut self, anchor_media: bool) -> &mut Dial {
            self.anchor_media = Some(anchor_media);
            self
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct TranscribeDial {
      
        #[serde(rename = "transcriptionHook")]
        pub transcription_hook: String,
       
        pub recognizer: Recognizer,
    }

    impl TranscribeDial {
        pub fn new(transcription_hook: &str, recognizer: Recognizer) -> TranscribeDial {
            TranscribeDial {
                transcription_hook: transcription_hook.to_string(),
                recognizer,
            }
        }
    }
