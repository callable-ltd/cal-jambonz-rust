use crate::verbs::conference::Conference;
use crate::verbs::config::Config;
use crate::verbs::dequeue::Dequeue;
use crate::verbs::dial::Dial;
use crate::verbs::dialogflow::DialogFlow;
use crate::verbs::dtmf::Dtmf;
use crate::verbs::dub::DubData;
use crate::verbs::enqueue::Enqueue;
use crate::verbs::gather::Gather;
use crate::verbs::hangup::Hangup;
use crate::verbs::leave::Leave;
use crate::verbs::lex::Lex;
use crate::verbs::listen::Listen;
use crate::verbs::message::Message;
use crate::verbs::pause::Pause;
use crate::verbs::play::Play;
use crate::verbs::rasa::Rasa;
use crate::verbs::redirect::Redirect;
use crate::verbs::say::Say;
use crate::verbs::sipdecline::SipDecline;
use crate::verbs::siprec::SipRec;
use crate::verbs::siprefer::SipRefer;
use crate::verbs::tag::Tag;
use crate::verbs::transcribe::Transcribe;
use serde::{Deserialize, Serialize};

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
