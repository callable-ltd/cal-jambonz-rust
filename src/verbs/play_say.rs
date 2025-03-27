use serde::{Deserialize, Serialize};
use crate::play::Play;
use crate::say::Say;

#[derive(Serialize, Deserialize)]
#[serde(tag = "verb")]
#[serde(rename_all = "camelCase")]
pub enum PlaySay {
    Play(Play),
    Say(Say),
}