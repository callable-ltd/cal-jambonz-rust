use crate::conference::Conference;
use crate::config::Config;
use crate::dequeue::Dequeue;
use crate::dial::Dial;
use crate::dialogflow::DialogFlow;
use crate::dtmf::Dtmf;
use crate::dub::{Dub, DubData};
use crate::enqueue::Enqueue;
use crate::gather::{Gather, Input};
use crate::hangup::Hangup;
use crate::leave::Leave;
use crate::lex::Lex;
use crate::listen::Listen;
use crate::message::Message;
use crate::pause::Pause;
use crate::play::Play;
use crate::rasa::Rasa;
use crate::redirect::Redirect;
use crate::say::Say;
use crate::sipdecline::SipDecline;
use crate::siprec::SipRec;
use crate::siprefer::SipRefer;
use crate::tag::Tag;
use crate::target::{Phone, Sip, User};
use crate::transcribe::Transcribe;
use crate::verb::Verb;
use crate::ws::{
    Ack, CallStatus, Command, CommandValue, ConferenceHoldStatus, ConferenceMuteStatus,
    ListenStatus, MuteStatus, Record, SessionNew, Verbs, WSRedirect, WebsocketReply, Whisper,
};
use log::error;

trait VerbTrait {
    fn new(msg_id: &str) -> Self;
    fn conference(&mut self, conference: Conference) -> &mut Self;
    fn config(&mut self, config: Config) -> &mut Self;
    fn dequeue(&mut self, config: Dequeue) -> &mut Self;
    fn dial(&mut self, dial: Dial) -> &mut Self;
    fn dialog_flow(&mut self, dialog_flow: DialogFlow) -> &mut Self;
    fn dtmf(&mut self, dtmf: Dtmf) -> &mut Self;
    fn dub(&mut self, dub: DubData) -> &mut Self;
    fn enqueue(&mut self, enqueue: Enqueue) -> &mut Self;
    fn gather(&mut self, dial: Gather) -> &mut Self;
    fn hangup(&mut self) -> &mut Self;
    fn hangup_with_reason(&mut self, reason: &str) -> &mut Self;
    fn leave(&mut self, leave: Leave) -> &mut Self;
    fn lex(&mut self, lex: Lex) -> &mut Self;
    fn listen(&mut self, listen: Listen) -> &mut Self;
    fn message(&mut self, message: Message) -> &mut Self;
    fn pause(&mut self, pause: Pause) -> &mut Self;
    fn play_url(&mut self, url: &str) -> &mut Self;
    fn play(&mut self, play: Play) -> &mut Self;
    fn rasa(&mut self, rasa: Rasa) -> &mut Self;
    fn redirect(&mut self, redirect: Redirect) -> &mut Self;
    fn redirect_url(&mut self, url: &str) -> &mut Self;
    fn say_text(&mut self, text: &str) -> &mut Self;
    fn say(&mut self, say: Say) -> &mut Self;
    fn sip_declined(&mut self, sip_decline: SipDecline) -> &mut Self;
    fn sip_rec(&mut self, sip_rec: SipRec) -> &mut Self;
    fn sip_refer(&mut self, sip_refer: SipRefer) -> &mut Self;
    fn tag(&mut self, tag: Tag) -> &mut Self;
    fn transcribe(&mut self, transcribe: Transcribe) -> &mut Self;
    fn push(&mut self, verb: Verb) -> &mut Self;
    fn as_ack(&mut self) -> Ack;
    fn as_ack_reply(&mut self) -> WebsocketReply;
    fn as_redirect(&mut self, queue_command: bool) -> WSRedirect;
    fn as_redirect_reply(&mut self, queue_command: bool) -> WebsocketReply;
}

impl WebsocketReply {
    pub fn json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|e| {
            error!("{}", e);
            "Error serializing WebsocketReply".to_string()
        })
    }
}

impl VerbTrait for Verbs {
    fn new(msg_id: &str) -> Verbs {
        Verbs {
            msg_id: msg_id.to_string(),
            data: Vec::new(),
        }
    }

