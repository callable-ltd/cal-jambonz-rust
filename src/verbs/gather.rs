use crate::verbs::play::Play;
use crate::verbs::recognizer::Recognizer;
use crate::verbs::say::Say;
use crate::verbs::verb::Verb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Gather {
    pub action_hook: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_hook_delay_action: Option<ActionHookDelayAction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bargein: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtmf_bargein: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_on_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Vec<Input>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inter_digit_timeout: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_during_prompt: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_bargein_word_count: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_digits: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_digits: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_digits: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_result_hook: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub play: Option<Play>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub say: Option<Say>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognizer: Option<Recognizer>,
}

impl Gather {
    pub fn new(action_hook: &str) -> Self {
        Gather {
            action_hook: action_hook.to_string(),
            action_hook_delay_action: None,
            bargein: None,
            dtmf_bargein: None,
            finish_on_key: None,
            input: None,
            inter_digit_timeout: None,
            listen_during_prompt: None,
            min_bargein_word_count: None,
            min_digits: None,
            max_digits: None,
            num_digits: None,
            partial_result_hook: None,
            play: None,
            say: None,
            recognizer: None,
        }
    }

    pub fn action_hook(&mut self, hook: String) -> &mut Gather {
        self.action_hook = hook;
        self
    }

    pub fn action_hook_delay_action(
        &mut self,
        delay_action: Option<ActionHookDelayAction>,
    ) -> &mut Gather {
        self.action_hook_delay_action = delay_action;
        self
    }

    pub fn bargein(&mut self, bargein: Option<bool>) -> &mut Gather {
        self.bargein = bargein;
        self
    }
    
    pub fn dtmf_bargin(&mut self, dtmf: Option<bool>) -> &mut Gather {
        self.dtmf_bargein = dtmf;
        self
    }
    
    pub fn finish_on_key(&mut self, finish_on_key: String) -> &mut Gather {
        self.finish_on_key = Some(finish_on_key);
        self
    }
    
    pub fn input(&mut self, input: Vec<Input>) -> &mut Gather {
        self.input = Some(input);
        self
    }

    pub fn speech(&mut self) -> &mut Gather {
        self.input = Some(vec![Input::Speech]);
        self
    }

    pub fn digits(&mut self) -> &mut Gather {
        self.input = Some(vec![Input::Digits]);
        self
    }

    pub fn speech_digits(&mut self) -> &mut Gather {
        self.input = Some(vec![Input::Digits, Input::Speech]);
        self
    }
    
    pub fn inter_digit_timeout(&mut self, timeout: Option<u8>) -> &mut Gather {
        self.inter_digit_timeout = timeout;
        self
    }
    
    pub fn listen_during_prompt(&mut self, listen: Option<bool>) -> &mut Gather {
        self.listen_during_prompt = listen;
        self
    }
    
    pub fn min_bargein_word_count(&mut self, count: Option<u8>) -> &mut Gather {
        self.min_bargein_word_count = count;
        self
    }
    
    pub fn min_digits(&mut self, digits: Option<u8>) -> &mut Gather {
        self.min_digits = digits;
        self
    }

    pub fn max_digits(&mut self, digits: Option<u8>) -> &mut Gather {
        self.max_digits = digits;
        self
    }

    pub fn num_digits(&mut self, digits: Option<u8>) -> &mut Gather {
        self.max_digits = digits;
        self
    }
    
    pub fn partial_result_hook(&mut self, hook: Option<String>) -> &mut Gather {
        self.partial_result_hook = hook;
        self
    }
    
    pub fn play(&mut self, play: Option<Play>) -> &mut Gather {
        self.play = play;
        self
    }
    
    pub fn say(&mut self, say: Option<Say>) -> &mut Gather {
        self.say = say;
        self
    }
    
    pub fn build(&mut self) -> Gather {
        self.clone().into()
    }
}

impl Into<Verb> for Gather {
    fn into(self) -> Verb {
        Verb::Gather(self)
    }
}

impl Into<Vec<Verb>> for Gather {
    fn into(self) -> Vec<Verb> {
        vec![self.into()]
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActionHookDelayAction {
    actions: Vec<Verb>,
    enabled: bool,
    no_response_give_up_timeout: Option<u8>,
    no_response_timeout: Option<u8>,
    retries: Option<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Input {
    Digits,
    Speech
}
