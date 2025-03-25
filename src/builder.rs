use crate::{Ack, Play, Say, Synthesizer, Verb, WebsocketReply};

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
