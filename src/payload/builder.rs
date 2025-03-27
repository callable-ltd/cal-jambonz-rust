use crate::dial::Dial;
use crate::gather::Gather;
use crate::hangup::Hangup;
use crate::play::Play;
use crate::say::Say;
use crate::verb::Verb;
use crate::ws::{Ack, Command, CommandValue, SessionNew, Verbs, WSRedirect, WebsocketReply};
use log::error;

trait VerbTrait {
    fn new() -> Verbs;
    fn hangup(&mut self) -> Self;
    fn hangup_with_reason(&mut self, reason: &str) -> Self;
    fn say_text(&mut self, text: &str) -> Self;
    fn say(&mut self, say: Say) -> Self;
    fn play_url(&mut self, url: &str) -> Self;
    fn play(&mut self, play: Play) -> Self;
    fn dial(&mut self, dial: Dial) -> Self;
    fn gather(&mut self, dial: Gather) -> Self;
    fn push(&mut self, verb: Verb) -> Self;
    fn to_ack(&mut self, msgid: &str) -> Ack;
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

impl SessionNew {
    pub fn builder(&self) -> Ack {
        Ack::new(&self.msgid)
    }
}

impl VerbTrait for Verbs {
    fn new() -> Verbs {
        Verbs { data: Vec::new() }
    }

    fn hangup(&mut self) -> Self {
        let hangup: Verb = Hangup::hangup().into();
        self.push(hangup)
    }

    fn hangup_with_reason(&mut self, reason: &str) -> Self {
        let hangup: Verb = Hangup::hangup_with_reason(reason).into();
        self.push(hangup)
    }

    fn say_text(&mut self, text: &str) -> Self {
        let say = Say {
            text: text.to_string(),
            say_loop: Some(1),
            synthesizer: None,
            early_media: Some(false),
        };
        self.push(Verb::Say(say))
    }

    fn say(&mut self, say: Say) -> Self {
        self.push(Verb::Say(say))
    }

    fn play_url(&mut self, url: &str) -> Self {
        let play = Play::new(url, None);
        self.push(Verb::Play(play))
    }

    fn play(&mut self, play: Play) -> Self {
        self.push(Verb::Play(play))
    }

    fn dial(&mut self, dial: Dial) -> Self {
        self.push(Verb::Dial(dial))
    }

    fn gather(&mut self, gather: Gather) -> Self {
        self.push(Verb::Gather(gather))
    }

    fn push(&mut self, verb: Verb) -> Self {
        self.data.push(verb);
        self.clone()
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

impl WSRedirect {
    pub fn queue(&mut self, queue_command: bool) -> &mut WSRedirect {
        self.queue_command = queue_command;
        self
    }

    pub fn play(&mut self, play: Play) -> WSRedirect {
        self.verbs.play(play);
        self.clone()
    }

    pub fn play_url(&mut self, url: &str) -> WSRedirect {
        self.verbs.play_url(url);
        self.clone()
    }

    pub fn say(&mut self, say: Say) -> WSRedirect {
        self.verbs.say(say);
        self.clone()
    }

    pub fn say_text(&mut self, text: &str) -> WSRedirect {
        self.verbs.say_text(text);
        self.clone()
    }

    pub fn hangup(&mut self) -> WSRedirect {
        self.verbs.hangup();
        self.clone()
    }

    pub fn hangup_with_reason(&mut self, reason: &str) -> WSRedirect {
        self.verbs.hangup_with_reason(reason);
        self.clone()
    }

    pub fn push(&mut self, verb: Verb) -> WSRedirect {
        self.verbs.push(verb);
        self.clone()
    }

    pub fn verbs(&mut self, verbs: Vec<Verb>) -> WSRedirect {
        self.verbs = Verbs { data: verbs };
        self.clone()
    }

    pub fn build(&mut self) -> WebsocketReply {
        let redirect = self.verbs.to_redirect(self.queue_command);
        WebsocketReply::Command(Command {
            command_type: CommandValue::Redirect(redirect),
        })
    }
}

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

    pub fn play(&mut self, play: Play) -> Ack {
        self.verbs.play(play);
        self.clone()
    }

    pub fn play_url(&mut self, url: &str) -> Ack {
        self.verbs.play_url(url);
        self.clone()
    }

    pub fn say(&mut self, say: Say) -> Ack {
        self.verbs.say(say);
        self.clone()
    }

    pub fn say_text(&mut self, text: &str) -> Ack {
        self.verbs.say_text(text);
        self.clone()
    }

    pub fn hangup(&mut self) -> Ack {
        self.verbs.hangup();
        self.clone()
    }

    pub fn hangup_with_reason(&mut self, reason: &str) -> Ack {
        self.verbs.hangup_with_reason(reason);
        self.clone()
    }

    pub fn push(&mut self, verb: Verb) -> Ack {
        self.verbs.push(verb);
        self.clone()
    }
}

#[test]
fn json() {
    let ack = Ack::new("1234")
        .say_text("Welcome to Callable")
        .build()
        .json();

    println!("{:#?}", ack);

    let cmd = Command::new()
        .redirect(true)
        .say_text("Here is another message.")
        .say_text("And Another one")
        .build()
        .json();

    println!("{:#?}", cmd);
}
