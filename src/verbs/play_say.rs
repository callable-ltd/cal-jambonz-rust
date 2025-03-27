use serde::{Deserialize, Serialize};
use crate::verbs::play::Play;
use crate::verbs::say::Say;

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "verb")]
#[serde(rename_all = "camelCase")]
pub enum PlaySay {
    Play(Play),
    Say(Say),
}