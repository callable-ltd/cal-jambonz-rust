use crate::verbs::auth::WSAuth;
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
        
        /// A telephone number in E.164 format.
        pub number: String,

        /// A webhook for an application to run on the callee’s end after
        /// the dialed number answers but before the call is connected.
        /// This will override the confirmHook property set on the parent dial verb,
        /// if any.
        #[serde(rename = "confirmHook")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub confirm_hook: Option<String>,

        /// If provided, this should be the name of a Carrier 
        /// that you created in the jambonz portal or API, 
        /// which you want to use to complete this call. 
        /// If not provided, jambonz will selectone of your 
        /// configured Carriers that has an outbound trunk.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trunk: Option<String>,

        /// An object containing arbitrary SIP headers
        /// to apply to the outbound call attempt(s).
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
        
        /// Sip uri to send call to
        /// e.g. (sip:+441234567890:5060)
        #[serde(rename = "sipUri")]
        pub sip_uri: String,

        /// A webhook for an application to run on the callee’s end after
        /// the dialed number answers but before the call is connected.
        /// This will override the confirmHook property set on the parent dial verb,
        /// if any.
        #[serde(rename = "confirmHook")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub confirm_hook: Option<String>,

        /// Authentication credentials
        #[serde(skip_serializing_if = "Option::is_none")]
        pub auth: Option<WSAuth>,

        /// An object containing arbitrary SIP headers
        /// to apply to the outbound call attempt(s).
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

        /// Registered sip user, including domain
        /// (e.g. "joeb@sip.jambonz.org")
        pub name: String,

        /// A webhook for an application to run on the callee’s end after
        /// the dialed number answers but before the call is connected.
        /// This will override the confirmHook property set on the parent dial verb,
        /// if any.
        #[serde(rename = "confirmHook")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub confirm_hook: Option<String>,

        /// An object containing arbitrary SIP headers
        /// to apply to the outbound call attempt(s).
        #[serde(skip_serializing_if = "HashMap::is_empty")]
        pub headers: HashMap<String, String>,

        /// Adds X-Override-To: +441234567890 INVITE from FS
        /// This is then translated to outbound INVITE URI on SBC:
        /// (e.g. INVITE sip:+441234567890@1.1.1.1:5060;transport=udp SIP/2.0)
        #[serde(rename = "overrideTo")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub override_to: Option<String>,
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
                override_to: None,
                headers: HashMap::new(),
            }
        }

        pub fn from_parts(username: &str, domain: &str) -> User {
            User {
                name: format!("{}@{}", username, domain),
                confirm_hook: None,
                override_to: None,
                headers: HashMap::new(),
            }
        }

        pub fn confirm_hook(&mut self, confirm_hook: &str) -> &mut User {
            self.confirm_hook = Some(confirm_hook.to_string());
            self
        }

        pub fn override_to(&mut self, override_to: &str) -> &mut User {
            self.override_to = Some(override_to.to_string());
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

        /// The phone number that has been mapped to the 
        /// teams user by the Microsoft Teams administrator
        pub number: String,

        /// A webhook for an application to run on the callee’s end after
        /// the dialed number answers but before the call is connected.
        /// This will override the confirmHook property set on the parent dial verb,
        /// if any.
        #[serde(rename = "confirmHook")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub confirm_hook: Option<String>,

        /// Microsoft Teams customer tenant domain name.
        /// Will default to the Microsoft Teams tenant
        /// associated with the account of the calling party.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tenant: Option<String>,

        /// if true, dial directly into user’s
        /// voicemail to leave a message
        #[serde(skip_serializing_if = "Option::is_none")]
        pub voicemail: Option<bool>,

        /// An object containing arbitrary SIP headers
        /// to apply to the outbound call attempt(s).
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
