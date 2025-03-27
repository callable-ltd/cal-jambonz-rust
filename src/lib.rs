pub mod payload;
pub mod shared;
pub mod verbs;

use serde::{Deserialize, Serialize};

pub use verbs::*;
pub use shared::*;
pub use payload::*;

//todo remove TenantType out of lib
#[derive(Serialize, Deserialize, Clone)]
pub enum TenantType {
    PROXY,
    TRUNK,
    USER,
    TEAMS,
    APPLICATION,
}
