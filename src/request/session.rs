use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

use super::properties::{Application, User};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session{
    pub new: bool,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub application: Option<Application>,
    pub attributes: Option<HashMap<String, JsonValue>>,
    pub user: Option<User>
}