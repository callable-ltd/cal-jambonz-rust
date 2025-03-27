use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct WSAuth {
    pub username: String,
    pub password: String,
}

impl WSAuth {
    pub fn new(username: &str, password: &str) -> WSAuth {
        WSAuth {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