    fn hangup(&mut self) -> &mut Self {
        let hangup: Verb = Hangup::hangup().into();
        self.push(hangup)
    }

    fn hangup_with_reason(&mut self, reason: &str) -> &mut Self {
        let hangup: Verb = Hangup::hangup_with_reason(reason).into();
        self.push(hangup)
    }

    fn say_text(&mut self, text: &str) -> &mut Self {
        let say = Say {
            text: text.to_string(),
            say_loop: Some(1),
            synthesizer: None,
            early_media: Some(false),
        };
        self.push(Verb::Say(say))
    }

    fn say(&mut self, say: Say) -> &mut Self {
        self.push(Verb::Say(say))
    }

    fn play_url(&mut self, url: &str) -> &mut Self {
        let play = Play::new(url);
        self.push(Verb::Play(play))
    }

    fn play(&mut self, play: Play) -> &mut Self {
        self.push(Verb::Play(play))
    }

    fn dial(&mut self, dial: Dial) -> &mut Self {
        self.push(Verb::Dial(dial))
    }

    fn gather(&mut self, gather: Gather) -> &mut Self {
        self.push(Verb::Gather(gather))
    }

    fn conference(&mut self, conference: Conference) -> &mut Self {
        self.push(Verb::Conference(conference))
    }

    fn config(&mut self, config: Config) -> &mut Self {
        self.push(Verb::Config(config))
    }

    fn dequeue(&mut self, config: Dequeue) -> &mut Self {
        self.push(Verb::Dequeue(config))
    }

    fn dialog_flow(&mut self, dialog_flow: DialogFlow) -> &mut Self {
        self.push(Verb::DialogFlow(dialog_flow))
    }

    fn dtmf(&mut self, dtmf: Dtmf) -> &mut Self {
        self.push(Verb::Dtmf(dtmf))
    }

    fn dub(&mut self, dub_data: DubData) -> &mut Self {
        self.push(Verb::Dub(dub_data))
    }

    fn enqueue(&mut self, enqueue: Enqueue) -> &mut Self {
        self.push(Verb::Enqueue(enqueue))
    }

    fn leave(&mut self, leave: Leave) -> &mut Self {
        self.push(Verb::Leave(leave))
    }

    fn lex(&mut self, lex: Lex) -> &mut Self {
        self.push(Verb::Lex(lex))
    }

    fn listen(&mut self, listen: Listen) -> &mut Self {
        self.push(Verb::Listen(listen))
    }

    fn message(&mut self, message: Message) -> &mut Self {
        self.push(Verb::Message(message))
    }

    fn pause(&mut self, pause: Pause) -> &mut Self {
        self.push(Verb::Pause(pause))
    }

    fn rasa(&mut self, rasa: Rasa) -> &mut Self {
        self.push(Verb::Rasa(rasa))
    }

    fn redirect_url(&mut self, url: &str) -> &mut Self {
        self.push(Verb::Redirect(Redirect {
            action_hook: url.to_string(),
        }))
    }
    
    fn redirect(&mut self, redirect: Redirect) -> &mut Self {
        self.push(Verb::Redirect(redirect))
    }

    fn sip_declined(&mut self, sip_decline: SipDecline) -> &mut Self {
        self.push(Verb::SipDecline(sip_decline))
    }

    fn sip_rec(&mut self, sip_rec: SipRec) -> &mut Self {
        self.push(Verb::SipRec(sip_rec))
    }

    fn sip_refer(&mut self, sip_refer: SipRefer) -> &mut Self {
        self.push(Verb::SipRefer(sip_refer))
    }

    fn tag(&mut self, tag: Tag) -> &mut Self {
        self.push(Verb::Tag(tag))
    }

    fn transcribe(&mut self, transcribe: Transcribe) -> &mut Self {
        self.push(Verb::Transcribe(transcribe))
    }

    fn as_ack(&mut self) -> Ack {
        Ack {
            msgid: self.msg_id.to_string(),
            verbs: self.clone(),
        }
    }

