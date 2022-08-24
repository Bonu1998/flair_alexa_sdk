use serde_json::Value as JsonValue;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestBody {
    #[serde(rename = "type")]
    pub req_type: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub timestamp: String,
    pub locale: String,
    #[serde(rename = "dialogState")]
    pub dailog_state: Option<String>,
    pub token: Option<String>,
    pub arguments: Option<Vec<HashMap<String, String>>>,
    pub source: Option<JsonValue>,
    pub components: Option<JsonValue>,
    pub intent: Option<Intent>,
    pub reason: Option<String>,
    pub error: Option<ReqError>,
    #[serde(rename = "shouldLinkResultBeReturned")]
    pub should_link_result_be_returned: Option<bool>,
    pub cause: Option<Cause>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cause {
    #[serde(rename = "type")]
    pub cause_type: String,
    pub token: String,
    status: HashMap<String, String>,
    result: Option<HashMap<String, String>>,
}

impl Cause {
    pub fn to_hash_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("cause_type".to_string(), self.cause_type.clone());
        map.insert("token".to_string(), self.token.clone());
        map.insert("cause_type".to_string(), self.cause_type.clone());
        for (key, value) in self.status.clone() {
            map.insert(format!("status_{}", key), value);
        }
        if let Some(result) = self.result.clone() {
            for (key, value) in result {
                map.insert(format!("result_{}", key), value);
            }
        }
        map
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReqError {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Intent {
    pub name: String,
    #[serde(rename = "confirmationStatus")]
    pub confirmation_status: Option<String>,
    pub slots: Option<HashMap<String, SlotName>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SlotName {
    pub name: String,
    pub value: Option<String>,
    #[serde(rename = "confirmationStatus")]
    pub confirmation_status: Option<String>,
    #[serde(rename = "slotValue")]
    pub slot_value: Option<Box<SlotName>>,
    pub resolutions: Option<Resolutions>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resolutions {
    #[serde(rename = "resolutionsPerAuthority")]
    pub resolutions_per_authority: Vec<ResolutionsPerAuthority>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResolutionsPerAuthority {
    pub authority: String,
    pub status: Status,
    pub values: Vec<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Value {
    pub value: ValueProperties,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueProperties {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Status {
    pub code: String,
}
