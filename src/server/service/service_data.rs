use serde::{Deserialize, Serialize};
use crate::core::http::template_data::AvatarDataURL;
use crate::core::template::text_template::TextData;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PetpetServiceData {
    pub key: String,
    #[serde(default = "AvatarDataURL::default")]
    pub avatar: AvatarDataURL,
    #[serde(default = "TextData::default")]
    pub text: TextData,
}