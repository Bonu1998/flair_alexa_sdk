use serde::{Deserialize, Serialize};
use std::fmt;

use super::play_behaviour::PlayBehaviour;

#[allow(dead_code)]
pub enum SpeechType {
    PlainText,
    SSML,
}

impl fmt::Display for SpeechType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            SpeechType::PlainText => "PlainText",
            SpeechType::SSML => "SSML",
        };
        write!(f, "{}", s)
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Speech {
    #[serde(rename = "type")]
    speech_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssml: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "playBehavior")]
    play_behaviour: Option<String>,
}

impl Speech {
    pub fn new(speech_type: String, text: Option<String>, ssml: Option<String>, play_behaviour: Option<String>)-> Speech{
        Speech{ speech_type, text, ssml, play_behaviour }
    }

    pub fn plain(content: String)-> Speech{
        Speech { speech_type: SpeechType::PlainText.to_string(), text: Some(content), ssml: None, play_behaviour: None }
    }

    pub fn ssml(content: String)-> Speech{
        Speech { speech_type: SpeechType::SSML.to_string(), text: None, ssml: Some(content), play_behaviour: None }
    }

    pub fn set_play_behaviour(mut self, pb: PlayBehaviour)-> Self{
        self.play_behaviour = Some(pb.to_string());
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reprompt {
    #[serde(rename = "outputSpeech")]
    pub output_speech: Speech,
}

impl Reprompt {
    pub fn new(rp_speech: Speech) -> Reprompt {
        Reprompt {
            output_speech: rp_speech,
        }
    }
}
