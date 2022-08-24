use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

pub mod card;
pub mod commands;
pub mod directive;
pub mod play_behaviour;
mod response_body;
pub mod speech;

use self::{
    card::Card,
    directive::Directive,
    response_body::ResponseBody,
    speech::{Reprompt, Speech},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub version: String,
    #[serde(rename = "sessionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_attributes: Option<HashMap<String, JsonValue>>,
    #[serde(rename = "response")]
    pub response_body: ResponseBody,
}

impl Response {
    pub fn default_session_close() -> Response {
        let mut _resp = Response::new("1.0".to_string());
        _resp.with_should_end_session(true);
        _resp
    }
    pub fn new(version: String) -> Response {
        Response {
            version,
            session_attributes: None,
            response_body: ResponseBody::new(None, None, None, None, None),
        }
    }

    pub fn set_session_attributes(&mut self, data: HashMap<String, JsonValue>) {
        self.session_attributes = Some(data);
    }

    pub fn speak(&mut self, data: Speech) {
        self.response_body.output_speech = Some(data);
    }

    pub fn reprompt(&mut self, data: Speech) {
        self.response_body.reprompt = Some(Reprompt::new(data));
    }

    pub fn card(&mut self, data: Card) {
        self.response_body.card = Some(data);
    }

    pub fn with_should_end_session(&mut self, should_end_session: bool) {
        self.response_body.should_end_session = Some(should_end_session);
    }

    pub fn add_directive(&mut self, directive: Directive) {
        match self.response_body.directive.clone() {
            Some(mut r) => {
                r.push(directive);
                self.response_body.directive = Some(r);
            }
            None => {
                self.response_body.directive = Some(vec![directive]);
            }
        }
    }
}
