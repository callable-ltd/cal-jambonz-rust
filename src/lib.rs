mod verbs;
mod shared;
mod payload;

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

