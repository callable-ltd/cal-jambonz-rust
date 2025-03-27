use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
    pub struct Dub {
        pub queue_command: bool,
        #[serde(flatten)]
        pub data: Vec<DubData>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct DubData {
        pub verb: String,
        pub action: DubTrack,
        pub track: String,
        pub play: Option<String>,
        pub say: Option<String>,
        pub loop_count: Option<u8>,
        pub gain: Option<String>,
    }

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub enum DubTrack {
        AddTrack,
        RemoveTrack,
        SilenceTrack,
        PlayOnTrack,
        SayOnTrack,
    }
