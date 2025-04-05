use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct WSAuth {
    pub username: String,
    pub password: String,
}

impl WSAuth {
    pub fn new(username: String, password: String) -> WSAuth {
        WSAuth {
            username,
            password,
        }
    }
}

