use crate::auth::WSAuth;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    #[serde(tag = "type")]
    pub enum Target {
        Phone(Phone),
        Sip(Sip),
        User(User),
        Teams(Teams),
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct Phone {
        pub number: String,

        #[serde(rename = "confirmHook")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub confirm_hook: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub trunk: Option<String>,

        #[serde(skip_serializing_if = "HashMap::is_empty")]
        pub headers: HashMap<String, String>,
    }

    impl Into<Target> for Phone {
        fn into(self) -> Target {
            Target::Phone(self)
        }
    }

    impl Into<Vec<Target>> for Phone {
        fn into(self) -> Vec<Target> {
            vec![self.into()]
        }
    }

    impl Phone {
        pub fn new(number: &str) -> Phone {
            Phone {
                number: number.to_string(),
                confirm_hook: None,
                trunk: None,
                headers: HashMap::new(),
            }
        }

        pub fn confirm_hook(&mut self, confirm_hook: &str) -> &mut Phone {
            self.confirm_hook = Some(confirm_hook.to_string());
            self
        }

        pub fn trunk(&mut self, trunk: &str) -> &mut Phone {
            self.trunk = Some(trunk.to_string());
            self
        }

        pub fn header(&mut self, key: &str, value: &str) -> &mut Phone {
            self.headers.insert(key.to_string(), value.to_string());
            self
        }

        pub fn replace_headers(&mut self, headers: HashMap<String, String>) -> &mut Phone {
            self.headers = headers;
            self
        }

        pub fn add_headers(&mut self, headers: HashMap<String, String>) -> &mut Phone {
            self.headers.extend(headers);
            self
        }
    }
    #[derive(Serialize, Deserialize, Clone)]
    pub struct Sip {
        #[serde(rename = "sipUri")]
        pub sip_uri: String,

        #[serde(rename = "confirmHook")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub confirm_hook: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub auth: Option<WSAuth>,

        #[serde(skip_serializing_if = "HashMap::is_empty")]
        pub headers: HashMap<String, String>,
    }

    impl Into<Target> for Sip {
        fn into(self) -> Target {
            Target::Sip(self)
        }
    }

    impl Into<Vec<Target>> for Sip {
        fn into(self) -> Vec<Target> {
            vec![self.into()]
        }
    }

    impl Sip {
        pub fn new(sip_uri: &str) -> Sip {
            Sip {
                sip_uri: sip_uri.to_string(),
                confirm_hook: None,
                auth: None,
                headers: HashMap::new(),
            }
        }

        pub fn from_parts(phone_number: &str, ip_address: &str, port: u16) -> Sip {
            Sip {
                sip_uri: format!("sip:{}@{}:{}", phone_number, ip_address, port),
                confirm_hook: None,
                auth: None,
                headers: HashMap::new(),
            }
        }

        pub fn confirm_hook(&mut self, confirm_hook: &str) -> &mut Sip {
            self.confirm_hook = Some(confirm_hook.to_string());
            self
        }

        pub fn auth(&mut self, auth: WSAuth) -> &mut Sip {
            self.auth = Some(auth);
            self
        }

        pub fn header(&mut self, key: &str, value: &str) -> &mut Sip {
            self.headers.insert(key.to_string(), value.to_string());
            self
        }

        pub fn replace_headers(&mut self, headers: HashMap<String, String>) -> &mut Sip {
            self.headers = headers;
            self
        }

        pub fn add_headers(&mut self, headers: HashMap<String, String>) -> &mut Sip {
            self.headers.extend(headers);
            self
        }
    }

    #[derive(Serialize, Deserialize, Clone)]
    pub struct User {
        pub name: String,

        #[serde(rename = "confirmHook")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub confirm_hook: Option<String>,

        #[serde(skip_serializing_if = "HashMap::is_empty")]
        pub headers: HashMap<String, String>,
    }

    impl Into<Target> for User {
        fn into(self) -> Target {
            Target::User(self)
        }
    }

    impl Into<Vec<Target>> for User {
        fn into(self) -> Vec<Target> {
            vec![self.into()]
        }
    }

    impl User {
        pub fn new(sip_uri: &str) -> User {
            User {
                name: sip_uri.to_string(),
                confirm_hook: None,
                headers: HashMap::new(),
            }
        }

        pub fn from_parts(username: &str, domain: &str) -> User {
            User {
                name: format!("{}@{}", username, domain),
                confirm_hook: None,
                headers: HashMap::new(),
            }
        }

        pub fn confirm_hook(&mut self, confirm_hook: &str) -> &mut User {
            self.confirm_hook = Some(confirm_hook.to_string());
            self
        }

        pub fn header(&mut self, key: &str, value: &str) -> &mut User {
            self.headers.insert(key.to_string(), value.to_string());
            self
        }

        pub fn replace_headers(&mut self, headers: HashMap<String, String>) -> &mut User {
            self.headers = headers;
            self
        }

        pub fn add_headers(&mut self, headers: HashMap<String, String>) -> &mut User {
            self.headers.extend(headers);
            self
        }
    }
    #[derive(Serialize, Deserialize, Clone)]
    pub struct Teams {
        pub number: String,

        #[serde(rename = "confirmHook")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub confirm_hook: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub tenant: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub voicemail: Option<bool>,

        #[serde(skip_serializing_if = "HashMap::is_empty")]
        pub headers: HashMap<String, String>,
    }

    impl Into<Target> for Teams {
        fn into(self) -> Target {
            Target::Teams(self)
        }
    }

    impl Into<Vec<Target>> for Teams {
        fn into(self) -> Vec<Target> {
            vec![self.into()]
        }
    }

    impl Teams {
        pub fn new(number: &str) -> Teams {
            Teams {
                number: number.to_string(),
                confirm_hook: None,
                tenant: None,
                voicemail: None,
                headers: HashMap::new(),
            }
        }

        pub fn from_parts(number: &str, tenant: &str) -> Teams {
            Teams {
                number: number.to_string(),
                confirm_hook: None,
                tenant: Some(tenant.to_string()),
                voicemail: None,
                headers: HashMap::new(),
            }
        }

        pub fn tenant(&mut self, tenant: &str) -> &mut Teams {
            self.tenant = Some(tenant.to_string());
            self
        }

        pub fn voicemail(&mut self, tenant: bool) -> &mut Teams {
            self.tenant = Some(tenant.to_string());
            self
        }

        pub fn confirm_hook(&mut self, confirm_hook: &str) -> &mut Teams {
            self.confirm_hook = Some(confirm_hook.to_string());
            self
        }

        pub fn header(&mut self, key: &str, value: &str) -> &mut Teams {
            self.headers.insert(key.to_string(), value.to_string());
            self
        }

        pub fn replace_headers(&mut self, headers: HashMap<String, String>) -> &mut Teams {
            self.headers = headers;
            self
        }

        pub fn add_headers(&mut self, headers: HashMap<String, String>) -> &mut Teams {
            self.headers.extend(headers);
            self
        }
    }
