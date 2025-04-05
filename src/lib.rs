#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod payload;
pub mod shared;
pub mod verbs;

use serde::{Deserialize, Serialize};

pub use payload::*;
pub use shared::*;
pub use verbs::*;

//todo remove TenantType out of lib
#[derive(Serialize, Deserialize, Clone)]
pub enum TenantType {
    PROXY,
    TRUNK,
    USER,
    TEAMS,
    APPLICATION,
}
