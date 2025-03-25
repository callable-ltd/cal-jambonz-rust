use crate::websocket::Redirect;
use crate::{Ack, Command, CommandValue, Play, Say, Synthesizer, Verb, Verbs, WebsocketReply};

trait VerbTrait {
    fn new() -> Verbs;
    fn say_text(&mut self, text: &str) -> Self;
    fn say(&mut self, say: Say) -> Self;
    fn play_url(&mut self, url: &str) -> Self;
    fn play(&mut self, play: Play) -> Self;
    fn push(&mut self, verb: Verb) -> Self;
    fn to_ack(&mut self, msgid: &str) -> Ack;
    fn to_redirect(&mut self, queue_command: bool) -> Redirect;
}
impl VerbTrait for Verbs {
    fn new() -> Verbs {
        Verbs { data: Vec::new() }
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
    fn to_redirect(&mut self, queue_command: bool) -> Redirect {
        Redirect {
            queue_command,
            verbs: self.clone(),
        }
    }
}

impl Command {
    pub fn new() -> Command {
        Command {
            command_type: CommandValue::Redirect(Redirect {
                queue_command: false,
                verbs: Verbs::new(),
            }),
        }
    }

    pub fn redirect(&mut self) -> Redirect {
        Redirect {
            queue_command: false,
            verbs: Verbs::new(),
        }
    }
}

impl Redirect {
    pub fn queue(&mut self, queue_command: bool) -> &mut Redirect {
        self.queue_command = queue_command;
        self
    }

    pub fn play(&mut self, play: Play) -> Redirect {
        self.verbs.play(play);
        self.clone()
    }

    pub fn play_url(&mut self, url: &str) -> Redirect {
        self.verbs.play_url(url);
        self.clone()
    }

    pub fn say(&mut self, say: Say) -> Redirect {
        self.verbs.say(say);
        self.clone()
    }

    pub fn say_text(&mut self, text: &str) -> Redirect {
        self.verbs.say_text(text);
        self.clone()
    }

    pub fn push(&mut self, verb: Verb) -> Redirect {
        self.verbs.push(verb);
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
        .redirect()
        .queue(true)
        .say_text("Here is another message.")
        .say_text("And Another one")
        .build()
        .json();

    println!("{:#?}", cmd);
}
