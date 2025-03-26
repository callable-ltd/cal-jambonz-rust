mod builder;
mod websocket;

use ip_in_subnet::iface_in_subnet;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use websocket::*;

//noinspection ALL
#[derive(Serialize, Deserialize, strum::IntoStaticStr)]
pub enum TenantType {
    PROXY,
    TRUNK,
    USER,
    TEAMS,
    APPLICATION,
}

#[derive(Serialize, Deserialize, Clone, strum::IntoStaticStr)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "verb")]
pub enum Verb {
    Conference(Conference),
    Config(Config),
    Dequeue(Dequeue),
    Dial(Dial),
    DialogFlow(DialogFlow),
    Dtmf(DTMF),
    Enqueue(Enqueue),
    Gather(Gather),
    Hangup(Hangup),
    Leave(Leave),
    Lex(Lex),
    Listen(Listen),
    Message(Message),
    #[serde(rename = "sip:decline")]
    SipDecline(SipDecline),
    Play(Play),
    Say(Say),
    #[serde(rename = "sip:refer")]
    SipRefer(SipRefer),
    Tag(Tag),
    Transcribe(Transcribe),
}

impl Into<Vec<Verb>> for Verb {
    fn into(self) -> Vec<Verb> {
        vec![self]
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Conference {
    pub name: String,
    pub beep: Option<bool>,
    pub action_hook: Option<String>,
    pub enter_hook: Option<String>,
    pub join_muted: Option<bool>,
    pub max_participants: Option<u8>,
    pub end_conference_on_exit: Option<bool>,
    pub start_conference_on_enter: Option<bool>,
    pub status_hook: Option<String>,
    pub status_events: Vec<String>,
    pub wait_hook: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub amd: Option<bool>,
    pub bargein: Option<BargeIn>,
    pub listen: Option<Listen>,
    pub notify_events: Option<bool>,
    pub on_hold_music: Option<String>,
    pub recognizer: Option<Recognizer>,
    pub reset: Option<Vec<String>>,
    pub record: Option<SipRec>,
    pub sip_request_within_dialog_hook: Option<String>,
    pub synthesizer: Synthesizer,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dequeue {
    pub name: String,
    pub action_hook: Option<String>,
    pub beep: Option<bool>,
    pub timeout: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dial {
    pub action_hook: Option<String>,
    pub amd: Option<bool>,
    pub answer_on_bridge: Option<bool>,
    pub caller_id: Option<String>,
    pub confirm_hook: Option<String>,
    pub dial_music: Option<String>,
    pub dtmf_capture: Option<Vec<String>>,
    pub dtmf_hook: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub listen: Option<Listen>,
    pub refer_hook: Option<String>,
    pub target: Vec<Target>,
    pub time_limit: Option<u16>,
    pub timeout: Option<u8>,
    pub transcribe: Option<Transcribe>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum Target {
    Pstn(Pstn),
    Sip(Sip),
    User(User),
    Teams(Teams),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pstn {
    #[serde(rename = "type")]
    pub target_type: String,
    pub number: String,
    pub confirm_hook: Option<String>,
    pub trunk: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sip {
    #[serde(rename = "type")]
    pub target_type: String,
    pub sip_uri: String,
    pub confirm_hook: Option<String>,
    pub auth: Option<WSAuth>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[serde(rename = "type")]
    pub target_type: String,
    pub name: String,
    pub confirm_hook: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Teams {
    #[serde(rename = "type")]
    pub target_type: String,
    pub number: String,
    pub confirm_hook: Option<String>,
    pub tenant: Option<String>,
    pub voicemail: Option<bool>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Say {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthesizer: Option<Synthesizer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "loop")]
    pub say_loop: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_media: Option<bool>,
}

impl Into<Verb> for Say {
    fn into(self) -> Verb {
        Verb::Say(self)
    }
}

impl Say {
    pub fn new(text: &str) -> Say {
        Say {
            text: text.to_string(),
            say_loop: Some(1),
            synthesizer: None,
            early_media: Some(false),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DialogFlow {
    pub project: String,
    pub lang: String,
    pub credentials: String,
    pub welcome_event: Option<String>,
    pub welcome_event_params: Option<HashMap<String, String>>,
    pub no_input_timeout: Option<u8>,
    pub no_input_event: Option<String>,
    pub pass_dtmf_as_text_input: Option<String>,
    pub thinking_music: Option<String>,
    pub action_hook: Option<String>,
    pub event_hook: Option<String>,
    pub tts: Synthesizer,
    pub baregin: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DTMF {
    pub dtmf: String,
    pub duration: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Enqueue {
    pub name: String,
    pub priority: Option<u16>,
    pub action_hook: Option<String>,
    pub wait_hook: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Gather {
    pub action_hook: Option<String>,
    pub bargein: Option<bool>,
    pub dtmf_bargein: Option<bool>,
    pub finish_on_key: Option<String>,
    pub input: Option<Vec<String>>,
    pub inter_digit_timeout: Option<u8>,
    pub listen_during_prompt: Option<bool>,
    pub min_bargein_word_count: Option<u8>,
    pub min_digits: Option<u8>,
    pub max_digits: Option<u8>,
    pub num_digits: Option<u8>,
    pub partial_result_hook: Option<String>,
    pub play: Option<Play>,
    pub say: Option<Say>,
    pub recognizer: Option<Recognizer>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Hangup {
    pub headers: Option<HashMap<String, String>>,
}

impl Into<Verb> for Hangup {
    fn into(self) -> Verb {
        Verb::Hangup(self)
    }
}

impl Hangup {
    pub fn hangup() -> Hangup {
        Hangup {
            headers: Some(HashMap::new()),
        }
    }

    pub fn hangup_with_reason(x_reason: &str) -> Hangup {
        let mut map = HashMap::new();
        map.insert("X-Reason".to_string(), x_reason.to_string());
        Hangup { headers: Some(map) }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Leave {}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Lex {
    pub bot_id: String,
    pub bot_alias: String,
    pub credentials: AWSCredentials,
    pub region: String,
    pub locale: LexLocale,
    pub event_hook: Option<String>,
    pub intent: Option<String>,
    pub welcome_message: Option<String>,
    pub no_input_timeout: Option<u8>,
    pub tts: Option<Synthesizer>,
    pub metadata: Option<LexMeta>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LexMeta {
    pub slots: Option<HashMap<String, String>>,
    pub context: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum LexLocale {
    #[serde(rename = "en_AU")]
    EnglishAU,
    #[serde(rename = "en_GB")]
    EnglishGB,
    #[serde(rename = "en_US")]
    EnglishUS,
    #[serde(rename = "fr_CA")]
    FrenchCA,
    #[serde(rename = "fr_FR")]
    FrenchFR,
    #[serde(rename = "es_ES")]
    FrenchES,
    #[serde(rename = "es_US")]
    SpanishUS,
    #[serde(rename = "it_IT")]
    ItalianIT,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AWSCredentials {
    pub access_key: String,
    pub secret_access_key: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Play {
    pub url: String,
    #[serde(rename = "loop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_loop: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seek_offset: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_hook: Option<String>,
}

impl Into<Verb> for Play {
    fn into(self) -> Verb {
        Verb::Play(self)
    }
}

impl Play {
    pub fn new(url: &str, action_hook: Option<String>) -> Self {
        Play {
            url: url.to_string(),
            action_hook,
            play_loop: None,
            early_media: None,
            timeout_secs: None,
            seek_offset: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Synthesizer {
    //change to enum
    pub vendor: Option<String>,
    //change to enum
    pub language: Option<String>,
    //change to enum
    pub gender: Option<String>,
    //change to enum
    pub voice: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SipRec {
    pub action: SipRecAction,
    pub siprec_server_url: String,
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BargeIn {
    pub enable: Option<bool>,
    pub sticky: Option<bool>,
    pub action_hook: Option<String>,
    pub input: Vec<String>,
    pub finish_on_key: Option<String>,
    pub num_digits: Option<u8>,
    pub min_digits: Option<u8>,
    pub max_digits: Option<u8>,
    pub inter_digit_timeout: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Listen {
    pub verb: String,
    pub url: String,
    pub action_hook: String,
    pub finish_on_key: Option<String>,
    pub max_length: Option<u16>,
    pub metadata: Option<HashMap<String, String>>,
    pub mix_type: Option<MixType>,
    pub pass_dtmf: Option<bool>,
    pub play_beep: Option<bool>,
    pub sample_rate: SampleRate,
    pub timeout: u8,
    pub transcribe: Option<Transcribe>,
    pub ws_auth: Option<WSAuth>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub from: String,
    pub to: String,
    pub text: String,
    pub carrier: Option<String>,
    pub action_hook: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pause {
    pub length: u8,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Rasa {
    pub url: String,
    pub prompt: Option<String>,
    pub event_hook: Option<String>,
    pub action_hook: Option<String>,
    pub tts: Option<Synthesizer>,
    pub recognizer: Option<Recognizer>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Redirect {
    pub action_hook: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SipDecline {
    pub status: Option<SIPStatus>,
    pub reason: Option<String>,
    pub headers: Option<HashMap<String, String>>,
}

impl SipDecline {
    pub fn custom(status: SIPStatus, reason: String, x_reason: String) -> SipDecline {
        let mut map = HashMap::new();
        map.insert("X-Reason".to_string(), x_reason);
        SipDecline {
            status: Some(status),
            reason: Some(reason),
            headers: Some(map),
        }
    }

    pub fn server_error(reason: String, x_reason: String) -> SipDecline {
        let mut map = HashMap::new();
        map.insert("X-Reason".to_string(), x_reason);
        SipDecline {
            status: Some(SIPStatus::InternalServerError),
            reason: Some(reason),
            headers: Some(map),
        }
    }

    pub fn unauthorised(reason: String, x_reason: String) -> SipDecline {
        let mut map = HashMap::new();
        map.insert("X-Reason".to_string(), x_reason);
        SipDecline {
            status: Some(SIPStatus::Unauthorized),
            reason: Some(reason),
            headers: Some(map),
        }
    }

    pub fn not_found(reason: String, x_reason: String) -> SipDecline {
        let mut map = HashMap::new();
        map.insert("X-Reason".to_string(), x_reason);
        SipDecline {
            status: Some(SIPStatus::Decline),
            reason: Some(reason),
            headers: Some(map),
        }
    }

    pub fn not_implemented(reason: String) -> SipDecline {
        let mut map = HashMap::new();
        map.insert("X-Reason".to_string(), "Not Implemented".to_string());
        SipDecline {
            status: Some(SIPStatus::NotImplemented),
            reason: Some(reason),
            headers: Some(map),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SipRefer {
    pub refer_to: String,
    pub action_hook: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub data: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WSAuth {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transcribe {
    pub verb: String,
    pub transcription_hook: String,
    pub recognizer: Recognizer,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Recognizer {
    pub vendor: Option<String>,
    pub language: Option<String>,
    pub interim: Option<bool>,
    pub hints: Option<Vec<String>>,
    pub hints_boost: Option<u8>,
    pub profanity_filter: Option<bool>,
    pub single_utterance: Option<bool>,
    pub vad: Vad,
    pub separate_recognition_per_channel: Option<bool>,
    pub alt_languages: Vec<String>,
    pub punctuation: Option<bool>,
    pub model: Option<GoogleSpeechModel>,
    pub enhanced_model: Option<bool>,
    pub words: Option<bool>,
    pub diarization: Option<bool>,
    pub diarization_min_speakers: Option<u8>,
    pub diarization_max_speakers: Option<u8>,
    pub interaction_type: Option<GoogleInteractionType>,
    pub naics_code: Option<bool>,
    pub vocabulary_name: Option<String>,
    pub vocabulary_filter_name: Option<String>,
    pub filter_method: Option<AWSFilterMethod>,
    pub identify_channels: Option<bool>,
    pub profanity_option: Option<MSProfanityOption>,
    pub output_format: Option<MSOutputFormat>,
    pub request_snr: Option<bool>,
    pub initial_speech_timeout_ms: Option<u16>,
    pub transcription_hook: String,
    pub asr_timeout: Option<u8>,
    pub asr_dtmf_termination_digit: Option<String>,
    pub azure_service_endpoint: Option<String>,
    pub azure_options: Option<AzureOptions>,
    pub deepgram_options: Option<DeepgramOptions>,
    pub ibm_options: Option<IBMOptions>,
    pub nuance_options: Option<NuanceOptions>,
    pub nvidia_options: Option<NvidiaOptions>,
    pub soniox_options: Option<SonioxOptions>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AzureOptions {
    pub speech_segmentation_silence_timeout_ms: Option<u16>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NuanceOptions {
    pub client_id: Option<String>,
    pub secret: Option<String>,
    pub krypton_endpoint: Option<String>,
    pub topic: Option<String>,
    pub utterance_detection_mode: Option<NuanceUtteranceDetectionMode>,
    pub punctuation: Option<bool>,
    pub include_tokenization: Option<bool>,
    pub discard_speaker_adaptation: Option<bool>,
    pub suppress_call_recording: Option<bool>,
    pub mask_load_failures: Option<bool>,
    pub suppress_initial_capitalization: Option<bool>,
    pub allow_zero_base_lm_weight: Option<bool>,
    pub filter_wakeup_word: Option<bool>,
    pub result_type: Option<NuanceResultType>,
    pub no_input_timeout_ms: Option<u16>,
    pub recognition_timeout_ms: Option<u16>,
    pub utterance_end_silence_ms: Option<u16>,
    pub max_hypotheses: Option<u8>,
    pub speech_domain: Option<bool>,
    pub user_id: Option<String>,
    pub speech_detection_sensitivity: Option<f32>,
    pub client_data: Option<HashMap<String, String>>,
    pub formatting: Option<NuanceFormatting>,
    pub resource: Vec<NuanceResource>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NuanceResource {
    pub inline_wordset: Option<String>,
    pub builtin: Option<String>,
    pub inline_grammar: Option<String>,
    pub wakeup_word: Option<Vec<String>>,
    pub weight_name: Option<NuanceWeightName>,
    pub weight_value: Option<f32>,
    pub reuse: Option<NuanceReusePolicy>,
    pub external_reference: Option<NuanceExternalReference>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NuanceExternalReference {
    #[serde(rename = "type")]
    pub ref_type: Option<NuanceReferenceType>,
    pub uri: Option<String>,
    pub max_load_failures: Option<bool>,
    pub request_timeout_ms: Option<u16>,
    pub headers: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NuanceFormatting {
    pub scheme: Option<String>,
    pub options: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NuanceReferenceType {
    UndefinedResourceType,
    Wordset,
    CompiledWordset,
    DomainLm,
    SpeakerProfile,
    Grammer,
    Settings,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NuanceReusePolicy {
    UndefinedReuse,
    LowReuse,
    HighReuse,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NuanceWeightName {
    DefaultWeight,
    Lowest,
    Low,
    Medium,
    High,
    Highest,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NuanceResultType {
    Final,
    Partial,
    ImmutablePartial,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum NuanceUtteranceDetectionMode {
    Single,
    Multiple,
    Disabled,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeepgramOptions {}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IBMOptions {
    pub stt_api_key: String,
    pub stt_region: String,
    pub instance_id: String,
    pub model: String,
    pub language_customization_id: String,
    pub acoustic_customization_id: String,
    pub base_model_version: String,
    pub watson_metadata: String,
    pub watson_learning_opt_out: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NvidiaOptions {
    pub riva_uri: String,
    pub max_alternatives: u8,
    pub profanity_filter: bool,
    pub punctuation: bool,
    pub word_time_offsets: bool,
    pub verbatim_transcripts: bool,
    pub custom_configuration: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SonioxOptions {
    pub api_key: Option<String>,
    pub model: Option<SonioxModel>,
    pub profanity_filter: Option<bool>,
    pub storage: Option<SonioxStorage>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SonioxStorage {
    pub id: String,
    pub title: String,
    pub disable_store_audio: bool,
    pub disable_store_transcript: bool,
    pub disable_search: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SonioxModel {
    PrecisionIvr,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Vad {
    pub enable: Option<bool>,
    pub voice_ms: Option<u16>,
    pub mode: Option<VadMode>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MSOutputFormat {
    Simple,
    Detailed,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MSProfanityOption {
    Masked,
    Removed,
    Raw,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AWSFilterMethod {
    Remove,
    Mask,
    Tag,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GoogleSpeechModel {
    PhoneCall,
    Telephony,
    TelephonyShort,
    MedicalDictation,
    MedialConversation,
    LatestShort,
    LatestLong,
    CommandAndSearch,
    Default,
    Video,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GoogleInteractionType {
    Discussion,
    Presentation,
    PhoneCall,
    VoiceMail,
    ProfessionallyProduced,
    VoiceSearch,
    VoiceCommand,
    Dictation,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum VadMode {
    M0 = 0,
    M1 = 1,
    M2 = 2,
    M3 = 3,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum SampleRate {
    SR8000 = 8000,
    SR16000 = 16000,
    SR24000 = 24000,
    SR48000 = 48000,
    SR64000 = 64000,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum MixType {
    Mono,
    Stereo,
    Mixed,
}

/*
Requests
*/

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
//todo add DialogFlow
//todo add rasaResult
pub enum Request {
    Initial(InitialRequest),
    Dial(SubsequentDialRequest),
    Queue(SubsequentQueueRequest),
    Subsequent(SubsequentRequest),
    BEvent(ChildEvent),
    AEvent(ParentEvent),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ParentEvent {
    pub call_sid: String,
    pub direction: Direction,
    pub from: String,
    pub to: String,
    pub call_id: String,
    pub sbc_callid: String,
    pub sip_status: SIPStatus,
    pub sip_reason: String,
    pub call_status: String,
    pub account_sid: String,
    pub trace_id: String,
    pub application_sid: String,
    pub fs_sip_address: String,
    pub fs_public_ip: String,
    pub api_base_url: String,
    pub originating_sip_ip: String,
    pub originating_sip_trunk_name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChildEvent {
    pub call_sid: String,
    pub direction: Direction,
    pub from: String,
    pub to: String,
    pub call_id: String,
    pub sbc_callid: String,
    pub sip_status: SIPStatus,
    pub sip_reason: String,
    pub call_status: String,
    pub account_sid: String,
    pub trace_id: String,
    pub application_sid: String,
    pub fs_sip_address: String,
    pub fs_public_ip: String,
    pub parent_call_sid: String,
    pub api_base_url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InitialRequest {
    pub sip: SipPayload,
    pub direction: Direction,
    pub account_sid: String,
    pub application_sid: String,
    pub call_sid: String,
    pub call_id: String,
    pub from: String,
    pub to: String,
    pub caller_name: String,
    pub sip_status: SIPStatus,
    pub call_status: String,
    pub trace_id: String,
    pub public_ip: String,
    pub service_provider_sid: String,
    pub local_sip_address: String,
    pub originating_sip_ip: Option<String>,
    pub originating_sip_trunk_name: Option<String>,
}

impl InitialRequest {
    pub fn get_contact_ip(&self) -> String {
        self.sip.get_contact_ip()
    }

    pub fn get_tenant_type(&self, proxies: Vec<&str>) -> TenantType {
        if self.sip.has_proxy(proxies) {
            TenantType::PROXY
        } else if self.sip.has_teams() {
            TenantType::TEAMS
        } else if self.sip.has_user() {
            TenantType::USER
        } else {
            TenantType::TRUNK
        }
        //todo impl APPLICATION
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SubsequentRequest {
    pub direction: Direction,
    pub account_sid: String,
    pub application_sid: String,
    pub call_sid: String,
    pub call_id: String,
    pub from: String,
    pub to: String,
    pub sip_status: SIPStatus,
    pub call_status: String,
    pub fs_sip_address: String,
    pub trace_id: String,
    pub originating_sip_ip: Option<String>,
    pub originating_sip_trunk_name: Option<String>,
    pub duration: Option<u16>,
    pub digits: Option<String>,
    pub speech: Option<Speech>,
    #[serde(alias = "customerData")]
    #[serde(alias = "customerdata")]
    #[serde(alias = "customer_data")]
    pub customer_data: CustomerData,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CustomerData {
    pub x_cid: String,
    pub paid: Option<String>,
    pub service_url: Option<String>,
    pub forwarded_ip: Option<String>,
    pub trunk_id: Option<String>,
    pub ddi_id: Option<String>,
    pub region_id: Option<String>,
    pub server_ip: Option<String>,
    pub customer_id: Option<String>,
    pub teams_id: Option<String>,
    pub client_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SubsequentDialRequest {
    pub direction: Direction,
    pub account_sid: String,
    pub application_sid: String,
    pub call_sid: String,
    pub call_id: String,
    pub from: String,
    pub to: String,
    pub sip_status: SIPStatus,
    pub call_status: String,
    pub fs_sip_address: String,
    pub fs_public_ip: String,
    pub originating_sip_ip: Option<String>,
    pub originating_sip_trunk_name: Option<String>,
    pub duration: Option<u16>,
    pub digits: Option<String>,
    pub speech: Option<Speech>,
    #[serde(alias = "customerData")]
    #[serde(alias = "customerdata")]
    #[serde(alias = "customer_data")]
    pub customer_data: HashMap<String, String>,
    pub dial_call_sid: String,
    pub dial_call_status: String,
    pub dial_sip_status: SIPStatus,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SubsequentQueueRequest {
    pub direction: Direction,
    pub account_sid: String,
    pub application_sid: String,
    pub call_sid: String,
    pub parent_call_sid: String,
    pub call_id: String,
    pub from: String,
    pub to: String,
    pub caller_name: String,
    pub sip_status: SIPStatus,
    pub call_status: String,
    pub fs_sip_address: String,
    pub public_ip: String,
    pub originating_sip_ip: Option<String>,
    pub originating_sip_trunk_name: Option<String>,
    pub duration: Option<u16>,
    pub digits: Option<String>,
    pub speech: Option<Speech>,
    #[serde(alias = "customerData")]
    #[serde(alias = "customerdata")]
    #[serde(alias = "customer_data")]
    pub customer_data: HashMap<String, String>,
    pub queue_sid: String,
    pub queue_time: u16,
    pub queue_position: u16,
    pub queue_size: u16,
    pub queue_result: QueueResult,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum QueueResult {
    Hangup,
    Leave,
    Bridged,
    Error,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Speech {
    pub stability: Option<u8>,
    pub final_result: bool,
    #[serde(default)]
    pub alternatives: Vec<Alternative>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Alternative {
    pub confidence: f32,
    pub transcript: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SipPayload {
    pub headers: SipPayloadHeaders,
    pub payload: Vec<HashMap<String, String>>,
    pub body: String,
    pub method: String,
    pub version: String,
    pub uri: String,
    pub raw: String,
}

impl SipPayload {
    fn get_contact_ip(&self) -> String {
        self.headers.get_contact_ip()
    }

    fn has_user(&self) -> bool {
        self.headers.x_authenticated_user.is_some()
    }

    fn has_teams(&self) -> bool {
        self.headers.x_ms_teams_tenant_fqdn.is_some()
    }

    fn has_proxy(&self, proxies: Vec<&str>) -> bool {
        let mut is_match = false;
        for x in proxies {
            let res = iface_in_subnet(self.headers.x_forwarded_for.as_str(), &x).unwrap();
            if res {
                is_match = true;
            }
        }
        is_match
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SipPayloadHeaders {
    pub via: String,
    #[serde(rename = "max-forwards")]
    pub max_forwards: String,
    pub from: String,
    pub to: String,
    #[serde(rename = "call-id")]
    pub call_id: String,
    pub cseq: String,
    pub contact: String,
    #[serde(rename = "user-agent")]
    pub user_agent: String,
    pub allow: String,
    pub supported: String,
    #[serde(rename = "min-se")]
    pub min_se: String,
    #[serde(rename = "content-type")]
    pub content_type: String,
    #[serde(rename = "content-length")]
    pub content_length: String,
    #[serde(rename = "X-Account-Sid")]
    pub x_account_sid: String,
    #[serde(rename = "X-CID")]
    pub x_cid: String,
    #[serde(rename = "X-Forwarded-For")]
    pub x_forwarded_for: String,
    #[serde(rename = "X-Originating-Carrier")]
    pub x_originating_carrier: String,
    #[serde(rename = "X-Voip-Carrier-Sid")]
    pub x_voip_carrier_sid: String,
    #[serde(rename = "X-Application-Sid")]
    pub x_application_sid: String,
    #[serde(rename = "p-asserted-identity")]
    pub p_asserted_identity: Option<String>,
    #[serde(rename = "X-Authenticated-User")]
    pub x_authenticated_user: Option<String>,
    #[serde(rename = "X-MS-Teams-Tenant-FQDN")]
    pub x_ms_teams_tenant_fqdn: Option<String>,
    #[serde(rename = "X-MS-Teams-FQDN")]
    pub x_ms_teams_fqdn: Option<String>,
}

impl SipPayloadHeaders {
    fn get_contact_ip(&self) -> String {
        let re = Regex::new(r"<sip:(.*?):.*").unwrap();
        if let Some(mat) = re.find(&self.contact) {
            mat.as_str()
                .replace("<sip:", "")
                .replace(">", "")
                .split(":")
                .next()
                .unwrap_or("1.1.1.1")
                .into()
        } else {
            String::from("1.1.1.1")
        }
    }
}

#[derive(Serialize, Deserialize, Clone, strum::IntoStaticStr)]
#[repr(u16)]
pub enum Direction {
    #[serde(rename = "inbound")]
    Inbound,
    #[serde(rename = "outbound")]
    Outbound,
}

#[derive(Serialize, Deserialize, Clone, strum::IntoStaticStr)]
#[repr(u16)]
pub enum SIPStatus {
    Trying = 100,
    Ringing = 180,
    CallForwarding = 181,
    Queued = 182,
    SessionInProgress = 183,
    EarlyDialogTerminated = 199,
    Ok = 200,
    Accepted = 202,
    NoNotification = 204,
    MultipleChoices = 300,
    MovedPermanently = 301,
    MovedTemporarily = 302,
    UseProxy = 305,
    AlternativeService = 308,
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    ConditionalRequestFailed = 412,
    RequestEntityTooLarge = 413,
    RequestURITooLong = 414,
    UnsupportedMediaType = 415,
    UnsupportedURIScheme = 416,
    UnknownResourcePriority = 417,
    BadExtension = 420,
    ExtensionRequired = 421,
    SessionIntervalTooSmall = 422,
    IntervalTooBrief = 423,
    BadLocationInformation = 424,
    BadAlertMessage = 425,
    UseIdentityHeader = 428,
    ProvideReferrerIdentity = 429,
    FlowFailed = 430,
    AnonymityDisallowed = 433,
    BadIdentityInfo = 436,
    UnsupportedCertificate = 437,
    InvalidIdentityHeader = 438,
    FirstHopLacksOutboundSupport = 439,
    MaxBreadthExceeded = 440,
    BadInfoPackage = 469,
    ConsentNeeded = 470,
    TemporarilyUnavailable = 480,
    CallDoesNotExist = 481,
    LoopDetected = 482,
    TooManyHops = 483,
    AddressIncomplete = 484,
    Ambiguous = 485,
    BusyHere = 486,
    RequestTerminated = 487,
    NotAcceptableHere = 488,
    BadEvent = 489,
    RequestPending = 491,
    Undecipherable = 493,
    SecurityAgreementRequired = 494,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    ServerTimeout = 504,
    VersionNotSupported = 505,
    MessageTooLarge = 513,
    PushNotificationServiceNotSupported = 555,
    PreconditionFailure = 580,
    BusyEverywhere = 600,
    Decline = 603,
    DoesNotExistAnywhere = 604,
    NotAcceptableGlobal = 606,
    Unwanted = 607,
    Rejected = 608,
}
