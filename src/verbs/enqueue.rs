use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Enqueue {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_hook: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_hook: Option<String>,
}

impl Enqueue {
    pub fn new(name: &str) -> Enqueue {
        Enqueue {
            name: name.to_string(),
            priority: None,
            action_hook: None,
            wait_hook: None,
        }
    }
    
    pub fn priority(&mut self, priority: u16) -> &mut Enqueue{
        self.priority = Some(priority);
        self
    }
    
    pub fn action_hook(&mut self, hook: String) -> &mut Enqueue {
        self.action_hook = Some(hook);
        self
    }
    
    pub fn wait_hook(&mut self, hook: String) -> &mut Enqueue {
        self.wait_hook = Some(hook);
        self
    }
    
}

impl Into<Verb> for Enqueue {
    fn into(self) -> Verb {
        Verb::Enqueue(self)
    }
}

impl Into<Vec<Verb>> for Enqueue {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}
