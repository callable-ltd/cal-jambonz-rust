//todo Add Dialogflow Request
//todo Add RasaResult Request
//todo Add AmdResult Request

use crate::shared::shared::{Direction, SIPStatus};
use crate::TenantType;
use ip_in_subnet::iface_in_subnet;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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