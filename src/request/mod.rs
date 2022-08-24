use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, fmt};

mod context;
mod properties;
mod request_body;
mod session;

use self::{context::Context, request_body::RequestBody, session::Session};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    pub version: String,
    pub session: Option<Session>,
    #[serde(rename = "request")]
    pub request_body: RequestBody,
    pub context: Context,
}

impl Request {
    pub fn new() -> Request {
        Request {
            version: "1.0".to_string(),
            session: None,
            request_body: RequestBody {
                req_type: "TEST".to_string(),
                request_id: "123456".to_string(),
                timestamp: "TIME".to_string(),
                locale: "en-US".to_string(),
                dailog_state: None,
                token: None,
                arguments: None,
                source: None,
                components: None,
                intent: None,
                reason: None,
                error: None,
                should_link_result_be_returned: None,
                cause: None,
            },
            context: Context {
                audio_player_details: None,
                apl_context: None,
                system: None,
                viewport: None,
                viewports: None,
                extensions: None,
            },
        }
    }
    pub fn is_new_session(&self) -> bool {
        match &self.session {
            Some(session) => return session.new || false,
            None => {
                error!("Session not found");
                return false;
            }
        }
    }

    pub fn get_session_id(&self) -> Result<String, String> {
        info!("get_session_id invoked");
        match self.session.clone() {
            Some(session) => Ok(session.session_id.to_string()),
            None => Err(String::from("Session id not found")),
        }
    }

    pub fn get_user_id(&self) -> Result<String, String> {
        info!("get_user_id invoked");
        let mut user_id: Option<String> = None;
        if let Some(session) = self.session.clone() {
            if let Some(user) = session.user {
                user_id = Some(user.user_id)
            }
        }
        if let Some(system) = self.context.system.clone() {
            if let Some(user) = system.user {
                user_id = Some(user.user_id)
            }
        }
        match user_id {
            Some(id) => Ok(id),
            None => Err("userId not found".to_string()),
        }
    }

    pub fn get_user_access_token(&self) -> Result<String, String> {
        info!("get_user_id invoked");
        let mut token: Option<String> = None;
        if let Some(session) = self.session.clone() {
            if let Some(user) = session.user {
                if let Some(access_token) = user.access_token {
                    token = Some(access_token);
                }
            }
        }
        if let Some(system) = self.context.system.clone() {
            if let Some(user) = system.user {
                if let Some(access_token) = user.access_token {
                    token = Some(access_token);
                }
            }
        }
        match token {
            Some(id) => Ok(id),
            None => Err("access token not found".to_string()),
        }
    }

    pub fn get_session_attributes(&self) -> Result<JsonValue, String> {
        info!("get_session_attributes invoked");
        match self.session.clone() {
            Some(session) => match session.attributes {
                Some(attributes) => {
                    match attributes.get("session_data"){
                        Some(data) => Ok(data.clone()),
                        None => Err(format!("session_data not found in Session attributes")),
                    }
                },
                None => Err(String::from("Session attributes not found")),
            },
            None => Err(String::from("Session not found")),
        }
    }

    pub fn get_intent_name(&self) -> Result<String, String> {
        info!("get_intent_name invoked");
        if self.request_body.req_type == "IntentRequest".to_string() {
            match self.request_body.intent.clone() {
                Some(intent) => Ok(intent.name),
                None => Err("intent name not found".to_string()),
            }
        } else {
            Err("It is not an intent request.".to_string())
        }
    }

    pub fn get_locale(&self) -> String {
        info!("get_locale invoked");
        self.request_body.locale.clone()
    }

    pub fn is_apl_supported(&self) -> bool {
        info!("is_apl_supported invoked");
        match self.context.system.clone() {
            Some(system) => match system.device {
                Some(device) => match device.supported_interfaces.apl_support {
                    Some(_apl_support) => {
                        debug!("APl Supported");
                        return true;
                    }
                    None => debug!("APL not Supported"),
                },
                None => {
                    debug!("device not found");
                }
            },
            None => {
                debug!("System no found");
            }
        }
        return false;
    }

    pub fn is_audio_player_supported(&self) -> bool {
        info!("is_audio_player_supported invoked");
        match self.context.system.clone() {
            Some(system) => match system.device {
                Some(device) => match device.supported_interfaces.audio_player_support {
                    Some(_apl_support) => {
                        debug!("AudioPlayer Supported");
                        return true;
                    }
                    None => debug!("AudioPlayer not Supported"),
                },
                None => error!("device not found"),
            },
            None => error!("System no found"),
        }
        return false;
    }

