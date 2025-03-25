use crate::websocket::Redirect;
use crate::{
    Ack, CallStatus, CallStatusData, CallStatusValue, Command, CommandType, CommandValue,
    ConferenceHoldStatus, ConferenceHoldStatusData, ConferenceHoldStatusValue,
    ConferenceMuteStatus, ConferenceMuteStatusData, ConferenceMuteStatusValue, Dub,
    ListenStatus, ListenStatusData, ListenStatusValue, MuteStatus, MuteStatusData, MuteStatusValue,
    Play, Record, RecordData, ResumeCallRecording, Say, Synthesizer, Verb, WebsocketReply, Whisper,
};

impl Command {
    pub fn new(command_type: CommandType, queue_command: bool) -> Command {
        match command_type {
            CommandType::Redirect => Command {
                command_type: CommandValue::Redirect(Redirect {
                    queue_command,
                    data: None,
                }),
            },
            CommandType::CallStatus => Command {
                command_type: CommandValue::CallStatus(CallStatus {
                    queue_command,
                    data: CallStatusData {
                        call_status: CallStatusValue::Completed,
                    },
                }),
            },
            CommandType::ListenStatus => Command {
                command_type: CommandValue::ListenStatus(ListenStatus {
                    queue_command,
                    data: ListenStatusData {
                        listen_status: ListenStatusValue::Resume,
                    },
                }),
            },
            CommandType::MuteStatus => Command {
                command_type: CommandValue::MuteStatus(MuteStatus {
                    queue_command,
                    data: MuteStatusData {
                        mute_status: MuteStatusValue::UnMute,
                    },
                }),
            },
            CommandType::ConferenceMuteStatus => Command {
                command_type: CommandValue::ConferenceMuteStatus(ConferenceMuteStatus {
                    queue_command,
                    data: ConferenceMuteStatusData {
                        conf_mute_status: ConferenceMuteStatusValue::UnMute,
                    },
                }),
            },
            CommandType::ConferenceHoldStatus => Command {
                command_type: CommandValue::ConferenceHoldStatus(ConferenceHoldStatus {
                    queue_command,
                    data: ConferenceHoldStatusData {
                        conf_hold_status: ConferenceHoldStatusValue::UnHold,
                    },
                }),
            },
            CommandType::Record => Command {
                command_type: CommandValue::Record(Record {
                    queue_command,
                    data: RecordData::ResumeCallRecording(ResumeCallRecording {}),
                }),
            },
            CommandType::Whisper => Command {
                command_type: CommandValue::Whisper(Whisper {
                    queue_command,
                    data: Vec::new(),
                }),
            },
            CommandType::Dub => Command {
                command_type: CommandValue::Dub(Dub {
                    queue_command,
                    data: Vec::new(),
                }),
            },
        }
    }
}

impl Ack {
    pub fn new(msg_id: &str) -> Ack {
        Ack {
            msgid: msg_id.to_string(),
            data: Some(Vec::new()),
        }
    }

    pub fn say(
        &mut self,
        text: &str,
        say_loop: Option<u8>,
        synthesizer: Option<Synthesizer>,
        early_media: Option<bool>,
    ) -> &mut Ack {
        let say = Say {
            text: text.to_string(),
            say_loop,
            synthesizer,
            early_media,
        };
        self.data.get_or_insert_default().push(Verb::Say(say));
        self
    }

    pub fn say_text(&mut self, text: &str) -> &mut Ack {
        let say = Say {
            text: text.to_string(),
            say_loop: Some(1),
            synthesizer: None,
            early_media: Some(false),
        };
        self.data.get_or_insert_default().push(Verb::Say(say));
        self
    }

    pub fn say_struct(&mut self, say: Say) -> &mut Ack {
        self.data.get_or_insert_default().push(Verb::Say(say));
        self
    }

    pub fn play(
        &mut self,
        url: &str,
        early_media: Option<bool>,
        play_loop: Option<u8>,
        seek_offset: Option<u16>,
        timeout_secs: Option<u8>,
        action_hook: Option<String>,
    ) -> &mut Ack {
        let play = Play {
            url: url.to_string(),
            early_media,
            play_loop,
            seek_offset,
            timeout_secs,
            action_hook,
        };
        self.data.get_or_insert_default().push(Verb::Play(play));
        self
    }

    pub fn play_url(&mut self, url: &str) -> &mut Ack {
        let play = Play::new(url, None);
        self.data.get_or_insert_default().push(Verb::Play(play));
        self
    }

    pub fn play_struct(&mut self, play: Play) -> &mut Ack {
        self.data.get_or_insert_default().push(Verb::Play(play));
        self
    }

    pub fn ack(&mut self) -> WebsocketReply {
        WebsocketReply::Ack(Ack {
            msgid: self.msgid.to_string(),
            data: self.data.clone(),
        })
    }
}

#[test]
fn json() {
    let response = Ack::new("1234")
        .say_text("Welcome to Callable")
        .ack()
        .json();
    println!("{:#?}", response);
}
