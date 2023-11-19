use std::sync::Arc;
use std::time::Duration;
use std::vec;

use once_cell::sync::Lazy;
use skia_safe::{Data, Image};

use crate::core::builder::avatar_builder::AvatarDataItem;
use crate::core::errors::Error::ImageDecodeError;

pub struct RequesterOptions<'a> {
    user_agent: &'a str,
    timeout: u64,
}

pub struct Requester {
    client: reqwest::Client,
}

pub(crate) static REQUESTER: Lazy<Requester> = Lazy::new(|| {
    Requester::new(&RequesterOptions {
        user_agent: "",
        timeout: 60000,
    })
});

impl Requester {
    pub fn new(opts: &RequesterOptions) -> Requester {
        let client = reqwest::Client::builder()
            .user_agent(opts.user_agent)
            .timeout(Duration::from_millis(opts.timeout))
            .build().ok().unwrap();

        Requester {
            client
        }
    }

    pub fn get_images(&self, url: reqwest::Url) -> AvatarDataItem {
        let future = Box::pin(async move {
            let blob = self.client.get(url).send().await?.bytes().await?;
            let data = Data::new_copy(blob.as_ref());
            let result = Image::from_encoded(data).ok_or(ImageDecodeError(""));
            match result {
                Ok(img) => Ok(Arc::new(vec![img])),
                Err(e) => Err(e)
            }
        });
        Some(future)
    }
}
