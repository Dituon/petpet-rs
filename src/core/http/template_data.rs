use serde::{Deserialize, Serialize};
use crate::core::template::text_template::TextData;

#[derive(Debug, Deserialize, Serialize)]
pub struct PetpetData {
    pub key: String,
    #[serde(default = "AvatarDataURL::default")]
    pub avatar: AvatarDataURL,
    #[serde(default = "TextData::default")]
    pub text: TextData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AvatarDataURL {
    pub from: Option<String>,
    pub to: Option<String>,
    pub bot: Option<String>,
    pub group: Option<String>,
    pub random: Option<Vec<String>>,
}

impl Default for AvatarDataURL {
    fn default() -> Self {
        AvatarDataURL {
            from: None,
            to: None,
            bot: None,
            group: None,
            random: None,
        }
    }
}