    fn as_ack_reply(&mut self) -> WebsocketReply {
        WebsocketReply::Ack(self.as_ack())
    }

    fn as_redirect(&mut self, queue_command: bool) -> WSRedirect {
        WSRedirect {
            queue_command,
            verbs: self.clone(),
        }
    }

    fn as_redirect_reply(&mut self, queue_command: bool) -> WebsocketReply {
        let redirect = self.as_redirect(queue_command);
        WebsocketReply::Command(Command {
            command_type: CommandValue::Redirect(redirect),
        })
    }

    fn push(&mut self, verb: Verb) -> &mut Self {
        self.data.push(verb);
        self
    }
}

impl Into<WebsocketReply> for DubData {
    fn into(self) -> WebsocketReply {
        WebsocketReply::Command(Command {
            command_type: CommandValue::Dub(self.clone()),
        })
    }
}

impl Into<WebsocketReply> for Whisper {
    fn into(self) -> WebsocketReply {
        WebsocketReply::Command(Command {
            command_type: CommandValue::Whisper(self.clone()),
        })
    }
}

impl Into<WebsocketReply> for Record {
    fn into(self) -> WebsocketReply {
        WebsocketReply::Command(Command {
            command_type: CommandValue::Record(self.clone()),
        })
    }
}

impl Into<WebsocketReply> for ListenStatus {
    fn into(self) -> WebsocketReply {
        WebsocketReply::Command(Command {
            command_type: CommandValue::ListenStatus(self.clone()),
        })
    }
}

impl Into<WebsocketReply> for ConferenceHoldStatus {
    fn into(self) -> WebsocketReply {
        WebsocketReply::Command(Command {
            command_type: CommandValue::ConferenceHoldStatus(self.clone()),
        })
    }
}

impl Into<WebsocketReply> for ConferenceMuteStatus {
    fn into(self) -> WebsocketReply {
        WebsocketReply::Command(Command {
            command_type: CommandValue::ConferenceMuteStatus(self.clone()),
        })
    }
}

impl Into<WebsocketReply> for WSRedirect {
    fn into(self) -> WebsocketReply {
        let redirect = self.clone().verbs.as_redirect(self.queue_command);
        WebsocketReply::Command(Command {
            command_type: CommandValue::Redirect(redirect),
        })
    }
}

impl Into<WebsocketReply> for CallStatus {
    fn into(self) -> WebsocketReply {
        WebsocketReply::Command(Command {
            command_type: CommandValue::CallStatus(self.clone()),
        })
    }
}

impl Into<WebsocketReply> for MuteStatus {
    fn into(self) -> WebsocketReply {
        WebsocketReply::Command(Command {
            command_type: CommandValue::MuteStatus(self),
        })
    }
}
impl Into<WebsocketReply> for Ack {
    fn into(self) -> WebsocketReply {
        let ack = self.clone().verbs.as_ack();
        WebsocketReply::Ack(ack)
    }
}

#[test]
fn json() {
    
    // Ack response to initial session:new
    
    let ack: WebsocketReply = Verbs::new("1234")
        .say_text("Welcome to Callable")
        .as_ack_reply();

    //Example Gather Command
    
    let gather_cmd: WebsocketReply = Verbs::new("1234")
        .gather(
            Gather::new("my_action_url")
                .speech_digits()
                .max_digits(1)
                .bargein(true)
                .say(Say::new("Welcome to Callable, press one for Sales"))
                .build(),
        )
        .redirect_url("my_timeout_url")
        .as_redirect(true)
        .into();

    //Example Dial Command
    
    let dial_cmd: WebsocketReply = Verbs::new("1234")
        .say_text("Putting you through now...")
        .dial(Dial::new(
            "+441234567890",
            vec![
                Phone::new("+441234567890").into(),
                Sip::from_parts("+441234567890", "1.1.1.1.", 5060).into(),
                User::from_parts("my-user", "callable.io").into(),
            ],
        ))
        .as_redirect(true)
        .into();

}