    pub fn get_slot_values(&self) -> Vec<HashMap<String, String>> {
        let mut slot_values: Vec<HashMap<String, String>> = Vec::new();
        if let Some(intent) = self.request_body.intent.clone() {
            if let Some(slots) = intent.slots {
                for (_key, val) in slots {
                    if let Some(slot_value_outer) = val.slot_value {
                        let mut entry: HashMap<String, String> = HashMap::new();
                        entry.insert("id".to_string(), "SLOT_VALUE".to_string());
                        if let Some(slot_value_inner) = slot_value_outer.slot_value {
                            if let Some(resolutions) = slot_value_inner.resolutions {
                                if resolutions.resolutions_per_authority.len() > 0
                                    && resolutions.resolutions_per_authority[0].values.len() > 0
                                {
                                    let value = resolutions.resolutions_per_authority[0].values[0]
                                        .value
                                        .clone();
                                    entry.insert("slot_id".to_string(), value.id.clone());
                                    entry.insert("slot_name".to_string(), value.name.clone());
                                    entry.insert(
                                        "slot_value".to_string(),
                                        slot_value_inner.value.unwrap_or_default().clone(),
                                    );
                                    entry.insert(
                                        "slot_type".to_string(),
                                        slot_value_outer.name.clone(),
                                    );
                                    slot_values.push(entry);
                                }
                            }
                        } else {
                            entry.insert("slot_id".to_string(), val.name.clone());
                            entry.insert("slot_name".to_string(), slot_value_outer.name.clone());
                            entry.insert(
                                "slot_value".to_string(),
                                slot_value_outer.value.unwrap_or_default().clone(),
                            );
                            entry.insert("slot_type".to_string(), slot_value_outer.name.clone());
                            slot_values.push(entry);
                        }
                    }
                }
            }
        }
        slot_values.clone()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RequestType {
    AplUserEvent,
    IntentRequest,
    LaunchRequest,
    AudioPlayerPlaybackStarted,
    AudioPlayerPlaybackFinished,
    AudioPlayerPlaybackNearlyFinished,
    AudioPlayerPlaybackStopped,
    PlaybackControllerNextCommandIssued,
    PlaybackControllerPreviousCommandIssued,
    PlaybackControllerPlayCommandIssued,
    PlaybackControllerPauseCommandIssued,
    AudioPlayerPlaybackFailed,
    SessionEndedRequest,
    ConnectionsResponse,
    CanFulfillIntentRequest,
    DisplayElementSelected,
    SessionResumedRequest,
}

impl RequestType {
    pub fn from_str(input: &str) -> RequestType {
        match input {
            "Alexa.Presentation.APL.UserEvent" => RequestType::AplUserEvent,
            "IntentRequest" => RequestType::IntentRequest,
            "LaunchRequest" => RequestType::LaunchRequest,
            "AudioPlayer.PlaybackStarted" => RequestType::AudioPlayerPlaybackStarted,
            "AudioPlayer.PlaybackFinished" => RequestType::AudioPlayerPlaybackFinished,
            "AudioPlayer.PlaybackNearlyFinished" => RequestType::AudioPlayerPlaybackNearlyFinished,
            "AudioPlayer.PlaybackStopped" => RequestType::AudioPlayerPlaybackStopped,
            "AudioPlayer.PlaybackFailed" => RequestType::AudioPlayerPlaybackFailed,
            "PlaybackController.NextCommandIssued" => {
                RequestType::PlaybackControllerNextCommandIssued
            }
            "PlaybackController.PreviousCommandIssued" => {
                RequestType::PlaybackControllerPreviousCommandIssued
            }
            "PlaybackController.PlayCommandIssued" => {
                RequestType::PlaybackControllerPlayCommandIssued
            }
            "PlaybackController.PauseCommandIssued" => {
                RequestType::PlaybackControllerPauseCommandIssued
            }
            "SessionEndedRequest" => RequestType::SessionEndedRequest,
            "Connections.Response" => RequestType::ConnectionsResponse,
            "CanFulfillIntentRequest" => RequestType::CanFulfillIntentRequest,
            "Display.ElementSelected" => RequestType::DisplayElementSelected,
            "SessionResumedRequest" => RequestType::SessionEndedRequest,
            _ => {
                error!("Incorrect Intent request {}", input);
                RequestType::SessionEndedRequest
            }
        }
    }
}
impl fmt::Display for RequestType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            RequestType::AplUserEvent => "Alexa.Presentation.APL.UserEvent",
            RequestType::IntentRequest => "IntentRequest",
            RequestType::LaunchRequest => "LaunchRequest",
            RequestType::AudioPlayerPlaybackStarted => "AudioPlayer.PlaybackStarted",
            RequestType::AudioPlayerPlaybackFinished => "AudioPlayer.PlaybackFinished",
            RequestType::AudioPlayerPlaybackNearlyFinished => "AudioPlayer.PlaybackNearlyFinished",
            RequestType::AudioPlayerPlaybackStopped => "AudioPlayer.PlaybackStopped",
            RequestType::AudioPlayerPlaybackFailed => "AudioPlayer.PlaybackFailed",
            RequestType::PlaybackControllerNextCommandIssued => {
                "PlaybackController.NextCommandIssued"
            }
            RequestType::PlaybackControllerPreviousCommandIssued => {
                "PlaybackController.PreviousCommandIssued"
            }
            RequestType::PlaybackControllerPlayCommandIssued => {
                "PlaybackController.PlayCommandIssued"
            }
            RequestType::PlaybackControllerPauseCommandIssued => {
                "PlaybackController.PauseCommandIssued"
            }
            RequestType::SessionEndedRequest => "SessionEndedRequest",
            RequestType::ConnectionsResponse => "Connections.Response",
            RequestType::CanFulfillIntentRequest => "CanFulfillIntentRequest",
            RequestType::DisplayElementSelected => "Display.ElementSelected",
            RequestType::SessionResumedRequest => "SessionResumedRequest",
        };
        write!(f, "{}", s)
    }
}
