use crate::payload::rest::{InitialRequest, Request};
use crate::shared::shared::SIPStatus;
use crate::verbs::dub::DubData;
use crate::verbs::play_say::PlaySay;
use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
pub struct Verbs {
    #[serde(skip)]
    pub msg_id: String,

    pub data: Vec<Verb>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum WebsocketRequest {
    #[serde(rename = "session:new")]
    SessionNew(SessionNew),
    #[serde(rename = "session:redirect")]
    SessionRedirect(SessionRedirect),
    #[serde(rename = "session:reconnect")]
    SessionReconnect(SessionReconnect),
    #[serde(rename = "call:status")]
    CallStatus(SessionCallStatus),
    #[serde(rename = "verb:hook")]
    VerbHook(SessionVerbHook),

    #[serde(rename = "session:record")]
    RecordingRequest(SessionRecording),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionRecording {
   pub call_sid: String,
    pub account_sid: String,
    pub application_sid: String,
    pub from: String,
    pub to: String,
    pub caller_id: String,
    pub call_id: String,
    pub call_status: SessionCallStatusEnum,
    pub sip_status: SIPStatus,
    pub sip_reason: String,
    pub local_sip_address: String,
    pub public_ip: String,
    pub sbc_callid: String,
    pub parent_call_sid: Option<String>,
    pub mix_type: String,
    pub sample_rate: u16,

    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionNew {
    pub msgid: String,
    pub call_sid: String,
    pub b3: Option<String>,
    pub data: InitialRequest,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionRedirect {
    pub msgid: String,
    pub call_sid: String,
    pub b3: Option<String>,
    pub hook: String,
    pub data: Request,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionReconnect {
    pub msgid: String,
    pub call_sid: String,
    pub b3: Option<String>,
    pub hook: String,
    pub data: Request,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionCallStatus {
    pub msgid: String,
    pub call_sid: String,
    pub b3: Option<String>,
    pub data: Request,
    #[serde(rename = "data.call_status")]
    pub call_status: SessionCallStatusEnum,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct SessionVerbHook {
    pub msgid: String,
    pub call_sid: String,
    pub b3: Option<String>,
    pub hook: String,
    pub data: Request,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionVerbStatus {
    pub msgid: String,
    pub call_sid: String,
    pub b3: Option<String>,
    pub hook: String,
    pub data: HashMap<String, String>,
    #[serde(rename = "data.id")]
    pub data_id: String,
    #[serde(rename = "data.verb")]
    pub data_verb: String,
    #[serde(rename = "data.status")]
    pub data_status: DataStatus,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DataStatus {
    Begin,
    End,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SessionCallStatusEnum {
    Trying,
    Ringing,
    #[serde(rename = "early-media")]
    EarlyMedia,
    #[serde(rename = "in-progress")]
    InProgress,
    Completed,
    Failed,
    #[serde(rename = "no-answer")]
    NoAnswer,
    Busy,
    Queued,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum WebsocketReply {
    Ack(Ack),
    Command(Command),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ack {
    pub msgid: String,
    #[serde(flatten)]
    pub verbs: Verbs,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Command {
    #[serde(flatten)]
    pub command_type: CommandValue,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum CommandType {
    Redirect,
    CallStatus,
    MuteStatus,
    ConferenceMuteStatus,
    ConferenceHoldStatus,
    ListenStatus,
    Record,
    Whisper,
    Dub,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "command")]
#[serde(rename_all = "camelCase")]
pub enum CommandValue {
    #[serde(rename = "redirect")]
    Redirect(WSRedirect),
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
    Dub(DubData),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct WSRedirect {
    #[serde(rename = "queueCommand")]
    pub queue_command: bool,
    #[serde(flatten)]
    pub verbs: Verbs,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CallStatus {
    pub queue_command: bool,
    pub data: CallStatusData,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CallStatusData {
    pub call_status: CallStatusValue,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum CallStatusValue {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "no-answer")]
    NoAnswer,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MuteStatus {
    pub queue_command: bool,
    pub data: MuteStatusData,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct MuteStatusData {
    pub mute_status: MuteStatusValue,
}
#[derive(Serialize, Deserialize, Clone)]
pub enum MuteStatusValue {
    #[serde(rename = "mute")]
    Mute,
    #[serde(rename = "unmute")]
    UnMute,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ConferenceMuteStatus {
    pub queue_command: bool,
    pub data: ConferenceMuteStatusData,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ConferenceMuteStatusData {
    pub conf_mute_status: ConferenceMuteStatusValue,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ConferenceMuteStatusValue {
    #[serde(rename = "mute")]
    Mute,
    #[serde(rename = "unmute")]
    UnMute,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ConferenceHoldStatus {
    pub queue_command: bool,
    pub data: ConferenceHoldStatusData,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ConferenceHoldStatusData {
    pub conf_hold_status: ConferenceHoldStatusValue,
}
#[derive(Serialize, Deserialize, Clone)]
pub enum ConferenceHoldStatusValue {
    #[serde(rename = "hold")]
    Hold,
    #[serde(rename = "unhold")]
    UnHold,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ListenStatus {
    pub queue_command: bool,
    pub data: ListenStatusData,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ListenStatusData {
    pub listen_status: ListenStatusValue,
}
#[derive(Serialize, Deserialize, Clone)]
pub enum ListenStatusValue {
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "resume")]
    Resume,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Record {
    pub queue_command: bool,
    pub data: RecordData,
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "action")]
#[serde(rename_all = "camelCase")]
pub enum RecordData {
    StartCallRecording(StartCallRecording),
    StopCallRecording(StopCallRecording),
    PauseCallRecording(PauseCallRecording),
    ResumeCallRecording(ResumeCallRecording),
}
#[derive(Serialize, Deserialize, Clone)]
pub struct StartCallRecording {
    #[serde(rename = "recordingID")]
    pub recording_id: String,
    #[serde(rename = "siprecServerURL")]
    pub sip_rec_server_url: String,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StopCallRecording {}
#[derive(Serialize, Deserialize, Clone)]
pub struct PauseCallRecording {}
#[derive(Serialize, Deserialize, Clone)]
pub struct ResumeCallRecording {}

#[derive(Serialize, Deserialize, Clone)]
pub struct Whisper {
    pub queue_command: bool,
    pub data: Vec<PlaySay>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SipRequest {
    pub queue_command: bool,
    pub data: SipRequestData,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct SipRequestData {
    pub method: SipMethod,
    pub content_type: Option<String>,
    pub content: String,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SipMethod {
    INFO,
    NOTIFY,
    MESSAGE,
}
