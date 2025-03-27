use crate::config::Config;
use crate::dequeue::Dequeue;
use crate::dial::Dial;
use crate::dialogflow::DialogFlow;
use crate::dtmf::Dtmf;
use crate::enqueue::Enqueue;
use crate::gather::Gather;
use crate::hangup::Hangup;
use crate::leave::Leave;
use crate::lex::Lex;
use crate::listen::Listen;
use crate::message::Message;
use crate::pause::Pause;
use crate::play::Play;
use crate::rasa::Rasa;
use crate::redirect::Redirect;
use crate::say::Say;
use crate::sipdecline::SipDecline;
use crate::siprefer::SipRefer;
use crate::tag::Tag;
use crate::transcribe::Transcribe;
use crate::verbs::conference::Conference;
use serde::{Deserialize, Serialize};
use crate::dub::DubData;
use crate::siprec::SipRec;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "verb")]
pub enum Verb {
    Conference(Conference),
    Config(Config),
    Dequeue(Dequeue),
    Dial(Dial),
    DialogFlow(DialogFlow),
    Dub(DubData),
    Dtmf(Dtmf),
    Enqueue(Enqueue),
    Gather(Gather),
    Hangup(Hangup),
    Leave(Leave),
    Lex(Lex),
    Listen(Listen),
    Message(Message),
    #[serde(rename = "sip:decline")]
    SipDecline(SipDecline),
    SipRec(SipRec),
    Pause(Pause),
    Play(Play),
    Redirect(Redirect),
    Rasa(Rasa),
    Say(Say),
    #[serde(rename = "sip:refer")]
    SipRefer(SipRefer),
    Tag(Tag),
    Transcribe(Transcribe),
}

impl Into<Vec<Verb>> for Verb {
    fn into(self) -> Vec<Verb> {
        vec![self]
    }
}
