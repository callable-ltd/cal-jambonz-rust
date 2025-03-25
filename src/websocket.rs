use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::iter::Map;

use crate::{Conference, Play, Say, Verb};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebsocketReply {
    Ack(Ack),
    Command(Command),
}

#[derive(Serialize, Deserialize)]
pub struct Ack {
    msgid: String,
    data: Option<Vec<Verb>>,
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    #[serde(flatten)]
    command_type: CommandValue,
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
    queue_command: bool,
    data: Option<Vec<Verb>>,
}

#[derive(Serialize, Deserialize)]
pub struct CallStatus {
    data: CallStatusData,
}

#[derive(Serialize, Deserialize)]
pub struct CallStatusData {
    call_status: CallStatusValue,
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
    data: MuteStatusData,
}
#[derive(Serialize, Deserialize)]
pub struct MuteStatusData {
    mute_status: MuteStatusValue,
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
    data: ConferenceMuteStatusData,
}
#[derive(Serialize, Deserialize)]
pub struct ConferenceMuteStatusData {
    conf_mute_status: ConferenceMuteStatusValue,
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
    data: ConferenceHoldStatusData,
}
#[derive(Serialize, Deserialize)]
pub struct ConferenceHoldStatusData {
    conf_hold_status: ConferenceHoldStatusValue,
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
    data: ListenStatusData,
}
#[derive(Serialize, Deserialize)]
pub struct ListenStatusData {
    listen_status: ListenStatusValue,
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
    data: RecordData,
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
    recording_id: String,
    #[serde(rename = "siprecServerURL")]
    sip_rec_server_url: String,
    headers: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub struct StopCallRecording {}
#[derive(Serialize, Deserialize)]
pub struct PauseCallRecording {}
#[derive(Serialize, Deserialize)]
pub struct ResumeCallRecording {}

#[derive(Serialize, Deserialize)]
pub struct Whisper {
    data: Vec<PlaySay>,
}

#[derive(Serialize, Deserialize)]
pub struct SipRequest {
    data: SipRequestData,
}
#[derive(Serialize, Deserialize)]
pub struct SipRequestData {
    method: SipMethod,
    content_type: Option<String>,
    content: String,
    headers: Option<HashMap<String, String>>,
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
    data: Vec<DubData>,
}

#[derive(Serialize, Deserialize)]
pub struct DubData {
    verb: String,
    action: DubTrack,
    track: String,
    play: Option<String>,
    say: Option<String>,
    loop_count: Option<u8>,
    gain: Option<String>,
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
