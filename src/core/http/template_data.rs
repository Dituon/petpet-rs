use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PetpetData {
    pub key: String,
    pub avatar: AvatarDataURL,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AvatarDataURL {
    pub from: Option<String>,
    pub to: Option<String>,
    pub bot: Option<String>,
    pub group: Option<String>,
    pub random: Option<Vec<String>>,
}