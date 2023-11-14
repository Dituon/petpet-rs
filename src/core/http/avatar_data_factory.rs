use std::str::FromStr;

use crate::core::builder::avatar_builder::{AvatarData, AvatarDataItem};
use crate::core::http::requester::REQUESTER;

pub struct AvatarDataURL<'a> {
    pub from: Option<&'a str>,
    pub to: Option<&'a str>,
    pub bot: Option<&'a str>,
    pub group: Option<&'a str>,
    pub random: Vec<Option<&'a str>>,
}

fn create_avatar_data_item<'a>(url_str: &Option<&str>) -> Result<AvatarDataItem<'a>, url::ParseError> {
    match url_str {
        None => Ok(None),
        Some(url) => {
            let parsed_url = reqwest::Url::from_str(url)?;
            let image = REQUESTER.get_images(parsed_url);
            Ok(image)
        }
    }
}


pub fn create_avatar_data<'a>(data_url: &AvatarDataURL) -> Result<AvatarData<'a>, url::ParseError> {
    Ok(AvatarData {
        from: create_avatar_data_item(&data_url.from)?,
        to: create_avatar_data_item(&data_url.to)?,
        bot: create_avatar_data_item(&data_url.bot)?,
        group: create_avatar_data_item(&data_url.group)?,
        random: data_url.random.iter()
            .map(|url| create_avatar_data_item(url).ok()?).collect(),
    })
}