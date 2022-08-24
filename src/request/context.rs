use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::properties::{Application, Person, Unit, User};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Context {
    #[serde(rename = "AudioPlayer")]
    pub audio_player_details: Option<HashMap<String, String>>,
    #[serde(rename = "Alexa.Presentation.APL")]
    pub apl_context: Option<AplContext>,
    #[serde(rename = "System")]
    pub system: Option<System>,
    #[serde(rename = "Viewport")]
    pub viewport: Option<Viewport>,
    #[serde(rename = "Viewports")]
    pub viewports: Option<Vec<Viewport>>,
    #[serde(rename = "Extensions")]
    pub extensions: Option<Extensions>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Extensions {
    pub available: Option<HashMap<String, Option<HashMap<String, String>>>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct System {
    #[serde(rename = "apiAccessToken")]
    pub api_access_token: String,
    #[serde(rename = "apiEndpoint")]
    pub api_endpoint: String,
    pub application: Option<Application>,
    pub device: Option<Device>,
    pub unit: Option<Unit>,
    pub person: Option<Person>,
    pub user: Option<User>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Device {
    #[serde(rename = "deviceId")]
    pub device_id: String,
    #[serde(rename = "supportedInterfaces")]
    pub supported_interfaces: SupportedInterfaces,
    #[serde(rename = "persistentEndpointId")]
    pub persistent_endpoint_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SupportedInterfaces {
    #[serde(rename = "AudioPlayer")]
    pub audio_player_support: Option<HashMap<String, String>>,
    #[serde(rename = "Alexa.Presentation.APL")]
    pub apl_support: Option<AplSupport>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Experience {
    #[serde(rename = "canRotate")]
    pub can_rotate: Option<bool>,
    #[serde(rename = "canResize")]
    pub can_resize: Option<bool>,
    #[serde(rename = "arcMinuteWidth")]
    pub arc_minute_width: Option<i32>,
    #[serde(rename = "arcMinuteHeight")]
    pub arc_minute_height: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Viewport {
    #[serde(rename = "presentationType")]
    pub presentation_type: Option<String>,
    #[serde(rename = "type")]
    pub viewport_type: Option<String>,
    pub experiences: Option<Vec<Experience>>,
    pub mode: Option<String>,
    pub shape: Option<String>,
    #[serde(rename = "canRotate")]
    pub can_rotate: Option<bool>,
    #[serde(rename = "canResize")]
    pub can_resize: Option<bool>,
    #[serde(rename = "arcMinuteWidth")]
    pub arc_minute_width: Option<i32>,
    #[serde(rename = "arcMinuteHeight")]
    pub arc_minute_height: Option<i32>,
    #[serde(rename = "pixelWidth")]
    pub pixel_width: Option<i32>,
    #[serde(rename = "pixelHeight")]
    pub pixel_height: Option<i32>,
    pub dpi: Option<i32>,
    #[serde(rename = "currentPixelWidth")]
    pub current_pixel_width: Option<i32>,
    #[serde(rename = "currentPixelHeight")]
    pub current_pixel_height: Option<i32>,
    pub keyboard: Option<String>,
    pub touch: Option<Vec<String>>,
    pub video: Option<ViewportVideo>,
    pub size: Option<ViewportSize>,
    pub configuration: Option<ViewportConfiguration>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ViewportConfiguration {
    pub current: Option<Box<Viewport>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ViewportVideo {
    pub codecs: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ViewportSize {
    #[serde(rename = "type")]
    pub size_type: Option<String>,
    #[serde(rename = "pixelWidth")]
    pub pixel_width: Option<i32>,
    #[serde(rename = "pixelHeight")]
    pub pixel_height: Option<i32>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AplSupport {
    pub runtime: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AplContext {
    pub token: String,
    pub version: String,
    #[serde(rename = "componentsVisibleOnScreen")]
    pub components_visible_on_screen: Option<Vec<VisibleComponent>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisibleComponent {
    pub uid: String,
    pub position: String,
    #[serde(rename = "type")]
    pub visible_component_type: String,
    pub tags: Option<VisibleComponentTags>,
    pub children: Option<Vec<VisibleComponent>>,
    pub entities: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisibleComponentTags {
    pub viewport: Option<HashMap<String, String>>,
    pub clickable: Option<bool>,
    pub focused: Option<bool>,
}
