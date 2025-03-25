use crate::{Play, Say, Verb};

#[derive(Clone)]
pub struct Builder {
    verbs: Vec<Verb>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder { verbs: Vec::new() }
    }

    pub fn say(&mut self, text: &str) -> &mut Self {
        let say = Say {
            text: text.to_string(),
            say_loop: Some(1),
            synthesizer: None,
            early_media: Some(false),
        };
        self.verbs.push(Verb::Say(say));
        self
    }

    pub fn play(&mut self, text: &str) -> &mut Self {
        let play = Play {
            url: text.to_string(),
            early_media: Some(false),
            play_loop: Some(1),
            seek_offset: None,
            timeout_secs: None,
            action_hook: None,
        };
        self.verbs.push(Verb::Play(play));
        self
    }

    pub fn build(&mut self) -> Vec<Verb> {
        self.verbs.clone()
    }
}
