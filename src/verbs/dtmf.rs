    use serde::{Deserialize, Serialize};
    use crate::verbs::verb::Verb;

    #[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct DTMF {
        pub dtmf: String,
        pub duration: Option<u8>,
    }

    impl Into<Verb> for DTMF {
        fn into(self) -> Verb {
            Verb::Dtmf(self)
        }
    }

    impl Into<Vec<Verb>> for DTMF {
        fn into(self) -> Vec<Verb> {
            vec![self.into()]
        }
    }
