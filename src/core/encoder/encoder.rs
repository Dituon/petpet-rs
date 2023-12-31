use gif::{DisposalMethod, Frame, Repeat};
use once_cell::sync::Lazy;
use rayon::prelude::*;
use skia_safe::{EncodedImageFormat, Image};

use crate::core::errors::Error;
use crate::core::errors::Error::ImageEncodeError;

pub static IMAGE_ENCODER: Lazy<ImageEncoder> = Lazy::new(|| {
    ImageEncoder::new()
});

pub static PNG_FORMAT: &'static str = "png";
pub static GIF_FORMAT: &'static str = "gif";

pub enum EncodeFormat {
    PNG,
    GIF
}

impl EncodeFormat {
    pub fn get_str(&self) -> &'static str {
        match self {
            EncodeFormat::PNG => PNG_FORMAT,
            EncodeFormat::GIF => GIF_FORMAT,
        }
    }

    pub fn to_format(&self) -> String {
        format!("image/{}", self.get_str())
    }
}

pub struct ImageEncoder {
    // context: Option<DirectContext>,
    png_quality: u32,
    gif_quality: i32,
}

impl ImageEncoder {
    pub fn new() -> Self {
        // let context_options = ContextOptions::default();
        ImageEncoder {
            // context: DirectContext::new_gl(None, None),
            png_quality: 90,
            gif_quality: 10
        }
    }

    pub fn encode(&self, images: &Vec<Image>, delay: u16)
        -> Result<(Vec<u8>, EncodeFormat), Error>
    {
        if images.len() == 1 {
            Ok((self.encode_image(&images[0])?, EncodeFormat::PNG))
        } else {
            Ok((self.encode_images(images, delay)?, EncodeFormat::GIF))
        }
    }

    pub fn encode_image(&self, image: &Image) -> Result<Vec<u8>, Error> {
        let data = image.encode(
            // &self.context,
            // &DirectContext::new_gl(None, None),
            None,
            EncodedImageFormat::PNG,
            self.png_quality
        ).ok_or(ImageEncodeError("".to_string()))?;
        Ok(data.as_bytes().to_owned())
    }

    pub fn encode_images(&self, images: &Vec<Image>, delay: u16) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::with_capacity(65536);
        {
            let mut encoder = gif::Encoder::new(
                &mut bytes,
                images[0].width() as u16,
                images[0].height() as u16,
                &[]
            ).unwrap();
            encoder.set_repeat(Repeat::Infinite).or_else(|_| Err(ImageEncodeError("".to_string())))?;

            let frames: Vec<Frame> = images.par_iter().map(|img| {
                let map = img.peek_pixels().unwrap();
                let mut ps = map.bytes().unwrap().to_owned();

                let mut frame = Frame::from_rgba_speed(
                    img.width() as u16,
                    img.height() as u16,
                    &mut ps,
                    self.gif_quality,
                );
                frame.dispose = DisposalMethod::Background;
                frame.delay = delay;
                frame.make_lzw_pre_encoded();
                frame
            }).collect();
            for frame in frames {
                encoder.write_lzw_pre_encoded_frame(&frame).or_else(|_| Err(ImageEncodeError("".to_string())))?;
            }
        }

        Ok(bytes)
    }
}