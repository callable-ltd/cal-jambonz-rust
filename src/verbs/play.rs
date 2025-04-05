use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Play {
   
    pub url: String,
   
    #[serde(rename = "loop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_loop: Option<u8>,
   
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_media: Option<bool>,
   
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<u8>,
   
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seek_offset: Option<u16>,
   
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_hook: Option<String>,
}

impl Into<Verb> for Play {
    fn into(self) -> Verb {
        Verb::Play(self)
    }
}

impl Into<Vec<Verb>> for Play {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}

impl Play {
    pub fn new(url: String) -> Self {
        Play {
            url,
            action_hook: None,
            play_loop: None,
            early_media: None,
            timeout_secs: None,
            seek_offset: None,
        }
    }

    pub fn action_hook(&mut self, action_hook: Option<String>) -> &mut Play {
        self.action_hook = action_hook;
        self
    }

    pub fn play_loop(&mut self, play_loop: Option<u8>) -> &mut Play {
        self.play_loop = play_loop;
        self
    }

    pub fn early_media(&mut self, early_media: Option<bool>) -> &mut Play {
        self.early_media = early_media;
        self
    }

    pub fn timeout_secs(&mut self, timeout_secs: Option<u8>) -> &mut Play {
        self.timeout_secs = timeout_secs;
        self
    }

    pub fn seek_offset(&mut self, seek_offset: Option<u16>) -> &mut Play {
        self.seek_offset = seek_offset;
        self
    }
    
    
}
