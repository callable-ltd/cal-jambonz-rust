use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Dub {
    #[serde(rename = "queueCommand")]
    pub queue_command: bool,
    #[serde(flatten)]
    pub data: Vec<DubData>,
}

impl Dub {
    pub fn new(queue_command: bool) -> Dub {
        Dub {
            queue_command,
            data: Vec::new(),
        }
    }

    pub fn with_action(queue_command: bool, data: Vec<DubData>) -> Dub {
        Dub {
            queue_command,
            data,
        }
    }

    pub fn add(&mut self, data: DubData) -> &mut Dub {
        self.data.push(data);
        self
    }
    
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DubData {

    pub verb: String,

    pub action: DubTrack,

    pub track: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub play: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub say: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_count: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gain: Option<String>,
}

impl DubData {
    pub fn new(action: DubTrack, track: String) -> DubData {
        DubData {
            action,
            verb: "dub".to_string(),
            track,
            play: None,
            say: None,
            loop_count: None,
            gain: None,
        }
    }

    pub fn play(&mut self, play: Option<String>) -> &mut DubData {
        self.play = play;
        self
    }

    pub fn say(&mut self, say: Option<String>) -> &mut DubData {
        self.say = say;
        self
    }

    pub fn loop_count(&mut self, loop_count: Option<u8>) -> &mut DubData {
        self.loop_count = loop_count;
        self
    }

    pub fn gain(&mut self, gain: Option<String>) -> &mut DubData {
        self.gain = gain;
        self
    }
}

impl Into<Verb> for DubData {
    fn into(self) -> Verb {
        Verb::Dub(self)
    }
}

impl Into<Vec<Verb>> for DubData {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}


#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DubTrack {
    AddTrack,
    RemoveTrack,
    SilenceTrack,
    PlayOnTrack,
    SayOnTrack,
}
