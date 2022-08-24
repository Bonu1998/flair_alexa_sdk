use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[allow(dead_code)]
pub enum DirectiveType {
    AplRenderDocument,
    AplaRenderDocument,
    AplExecuteCommands,
    ConnectionsStartConnection,
}

impl fmt::Display for DirectiveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            DirectiveType::AplRenderDocument => "Alexa.Presentation.APL.RenderDocument",
            DirectiveType::AplExecuteCommands => "Alexa.Presentation.APL.ExecuteCommands",
            DirectiveType::ConnectionsStartConnection => "Connections.StartConnection",
            DirectiveType::AplaRenderDocument => "Alexa.Presentation.APLA.RenderDocument",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Directive {
    #[serde(rename = "type")]
    directive_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    document: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datasources: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sources: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commands: Option<Vec<JsonValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "onCompletion")]
    on_completion: Option<String>,
}

impl Directive {
    pub fn new(_type: String) -> Directive {
        Directive {
            directive_type: _type,
            token: None,
            document: None,
            datasources: None,
            sources: None,
            commands: None,
            uri: None,
            input: None,
            on_completion: None,
        }
    }

    pub fn apl_execute_commands(token: String, commands: Option<Vec<JsonValue>>) -> Directive {
        Directive {
            directive_type: DirectiveType::AplExecuteCommands.to_string(),
            token: Some(token),
            document: None,
            datasources: None,
            sources: None,
            commands,
            uri: None,
            input: None,
            on_completion: None,
        }
    }

    pub fn apl_render_document(
        token: String,
        document: Option<JsonValue>,
        datasources: Option<JsonValue>,
        sources: Option<JsonValue>,
    ) -> Directive {
        Directive {
            directive_type: DirectiveType::AplRenderDocument.to_string(),
            token: Some(token),
            document,
            datasources,
            sources,
            commands: None,
            uri: None,
            input: None,
            on_completion: None,
        }
    }

    pub fn apla_render_document(
        token: String,
        document: Option<JsonValue>,
        datasources: Option<JsonValue>,
        sources: Option<JsonValue>,
    ) -> Directive {
        Directive {
            directive_type: DirectiveType::AplRenderDocument.to_string(),
            token: Some(token),
            document,
            datasources,
            sources,
            commands: None,
            uri: None,
            input: None,
            on_completion: None,
        }
    }

    pub fn connection_start(
        uri: String,
        token: Option<String>,
        input: Option<JsonValue>,
        on_completion: Option<String>,
    ) -> Directive {
        Directive {
            directive_type: DirectiveType::ConnectionsStartConnection.to_string(),
            token,
            document: None,
            datasources: None,
            sources: None,
            commands: None,
            uri: Some(uri),
            input,
            on_completion,
        }
    }
}
