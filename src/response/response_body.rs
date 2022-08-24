use serde::{Deserialize, Serialize};

use super::{
    card::Card,
    directive::Directive,
    speech::{Reprompt, Speech},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseBody {
    #[serde(rename = "outputSpeech")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_speech: Option<Speech>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reprompt: Option<Reprompt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directive: Option<Vec<Directive>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shouldEndSession")]
    pub should_end_session: Option<bool>,
}

impl ResponseBody {
    pub fn new(
        output_speech: Option<Speech>,
        card: Option<Card>,
        reprompt: Option<Reprompt>,
        should_end_session: Option<bool>,
        directive: Option<Vec<Directive>>
    ) -> ResponseBody {
        ResponseBody {
            output_speech,
            card,
            reprompt,
            directive,
            should_end_session,
        }
    }
}
