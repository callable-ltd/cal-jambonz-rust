use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Dtmf {
    pub dtmf: String,
    pub duration: Option<u8>,
}

impl Dtmf {
    pub fn new(dtmf: String) -> Dtmf {
        Dtmf {
            dtmf: dtmf.to_string(),
            duration: None,
        }
    }

    pub fn duration(&mut self, duration: Option<u8>) -> &mut Dtmf {
        self.duration = duration;
        self
    }
}

impl Into<Verb> for Dtmf {
    fn into(self) -> Verb {
        Verb::Dtmf(self)
    }
}

impl Into<Vec<Verb>> for Dtmf {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}
