use serde::{Deserialize, Serialize};
use crate::verbs::verb::Verb;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Conference {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beep: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_hook: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enter_hook: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_muted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_participants: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_conference_on_exit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_conference_on_enter: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_hook: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub status_events: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_hook: Option<String>,
}

impl Into<Verb> for Conference {
    fn into(self) -> Verb {
        Verb::Conference(self)
    }
}

impl Into<Vec<Verb>> for Conference {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}

impl Conference {
    pub fn new(name: &str) -> Conference {
        Conference {
            name: name.to_string(),
            beep: None,
            action_hook: None,
            enter_hook: None,
            join_muted: None,
            max_participants: None,
            end_conference_on_exit: None,
            start_conference_on_enter: None,
            status_hook: None,
            status_events: vec![],
            wait_hook: None,
        }
    }

    pub fn beep(&mut self, beep: bool) -> &mut Conference {
        self.beep = Some(beep);
        self
    }

    pub fn action_hook(&mut self, action_hook: &str) -> &mut Conference {
        self.action_hook = Some(action_hook.to_string());
        self
    }

    pub fn enter_hook(&mut self, enter_hook: &str) -> &mut Conference {
        self.enter_hook = Some(enter_hook.to_string());
        self
    }

    pub fn join_muted(&mut self, join_muted: bool) -> &mut Conference {
        self.join_muted = Some(join_muted);
        self
    }

    pub fn max_participants(&mut self, max_participants: u8) -> &mut Conference {
        self.max_participants = Some(max_participants);
        self
    }

    pub fn end_conference_on_exit(&mut self, end_conference_on_exit: bool) -> &mut Conference {
        self.end_conference_on_exit = Some(end_conference_on_exit);
        self
    }

    pub fn start_conference_on_enter(
        &mut self,
        start_conference_on_enter: bool,
    ) -> &mut Conference {
        self.start_conference_on_enter = Some(start_conference_on_enter);
        self
    }

    pub fn status_hook(&mut self, status_hook: &str) -> &mut Conference {
        self.status_hook = Some(status_hook.to_string());
        self
    }

    pub fn wait_hook(&mut self, wait_hook: &str) -> &mut Conference {
        self.wait_hook = Some(wait_hook.to_string());
        self
    }

    pub fn replace_status_events(&mut self, status_events: Vec<&str>) -> &mut Conference {
        self.status_events = status_events.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn add_status_events(&mut self, status_events: Vec<&str>) -> &mut Conference {
        for ev in status_events {
            self.status_events.push(ev.to_string());
        }
        self
    }

    pub fn add_status_event(&mut self, status_event: &str) -> &mut Conference {
        self.status_events.push(status_event.to_string());
        self
    }
}
