use std::collections::HashMap;
use serde::{Deserialize, Serialize};

//todo move this to an enum where we can have a subset on languages, genders and voices

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Synthesizer {
   
    pub vendor: String,

    pub language: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
}

impl Synthesizer {
    pub fn new(vendor:&str, language: &str) -> Synthesizer {
        Synthesizer {
            vendor: vendor.to_string(),
            language: language.to_string(),
            voice: None,
            gender: None,
        }
    }
    
   pub fn voice(&mut self, voice:&str) -> &mut  Synthesizer {
       self.voice = Some(voice.to_string());
       self
   }
    
    pub fn gender(&mut self, gender:&str) -> &mut Synthesizer {
        self.gender = Some(gender.to_string());
        self
    }
    
}
