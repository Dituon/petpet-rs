use std::sync::Arc;
use std::time::{Duration, Instant};
use std::vec;

use once_cell::sync::Lazy;
use skia_safe::{AlphaType, Codec, ColorType, Data, ImageInfo};
use skia_safe::codec::{EncodedImageFormat, Options, ZeroInitialized};

use crate::core::builder::avatar_builder::AvatarDataItem;

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
        Box::pin(async move {
            let time = Instant::now();
            let blob = self.client.get(url).send().await?.bytes().await?;
            println!("download: {:?}", time.elapsed());
            let data = Data::new_copy(blob.as_ref());
            let mut codec = Codec::from_data(data).unwrap();
            let mut delay: u16 = 6;
            let info = ImageInfo::new(
                codec.dimensions(),
                ColorType::RGBA8888,
                AlphaType::Premul,
                None,
            );
            let imgs = match codec.encoded_format() {
                EncodedImageFormat::GIF => {
                    let mut v = Vec::with_capacity(codec.get_frame_count());
                    let mut count = 0;
                    for i in 0..codec.get_frame_count() {
                        let frame_info = codec.get_frame_info(i);
                        if frame_info.is_some() {
                            delay += frame_info.unwrap().duration as u16;
                            count += 1;
                        }
                        v.push(codec.get_image(info.clone(), &Options {
                            zero_initialized: ZeroInitialized::Yes,
                            subset: None,
                            frame_index: i,
                            prior_frame: None,
                        })?)
                    }
                    delay /= count;
                    v
                }
                _ => {
                    vec![codec.get_image(info, None)?]
                }
            };

            Ok((Arc::new(imgs), delay))
        })
    }
}
