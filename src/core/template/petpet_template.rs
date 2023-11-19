use serde::{Deserialize, Serialize};

use crate::core::template::avatar_template::AvatarTemplate;
use crate::core::template::background_template::BackgroundTemplate;

#[derive(Debug, Deserialize, Serialize)]
pub enum PetpetType {
    IMG,
    GIF,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PetpetTemplate {
    #[serde(rename = "type")]
    pub _type: PetpetType,
    pub avatar: Vec<AvatarTemplate>,
    // text: Vec<TextTemplate>,
    #[serde(default = "background_default")]
    pub background: Option<BackgroundTemplate>,
    #[serde(default = "delay_default")]
    pub delay: u32,
    #[serde(default = "alias_default")]
    pub alias: Vec<String>,
    #[serde(default = "in_random_list_default", rename = "inRandomList")]
    pub in_random_list: bool,
    #[serde(default = "reverse_default")]
    pub reverse: bool,
    #[serde(default = "hidden_default")]
    pub hidden: bool,
}

fn background_default() -> Option<BackgroundTemplate> {
    None
}

fn delay_default() -> u32 {
    65
}

fn alias_default() -> Vec<String> {
    Vec::new()
}

fn in_random_list_default() -> bool {
    true
}

fn reverse_default() -> bool {
    false
}

fn hidden_default() -> bool {
    false
}
