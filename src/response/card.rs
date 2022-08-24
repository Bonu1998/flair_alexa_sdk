use std::fmt;

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub enum CardType {
    Simple,
    Standard,
    LinkAccount,
    AskForPermission,
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            CardType::Simple => "Simple",
            CardType::Standard => "Standard",
            CardType::LinkAccount => "LinkAccount",
            CardType::AskForPermission => "AskForPermissonConsent",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    #[serde(rename = "type")]
    card_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<CardImage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<Vec<String>>,
}

impl Card {
    pub fn new(
        card_type: CardType,
        title: Option<String>,
        text: Option<String>,
        content: Option<String>,
        image: Option<CardImage>,
        permissions: Option<Vec<String>>,
    ) -> Card {
        Card {
            card_type: card_type.to_string(),
            title,
            content,
            image,
            text,
            permissions,
        }
    }

    pub fn simple(title: String, content: String) -> Card {
        Card {
            card_type: CardType::Simple.to_string(),
            title: Some(title),
            text: None,
            content: Some(content),
            image: None,
            permissions: None,
        }
    }

    pub fn standard(
        title: String,
        text: String,
        small_image_url: Option<String>,
        large_image_url: Option<String>,
    ) -> Card {
        Card {
            card_type: CardType::Standard.to_string(),
            title: Some(title),
            text: Some(text),
            content: None,
            image: Some(CardImage {
                small_image_url,
                large_image_url,
            }),
            permissions: None,
        }
    }

    pub fn account_link() -> Card {
        Card {
            card_type: CardType::LinkAccount.to_string(),
            title: None,
            text: None,
            content: None,
            image: None,
            permissions: None,
        }
    }

    pub fn permissions(permissions: Vec<String>) -> Card {
        Card {
            card_type: CardType::AskForPermission.to_string(),
            title: None,
            text: None,
            content: None,
            permissions: Some(permissions),
            image: None,
        }
    }    
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "smallImageUrl")]
    small_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "largeImageUrl")]
    large_image_url: Option<String>,
}

impl CardImage {
    pub fn new(siu: Option<String>, liu: Option<String>) -> CardImage {
        CardImage {
            small_image_url: siu,
            large_image_url: liu,
        }
    }
}
