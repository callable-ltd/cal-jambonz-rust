    use std::collections::HashMap;
    use std::fs::File;
    use serde::{Deserialize, Serialize};
    use serde_repr::{Deserialize_repr, Serialize_repr};

    #[derive(Serialize, Deserialize)]
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

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Conference {
        name: String,
        beep: Option<bool>,
        action_hook: Option<String>,
        enter_hook: Option<String>,
        join_muted: Option<bool>,
        max_participants: Option<u8>,
        end_conference_on_exit: Option<bool>,
        start_conference_on_enter: Option<bool>,
        status_hook: Option<String>,
        status_events: Vec<String>,
        wait_hook: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Config {
        amd: Option<bool>,
        bargein: Option<BargeIn>,
        listen: Option<Listen>,
        notify_events: Option<bool>,
        on_hold_music: Option<String>,
        recognizer: Option<Recognizer>,
        reset: Option<Vec<String>>,
        record: Option<SipRec>,
        sip_request_within_dialog_hook: Option<String>,
        synthesizer: Synthesizer,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Dequeue {
        name: String,
        action_hook: Option<String>,
        beep: Option<bool>,
        timeout: Option<u8>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Dial {
        action_hook: Option<String>,
        amd: Option<bool>,
        answer_on_bridge: Option<bool>,
        caller_id: Option<String>,
        confirm_hook: Option<String>,
        dial_music: Option<String>,
        dtmf_capture: Option<Vec<String>>,
        dtmf_hook: Option<String>,
        headers: Option<HashMap<String, String>>,
        listen: Option<Listen>,
        refer_hook: Option<String>,
        target: Vec<Target>,
        time_limit: Option<u16>,
        timeout: Option<u8>,
        transcribe: Option<Transcribe>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "type")]
    pub enum Target {
        Pstn(Pstn),
        Sip(Sip),
        User(User),
        Teams(Teams),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Pstn {
        #[serde(rename = "type")]
        target_type: String,
        number: String,
        confirm_hook: Option<String>,
        trunk: Option<String>,
        headers: Option<HashMap<String, String>>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Sip {
        #[serde(rename = "type")]
        target_type: String,
        sip_uri: String,
        confirm_hook: Option<String>,
        auth: Option<WSAuth>,
        headers: Option<HashMap<String, String>>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct User {
        #[serde(rename = "type")]
        target_type: String,
        name: String,
        confirm_hook: Option<String>,
        headers: Option<HashMap<String, String>>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Teams {
        #[serde(rename = "type")]
        target_type: String,
        number: String,
        confirm_hook: Option<String>,
        tenant: Option<String>,
        voicemail: Option<bool>,
        headers: Option<HashMap<String, String>>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Say {
        text: String,
        synthesizer: Synthesizer,
        #[serde(rename = "loop")]
        say_loop: Option<u8>,
        early_media: Option<bool>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DialogFlow {
        project: String,
        lang: String,
        credentials: String,
        welcome_event: Option<String>,
        welcome_event_params: Option<HashMap<String, String>>,
        no_input_timeout: Option<u8>,
        no_input_event: Option<String>,
        pass_dtmf_as_text_input: Option<String>,
        thinking_music: Option<String>,
        action_hook: Option<String>,
        event_hook: Option<String>,
        tts: Synthesizer,
        baregin: Option<bool>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DTMF {
        dtmf: String,
        duration: Option<u8>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Enqueue {
        name: String,
        priority: Option<u16>,
        action_hook: Option<String>,
        wait_hook: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Gather {
        action_hook: Option<String>,
        bargein: Option<bool>,
        dtmf_bargein: Option<bool>,
        finish_on_key: Option<String>,
        input: Option<Vec<String>>,
        inter_digit_timeout: Option<u8>,
        listen_during_prompt: Option<bool>,
        min_bargein_word_count: Option<u8>,
        min_digits: Option<u8>,
        max_digits: Option<u8>,
        num_digits: Option<u8>,
        partial_result_hook: Option<String>,
        play: Option<Play>,
        say: Option<Say>,
        recognizer: Option<Recognizer>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Hangup {
        headers: Option<HashMap<String, String>>,
    }

    impl Hangup {
        pub fn hangup(x_reason: String) -> Hangup {
            let mut map = HashMap::new();
            map.insert("X-Reason".to_string(), x_reason);
            Hangup { headers: Some(map) }
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Leave {}

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Lex {
        bot_id: String,
        bot_alias: String,
        credentials: AWSCredentials,
        region: String,
        locale: LexLocale,
        event_hook: Option<String>,
        intent: Option<String>,
        welcome_message: Option<String>,
        no_input_timeout: Option<u8>,
        tts: Option<Synthesizer>,
        metadata: Option<LexMeta>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct LexMeta {
        slots: Option<HashMap<String, String>>,
        context: Option<HashMap<String, String>>,
    }

    #[derive(Serialize, Deserialize)]
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

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AWSCredentials {
        access_key: String,
        secret_access_key: String,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Play {
        url: String,
        #[serde(rename = "loop")]
        #[serde(skip_serializing_if = "Option::is_none")]
        play_loop: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        early_media: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        timeout_secs: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        seek_offset: Option<u16>,
        #[serde(skip_serializing_if = "Option::is_none")]
        action_hook: Option<String>,
    }

    impl Play {
        pub fn new(url: String, action_hook: String) -> Self {
            Play {
                url: url,
                action_hook: Some(action_hook),
                play_loop: None,
                early_media: None,
                timeout_secs: None,
                seek_offset: None,
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Synthesizer {
        //change to enum
        vendor: Option<String>,
        //change to enum
        language: Option<String>,
        //change to enum
        gender: Option<String>,
        //change to enum
        voice: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SipRec {
        action: SipRecAction,
        siprec_server_url: String,
        recording_id: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum SipRecAction {
        StartCallRecording,
        StopCallRecording,
        PauseCallRecording,
        ResumeCallRecording,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BargeIn {
        enable: Option<bool>,
        sticky: Option<bool>,
        action_hook: Option<String>,
        input: Vec<String>,
        finish_on_key: Option<String>,
        num_digits: Option<u8>,
        min_digits: Option<u8>,
        max_digits: Option<u8>,
        inter_digit_timeout: Option<u8>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Listen {
        verb: String,
        url: String,
        action_hook: String,
        finish_on_key: Option<String>,
        max_length: Option<u16>,
        metadata: Option<HashMap<String, String>>,
        mix_type: Option<MixType>,
        pass_dtmf: Option<bool>,
        play_beep: Option<bool>,
        sample_rate: SampleRate,
        timeout: u8,
        transcribe: Option<Transcribe>,
        ws_auth: Option<WSAuth>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Message {
        from: String,
        to: String,
        text: String,
        carrier: Option<String>,
        action_hook: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Pause {
        length: u8,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Rasa {
        url: String,
        prompt: Option<String>,
        event_hook: Option<String>,
        action_hook: Option<String>,
        tts: Option<Synthesizer>,
        recognizer: Option<Recognizer>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Redirect {
        action_hook: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SipDecline {
        status: Option<SIPStatus>,
        reason: Option<String>,
        headers: Option<HashMap<String, String>>,
    }

    impl SipDecline {
        pub fn server_error(reason: String, x_reason: String) -> SipDecline {
            let mut map = ::std::collections::HashMap::new();
            map.insert("X-Reason".to_string(), x_reason);
            SipDecline {
                status: Some(SIPStatus::InternalServerError),
                reason: Some(reason),
                headers: Some(map),
            }
        }

        pub fn not_found(reason: String, x_reason: String) -> SipDecline {
            let mut map = ::std::collections::HashMap::new();
            map.insert("X-Reason".to_string(), x_reason);
            SipDecline {
                status: Some(SIPStatus::Decline),
                reason: Some(reason),
                headers: Some(map),
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SipRefer {
        refer_to: String,
        action_hook: Option<String>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Tag {
        data: Option<HashMap<String, String>>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WSAuth {
        username: String,
        password: String,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Transcribe {
        verb: String,
        transcription_hook: String,
        recognizer: Recognizer,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Recognizer {
        vendor: Option<String>,
        language: Option<String>,
        interim: Option<bool>,
        hints: Option<Vec<String>>,
        hints_boost: Option<u8>,
        profanity_filter: Option<bool>,
        single_utterance: Option<bool>,
        vad: Vad,
        separate_recognition_per_channel: Option<bool>,
        alt_languages: Vec<String>,
        punctuation: Option<bool>,
        model: Option<GoogleSpeechModel>,
        enhanced_model: Option<bool>,
        words: Option<bool>,
        diarization: Option<bool>,
        diarization_min_speakers: Option<u8>,
        diarization_max_speakers: Option<u8>,
        interaction_type: Option<GoogleInteractionType>,
        naics_code: Option<bool>,
        vocabulary_name: Option<String>,
        vocabulary_filter_name: Option<String>,
        filter_method: Option<AWSFilterMethod>,
        identify_channels: Option<bool>,
        profanity_option: Option<MSProfanityOption>,
        output_format: Option<MSOutputFormat>,
        request_snr: Option<bool>,
        initial_speech_timeout_ms: Option<u16>,
        transcription_hook: String,
        asr_timeout: Option<u8>,
        asr_dtmf_termination_digit: Option<String>,
        azure_service_endpoint: Option<String>,
        azure_options: Option<AzureOptions>,
        deepgram_options: Option<DeepgramOptions>,
        ibm_options: Option<IBMOptions>,
        nuance_options: Option<NuanceOptions>,
        nvidia_options: Option<NvidiaOptions>,
        soniox_options: Option<SonioxOptions>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AzureOptions {
        speech_segmentation_silence_timeout_ms: Option<u16>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NuanceOptions {
        client_id: Option<String>,
        secret: Option<String>,
        krypton_endpoint: Option<String>,
        topic: Option<String>,
        utterance_detection_mode: Option<NuanceUtteranceDetectionMode>,
        punctuation: Option<bool>,
        include_tokenization: Option<bool>,
        discard_speaker_adaptation: Option<bool>,
        suppress_call_recording: Option<bool>,
        mask_load_failures: Option<bool>,
        suppress_initial_capitalization: Option<bool>,
        allow_zero_base_lm_weight: Option<bool>,
        filter_wakeup_word: Option<bool>,
        result_type: Option<NuanceResultType>,
        no_input_timeout_ms: Option<u16>,
        recognition_timeout_ms: Option<u16>,
        utterance_end_silence_ms: Option<u16>,
        max_hypotheses: Option<u8>,
        speech_domain: Option<bool>,
        user_id: Option<String>,
        speech_detection_sensitivity: Option<f32>,
        client_data: Option<HashMap<String, String>>,
        formatting: Option<NuanceFormatting>,
        resource: Vec<NuanceResource>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NuanceResource {
        inline_wordset: Option<String>,
        builtin: Option<String>,
        inline_grammar: Option<String>,
        wakeup_word: Option<Vec<String>>,
        weight_name: Option<NuanceWeightName>,
        weight_value: Option<f32>,
        reuse: Option<NuanceReusePolicy>,
        external_reference: Option<NuanceExternalReference>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NuanceExternalReference {
        #[serde(rename = "type")]
        ref_type: Option<NuanceReferenceType>,
        uri: Option<String>,
        max_load_failures: Option<bool>,
        request_timeout_ms: Option<u16>,
        headers: Option<HashMap<String, String>>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NuanceFormatting {
        scheme: Option<String>,
        options: Option<HashMap<String, String>>,
    }

    #[derive(Serialize, Deserialize)]
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

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum NuanceReusePolicy {
        UndefinedReuse,
        LowReuse,
        HighReuse,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum NuanceWeightName {
        DefaultWeight,
        Lowest,
        Low,
        Medium,
        High,
        Highest,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum NuanceResultType {
        Final,
        Partial,
        ImmutablePartial,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum NuanceUtteranceDetectionMode {
        Single,
        Multiple,
        Disabled,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DeepgramOptions {}

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct IBMOptions {
        stt_api_key: String,
        stt_region: String,
        instance_id: String,
        model: String,
        language_customization_id: String,
        acoustic_customization_id: String,
        base_model_version: String,
        watson_metadata: String,
        watson_learning_opt_out: bool,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NvidiaOptions {
        riva_uri: String,
        max_alternatives: u8,
        profanity_filter: bool,
        punctuation: bool,
        word_time_offsets: bool,
        verbatim_transcripts: bool,
        custom_configuration: Option<HashMap<String, String>>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SonioxOptions {
        api_key: Option<String>,
        model: Option<SonioxModel>,
        profanity_filter: Option<bool>,
        storage: Option<SonioxStorage>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SonioxStorage {
        id: String,
        title: String,
        disable_store_audio: bool,
        disable_store_transcript: bool,
        disable_search: bool,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum SonioxModel {
        PrecisionIvr,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Vad {
        enable: Option<bool>,
        voice_ms: Option<u16>,
        mode: Option<VadMode>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum MSOutputFormat {
        Simple,
        Detailed,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum MSProfanityOption {
        Masked,
        Removed,
        Raw,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum AWSFilterMethod {
        Remove,
        Mask,
        Tag,
    }

    #[derive(Serialize, Deserialize)]
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

    #[derive(Serialize, Deserialize)]
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

    #[derive(Serialize, Deserialize)]
    pub enum VadMode {
        M0 = 0,
        M1 = 1,
        M2 = 2,
        M3 = 3,
    }

    #[derive(Serialize, Deserialize)]
    pub enum SampleRate {
        SR8000 = 8000,
        SR16000 = 16000,
        SR24000 = 24000,
        SR48000 = 48000,
        SR64000 = 64000,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum MixType {
        Mono,
        Stereo,
        Mixed,
    }

    /*
    Requests
    */

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    //todo add DialogFlow
    //todo add rasaResult
    pub enum Request {
        Initial(InitialRequest),
        Dial(SubsequentDialRequest),
        Queue(SubsequentQueueRequest),
        Subsequent(SubsequentRequest),
        BEvent(ChildEvent),
        AEvent(ChildEvent),
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ParentEvent {
        pub call_sid: String,
        pub direction: Direction,
        pub from: String,
        pub to: String,
        pub call_id: String,
        pub sip_status: SIPStatus,
        pub sip_reason: String,
        pub call_status: String,
        pub caller_id: String,
        pub account_sid: String,
        pub trace_id: String,
        pub application_sid: String,
        pub fs_sip_address: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ChildEvent {
        pub call_sid: String,
        pub direction: Direction,
        pub from: String,
        pub to: String,
        pub call_id: String,
        pub sip_status: SIPStatus,
        pub sip_reason: String,
        pub call_status: String,
        pub caller_id: String,
        pub account_sid: String,
        pub trace_id: String,
        pub application_sid: String,
        pub fs_sip_address: String,
        pub parent_call_sid: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
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
        pub fn get_contact_ip(self) -> String { self.sip.get_contact_ip() }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SubsequentRequest {
        direction: Direction,
        account_sid: String,
        application_sid: String,
        call_sid: String,
        call_id: String,
        from: String,
        to: String,
        sip_status: SIPStatus,
        call_status: String,
        fs_sip_address: String,
        trace_id: String,
        originating_sip_ip: Option<String>,
        originating_sip_trunk_name: Option<String>,
        duration: Option<u16>,
        digits: Option<String>,
        speech: Option<Speech>,
        #[serde(alias = "customerData")]
        #[serde(alias = "customerdata")]
        #[serde(alias = "customer_data")]
        customer_data: HashMap<String, String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SubsequentDialRequest {
        direction: Direction,
        account_sid: String,
        application_sid: String,
        call_sid: String,
        call_id: String,
        from: String,
        to: String,
        sip_status: SIPStatus,
        call_status: String,
        fs_sip_address: String,
        fs_public_ip: String,
        originating_sip_ip: Option<String>,
        originating_sip_trunk_name: Option<String>,
        duration: Option<u16>,
        digits: Option<String>,
        speech: Option<Speech>,
        #[serde(alias = "customerData")]
        #[serde(alias = "customerdata")]
        #[serde(alias = "customer_data")]
        customer_data: HashMap<String, String>,
        dial_call_sid: String,
        dial_call_status: String,
        dial_sip_status: SIPStatus,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SubsequentQueueRequest {
        direction: Direction,
        account_sid: String,
        application_sid: String,
        call_sid: String,
        parent_call_sid: String,
        call_id: String,
        from: String,
        to: String,
        caller_name: String,
        sip_status: SIPStatus,
        call_status: String,
        fs_sip_address: String,
        public_ip: String,
        originating_sip_ip: Option<String>,
        originating_sip_trunk_name: Option<String>,
        duration: Option<u16>,
        digits: Option<String>,
        speech: Option<Speech>,
        #[serde(alias = "customerData")]
        #[serde(alias = "customerdata")]
        #[serde(alias = "customer_data")]
        customer_data: HashMap<String, String>,
        queue_sid: String,
        queue_time: u16,
        queue_position: u16,
        queue_size: u16,
        queue_result: QueueResult,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub enum QueueResult {
        Hangup,
        Leave,
        Bridged,
        Error,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Speech {
        stability: Option<u8>,
        final_result: bool,
        #[serde(default)]
        alternatives: Vec<Alternative>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Alternative {
        confidence: f32,
        transcript: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SipPayload {
        headers: SipPayloadHeaders,
        payload: Vec<HashMap<String, String>>,
        body: String,
        method: String,
        version: String,
        uri: String,
        raw: String,
    }

    impl SipPayload {
        fn get_contact_ip(self) -> String {
            self.headers.get_contact_ip()
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SipPayloadHeaders {
        via: String,
        #[serde(rename = "max-forwards")]
        max_forwards: String,
        from: String,
        to: String,
        #[serde(rename = "call-id")]
        call_id: String,
        cseq: String,
        contact: String,
        #[serde(rename = "user-agent")]
        user_agent: String,
        allow: String,
        supported: String,
        #[serde(rename = "min-se")]
        min_se: String,
        #[serde(rename = "content-type")]
        content_type: String,
        #[serde(rename = "content-length")]
        content_length: String,
        #[serde(rename = "X-Account-Sid")]
        x_account_sid: String,
        #[serde(rename = "X-CID")]
        x_cid: String,
        #[serde(rename = "X-Forwarded-For")]
        x_forwarded_for: String,
        #[serde(rename = "X-Originating-Carrier")]
        x_originating_carrier: String,
        #[serde(rename = "X-Voip-Carrier-Sid")]
        x_voip_carrier_sid: String,
        #[serde(rename = "X-Application-Sid")]
        x_application_sid: String,
        #[serde(rename = "p-asserted-identity")]
        p_asserted_identity: String,
    }

    impl SipPayloadHeaders {
        fn get_contact_ip(self) -> String {
            // let re = Regex::new(r"<sip:(.*?):.*").unwrap();
            // if let Some(mat) = re.find(&self.contact) {
            //     String::from(mat.as_str())
            // }
            String::from("1.1.1.1")
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Direction {
        #[serde(rename = "inbound")]
        Inbound,
        #[serde(rename = "outbound")]
        Outbound,
    }

    #[derive(Serialize_repr, Deserialize_repr, Debug)]
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

    #[test]
    fn test_inbound_initial_json() {
        let file = File::open("./assets/initial-request.json")
            .expect("file should open read only");

        let req: Request = serde_json::from_reader(file)
            .expect("file should be proper JSON");

        match req {
            Request::Initial(value) => println!("{:#?}", value),
            _ => panic!("Incorrect request type")
        }
    }

    #[test]
    fn test_inbound_subsequent_json() {
        let file = File::open("./assets/subsequent-request.json")
            .expect("file should open read only");

        let req: Request = serde_json::from_reader(file)
            .expect("file should be proper JSON");

        match req {
            Request::Subsequent(value) => println!("{:#?}", value),
            _ => panic!("Incorrect request type")
        }
    }

    #[test]
    fn test_inbound_subsequent_dial_json() {
        let file = File::open("./assets/subsequent-dial-request.json")
            .expect("file should open read only");

        let req: Request = serde_json::from_reader(file)
            .expect("file should be proper JSON");

        match req {
            Request::Dial(value) => println!("{:#?}", value),
            _ => panic!("Incorrect request type")
        }
    }

    #[test]
    fn test_outbound_json() {
        let file = File::open("./assets/outbound-child-event.json")
            .expect("file should open read only");

        let req: Request = serde_json::from_reader(file)
            .expect("file should be proper JSON");

        match req {
            Request::BEvent(value) => println!("{:#?}", value),
            _ => panic!("Incorrect request type")
        }
    }

    #[test]
    fn test_play() {
        let play: Play = Play::new(
            String::from("http://test.com/play.mp3"),
            String::from("http://test.com/voice/route/1"),
        );

        let verb = Verb::Play(play);
        let json: String = serde_json::to_string(&verb).expect("Play cannot map to JSON string");

        assert_eq!(json, "{\"verb\":\"play\",\"url\":\"http://test.com/play.mp3\",\"actionHook\":\"http://test.com/voice/route/1\"}");

        println!("{:#?}", json)
    }

