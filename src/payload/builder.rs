use crate::conference::Conference;
use crate::config::Config;
use crate::dial::Dial;
use crate::gather::Gather;
use crate::hangup::Hangup;
use crate::play::Play;
use crate::say::Say;
use crate::verb::Verb;
use crate::ws::{Ack, Command, CommandValue, SessionNew, Verbs, WSRedirect, WebsocketReply};
use log::error;
use crate::dequeue::Dequeue;
use crate::dialogflow::DialogFlow;
use crate::dtmf::Dtmf;
use crate::dub::{Dub, DubData};
use crate::enqueue::Enqueue;
use crate::leave::Leave;
use crate::lex::Lex;
use crate::listen::Listen;
use crate::message::Message;
use crate::pause::Pause;
use crate::rasa::Rasa;
use crate::redirect::Redirect;
use crate::sipdecline::SipDecline;
use crate::siprec::SipRec;
use crate::siprefer::SipRefer;
use crate::tag::Tag;
use crate::transcribe::Transcribe;

trait VerbTrait {
    fn new() -> Self;
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
    fn say_text(&mut self, text: &str) -> &mut Self;
    fn say(&mut self, say: Say) -> &mut Self;
    fn sip_declined(&mut self, sip_decline: SipDecline) -> &mut Self;
    fn sip_rec(&mut self, sip_rec: SipRec) -> &mut Self;
    fn sip_refer(&mut self, sip_refer: SipRefer) -> &mut Self;
    fn tag(&mut self, tag: Tag) -> &mut Self;
    fn transcribe(&mut self, transcribe: Transcribe) -> &mut Self;
    fn push(&mut self, verb: Verb) -> &mut Self;
    fn to_ack(&mut self, msg_id: &str) -> Ack;
    fn to_redirect(&mut self, queue_command: bool) -> WSRedirect;
}

impl WebsocketReply {
    pub fn json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|e| {
            error!("{}", e);
            "Error serializing WebsocketReply".to_string()
        })
    }
}

// impl SessionNew {
//     pub fn ack(&self, ) -> Verbs {
//         Verbs::new(&self.msgid)
//     }
// }

impl VerbTrait for Verbs {
    fn new() -> Verbs {
        Verbs { data: Vec::new() }
    }

    fn hangup(&mut self) ->&mut Self {
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

    fn to_ack(&mut self, msgid: &str) -> Ack {
        Ack {
            msgid: msgid.to_string(),
            verbs: self.clone(),
        }
    }
    fn to_redirect(&mut self, queue_command: bool) -> WSRedirect {
        WSRedirect {
            queue_command,
            verbs: self.clone(),
        }
    }
    fn push(&mut self, verb: Verb) -> &mut Self {
        self.data.push(verb);
        self
    }

}

impl Command {
    pub fn new() -> Command {
        Command {
            command_type: CommandValue::Redirect(WSRedirect {
                queue_command: false,
                verbs: Verbs::new(),
            }),
        }
    }

    pub fn redirect(&mut self, queue_command: bool) -> WSRedirect {
        WSRedirect {
            queue_command,
            verbs: Verbs::new(),
        }
    }
}

// impl WSRedirect {
//    pub fn queue(&mut self, queue_command: bool) -> &mut WSRedirect {
//         self.queue_command = queue_command;
//         self
//     }
//
//     pub  fn verbs(&mut self, verbs: Vec<Verb>) -> WSRedirect {
//         self.verbs = Verbs { data: verbs };
//         self.clone()
//     }
//
//     pub  fn build(&mut self) -> WebsocketReply {
//         let redirect = self.verbs.to_redirect(self.queue_command);
//         WebsocketReply::Command(Command {
//             command_type: CommandValue::Redirect(redirect),
//         })
//     }
// }


impl Ack {
    pub fn new(msg_id: &str) -> Ack {
        Ack {
            msgid: msg_id.to_string(),
            verbs: Verbs::new(),
        }
    }
    pub fn build(&mut self) -> WebsocketReply {
        let ack = self.verbs.to_ack(&self.msgid);
        WebsocketReply::Ack(ack)
    }
}

#[test]
fn json() {

    let ack = Verbs::new()
        .say_text("Welcome to Callable")
        .to_ack("1234")
        .build()
        .json();

    println!("{:#?}", ack);

    let cmd = Verbs::new()
        .say_text("Welcome to Callable")
        .to_ack("1234")
        .build()
        .json();

    println!("{:#?}", cmd);
}
