use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BargeIn {
    pub input: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_hook: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_on_key: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_digits: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_digits: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_digits: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inter_digit_timeout: Option<u8>,
}

impl BargeIn {
    pub fn new(input: Vec<String>) -> Self {
        BargeIn {
            input,
            enable: None,
            sticky: None,
            action_hook: None,
            finish_on_key: None,
            num_digits: None,
            min_digits: None,
            max_digits: None,
            inter_digit_timeout: None,
        }
    }
    
    pub fn enable(&mut self, enable: bool) -> &mut BargeIn {
        self.enable = Some(enable);
        self
    }
    
    pub fn sticky(&mut self, sticky: bool) -> &mut BargeIn {
        self.sticky = Some(sticky);
        self
    }
    
    pub fn action_hook(&mut self, action_hook: &str) -> &mut BargeIn {
        self.action_hook = Some(action_hook.to_string());
        self
    }
    
    pub fn finish_on_key(&mut self, finish_on_key: &str) -> &mut BargeIn {
        self.finish_on_key = Some(finish_on_key.to_string());
        self
    }
    
    pub fn num_digits(&mut self, num_digits: u8) -> &mut BargeIn {
        self.num_digits = Some(num_digits);
        self
    }
    
    pub fn min_digits(&mut self, min_digits: u8) -> &mut BargeIn {
        self.min_digits = Some(min_digits);
        self
    }
    
    pub fn max_digits(&mut self, max_digits: u8) -> &mut BargeIn {
        self.max_digits = Some(max_digits);
        self
    }
    
    pub fn inter_digit_timeout(&mut self, inter_digit_timeout: u8) -> &mut BargeIn {
        self.inter_digit_timeout = Some(inter_digit_timeout);
        self
    }
}
