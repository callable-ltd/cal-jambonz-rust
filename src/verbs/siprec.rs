    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct SipRec {
        pub action: SipRecAction,

        pub siprec_server_url: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub recording_id: Option<String>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub enum SipRecAction {
        StartCallRecording,
        StopCallRecording,
        PauseCallRecording,
        ResumeCallRecording,
    }
