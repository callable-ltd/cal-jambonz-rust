use crate::{Ack, Play, Say, Verb, WebsocketReply};

impl Ack {
    pub fn new(msg_id: &str) -> Ack {
        Ack {
            msgid: msg_id.to_string(),
            data: Some(Vec::new()),
        }
    }

    pub fn say(&mut self, text: &str) -> Self {
        let say = Say {
            text: text.to_string(),
            say_loop: Some(1),
            synthesizer: None,
            early_media: Some(false),
        };
        self.data.get_or_insert_default().push(Verb::Say(say));
      
        Ack {
            msgid: self.msgid.to_string(),
            data: self.data.clone(),
        }
    }

    pub fn play(&mut self, text: &str) -> Self {
        let play = Play {
            url: text.to_string(),
            early_media: Some(false),
            play_loop: Some(1),
            seek_offset: None,
            timeout_secs: None,
            action_hook: None,
        };
        
        self.data.get_or_insert_default().push(Verb::Play(play));
        
        Ack {
            msgid: self.msgid.to_string(),
            data: self.data.clone(),
        }
    }
    
    pub fn ack(self) -> WebsocketReply {
        WebsocketReply::Ack(self)
    }
}
