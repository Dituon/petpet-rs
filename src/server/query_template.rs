use serde::{Deserialize, Serialize};
use crate::core::template::text_template::TextData;
use crate::core::http::template_data::{AvatarDataURL, PetpetData};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParams {
    pub key: String,
    #[serde(rename = "fromAvatar")]
    pub from_avatar: Option<String>,
    #[serde(rename = "toAvatar")]
    pub to_avatar: Option<String>,
    #[serde(rename = "groupAvatar")]
    pub group_avatar: Option<String>,
    #[serde(rename = "botAvatar")]
    pub bot_avatar: Option<String>,
    #[serde(rename = "fromName", default = "name_default")]
    pub from_name: String,
    #[serde(rename = "toName", default = "name_default")]
    pub to_name: String,
    #[serde(rename = "groupName", default = "name_default")]
    pub group_name: String,
    #[serde(rename = "textList", default = "text_list_default")]
    pub text_list: String,
}

impl QueryParams {
    pub fn to_data(self) -> PetpetData {
        PetpetData {
            key: self.key,
            avatar: AvatarDataURL {
                from: self.from_avatar,
                to: self.to_avatar,
                bot: self.bot_avatar,
                group: self.group_avatar,
                random: None,
            },
            text: TextData {
                from: self.from_name,
                to: self.to_name,
                group: self.group_name,
                text_list: self.text_list.split_whitespace()
                    .map(|s| s.to_owned()).collect(),
            },
        }
    }
}

fn name_default() -> String {
    "default name".to_string()
}

fn text_list_default() -> String {
    "".to_string()
}