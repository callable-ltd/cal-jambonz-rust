use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SipRec {
    pub action: SipRecAction,

    pub siprec_server_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_id: Option<String>,
}

impl SipRec {
    pub fn new(action: SipRecAction, siprec_server_url: &str) -> SipRec {
        SipRec {
            action,
            siprec_server_url: siprec_server_url.to_string(),
            recording_id: None,
        }
    }
    pub fn siprec_server_url(&mut self, siprec_server_url: &str) -> &mut SipRec {
        self.recording_id = Some(siprec_server_url.to_string());
        self
    }
    pub fn set_recording_id(&mut self, recording_id: &str) -> &mut SipRec {
        self.recording_id = Some(recording_id.to_string());
        self
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SipRecAction {
    StartCallRecording,
    StopCallRecording,
    PauseCallRecording,
    ResumeCallRecording,
}
