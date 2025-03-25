use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{Play, Say, Verb};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum WebsocketReply {
    Ack(Ack),
    Command(Command),
}

impl WebsocketReply {
    pub fn json(&self) -> String {
        serde_json::to_string(self)
            .unwrap_or_else(|e| "Error serializing WebsocketReply".to_string())
    }
}
#[derive(Serialize, Deserialize)]
pub struct Ack {
    pub msgid: String,
    pub data: Option<Vec<Verb>>,
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    #[serde(flatten)]
    pub command_type: CommandValue,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "command")]
#[serde(rename_all = "camelCase")]
pub enum CommandValue {
    #[serde(rename = "redirect")]
    Redirect(Redirect),
    #[serde(rename = "call:status")]
    CallStatus(CallStatus),
    #[serde(rename = "mute:status")]
    MuteStatus(MuteStatus),
    #[serde(rename = "conf:mute-status")]
    ConferenceMuteStatus(ConferenceMuteStatus),
    #[serde(rename = "conf:hold-status")]
    ConferenceHoldStatus(ConferenceHoldStatus),
    #[serde(rename = "listen:status")]
    ListenStatus(ListenStatus),
    #[serde(rename = "record")]
    Record(Record),
    #[serde(rename = "whisper")]
    Whisper(Whisper),
    #[serde(rename = "dub")]
    Dub(Whisper),
}

#[derive(Serialize, Deserialize)]
pub struct Redirect {
    #[serde(rename = "queueCommand")]
    pub queue_command: bool,
    pub data: Option<Vec<Verb>>,
}

#[derive(Serialize, Deserialize)]
pub struct CallStatus {
    pub data: CallStatusData,
}

#[derive(Serialize, Deserialize)]
pub struct CallStatusData {
    pub call_status: CallStatusValue,
}

#[derive(Serialize, Deserialize)]
pub enum CallStatusValue {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "no-answer")]
    NoAnswer,
}

#[derive(Serialize, Deserialize)]
pub struct MuteStatus {
    pub data: MuteStatusData,
}
#[derive(Serialize, Deserialize)]
pub struct MuteStatusData {
    pub mute_status: MuteStatusValue,
}
#[derive(Serialize, Deserialize)]
pub enum MuteStatusValue {
    #[serde(rename = "mute")]
    Mute,
    #[serde(rename = "unmute")]
    UnMute,
}
#[derive(Serialize, Deserialize)]
pub struct ConferenceMuteStatus {
    pub data: ConferenceMuteStatusData,
}
#[derive(Serialize, Deserialize)]
pub struct ConferenceMuteStatusData {
    pub conf_mute_status: ConferenceMuteStatusValue,
}

#[derive(Serialize, Deserialize)]
pub enum ConferenceMuteStatusValue {
    #[serde(rename = "mute")]
    Mute,
    #[serde(rename = "unmute")]
    UnMute,
}
#[derive(Serialize, Deserialize)]
pub struct ConferenceHoldStatus {
    pub data: ConferenceHoldStatusData,
}
#[derive(Serialize, Deserialize)]
pub struct ConferenceHoldStatusData {
    pub conf_hold_status: ConferenceHoldStatusValue,
}
#[derive(Serialize, Deserialize)]
pub enum ConferenceHoldStatusValue {
    #[serde(rename = "hold")]
    Hold,
    #[serde(rename = "unhold")]
    UnHold,
}
#[derive(Serialize, Deserialize)]
pub struct ListenStatus {
    pub data: ListenStatusData,
}
#[derive(Serialize, Deserialize)]
pub struct ListenStatusData {
    pub listen_status: ListenStatusValue,
}
#[derive(Serialize, Deserialize)]
pub enum ListenStatusValue {
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "resume")]
    Resume,
}
#[derive(Serialize, Deserialize)]
pub struct Record {
    pub data: RecordData,
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
#[serde(rename_all = "camelCase")]
pub enum RecordData {
    StartCallRecording(StartCallRecording),
    StopCallRecording(StopCallRecording),
    PauseCallRecording(PauseCallRecording),
    ResumeCallRecording(ResumeCallRecording),
}
#[derive(Serialize, Deserialize)]
pub struct StartCallRecording {
    #[serde(rename = "recordingID")]
    pub recording_id: String,
    #[serde(rename = "siprecServerURL")]
    pub sip_rec_server_url: String,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub struct StopCallRecording {}
#[derive(Serialize, Deserialize)]
pub struct PauseCallRecording {}
#[derive(Serialize, Deserialize)]
pub struct ResumeCallRecording {}

#[derive(Serialize, Deserialize)]
pub struct Whisper {
    pub data: Vec<PlaySay>,
}

#[derive(Serialize, Deserialize)]
pub struct SipRequest {
    pub data: SipRequestData,
}
#[derive(Serialize, Deserialize)]
pub struct SipRequestData {
    pub method: SipMethod,
    pub content_type: Option<String>,
    pub content: String,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub enum SipMethod {
    INFO,
    NOTIFY,
    MESSAGE,
}

#[derive(Serialize, Deserialize)]
pub struct Dub {
    #[serde(flatten)]
    pub data: Vec<DubData>,
}

#[derive(Serialize, Deserialize)]
pub struct DubData {
    pub verb: String,
    pub action: DubTrack,
    pub track: String,
    pub play: Option<String>,
    pub say: Option<String>,
    pub loop_count: Option<u8>,
    pub gain: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DubTrack {
    AddTrack,
    RemoveTrack,
    SilenceTrack,
    PlayOnTrack,
    SayOnTrack,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "verb")]
#[serde(rename_all = "camelCase")]
pub enum PlaySay {
    Play(Play),
    Say(Say),
}
