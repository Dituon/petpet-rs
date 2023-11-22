use std::str::FromStr;

use crate::core::builder::avatar_builder::{AvatarData, AvatarDataItem};
use crate::core::errors::Error;
use crate::core::http::requester::REQUESTER;
use crate::core::http::template_data::AvatarDataURL;

fn create_avatar_data_item<'a>(url_str: &Option<String>) -> Result<Option<AvatarDataItem<'a>>, Error> {
    match url_str {
        None => Ok(None),
        Some(url) => {
            let parsed_url = reqwest::Url::from_str(url)?;
            let image = REQUESTER.get_images(parsed_url);
            Ok(Some(image))
        }
    }
}


pub fn create_avatar_data(data_url: &AvatarDataURL) -> Result<AvatarData, Error> {
    let random_vec = if data_url.random.is_none() {
        Vec::with_capacity(0)
    } else {
        let random = data_url.random.as_ref().unwrap();
        let mut v = Vec::with_capacity(random.len());
        for random_url in random {
            v.push(create_avatar_data_item(&Some(random_url.clone()))?.unwrap())
        }
        v
    };

    Ok(AvatarData {
        from: create_avatar_data_item(&data_url.from)?,
        to: create_avatar_data_item(&data_url.to)?,
        bot: create_avatar_data_item(&data_url.bot)?,
        group: create_avatar_data_item(&data_url.group)?,
        random: random_vec,
    })
}