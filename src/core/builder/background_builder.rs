use alloc::borrow::Cow;
use rayon::prelude::*;
use skia_safe::{AlphaType, Color, ColorType, Image, ImageInfo, Surface};

use crate::core::builder::pos_builder::{compile_size, CompiledSize, eval_background_size};
use crate::core::errors::Error;
use crate::core::errors::Error::TemplateError;
use crate::core::loader::color_util::parse_color;
use crate::core::loader::image_loader::{image_count, load_cached_background};
use crate::core::template::background_template::BackgroundTemplate;

pub struct BackgroundBuilder {
    pub info: Option<(CompiledSize, Color)>,
    pub length: usize,
    pub path: Option<String>,
}

static EMPTY_VEC: Vec<Image> = vec![];

impl BackgroundBuilder {
    pub fn new(
        template: Option<BackgroundTemplate>,
        path: Option<String>,
    ) -> Result<BackgroundBuilder, Error> {
        match template {
            Some(template) => {
                Ok(BackgroundBuilder {
                    info: Some((
                        compile_size(&template.size),
                        parse_color(&template.color)?
                    )),
                    length: template.length as usize,
                    path,
                })
            }
            None => {
                if let Some(p) = path {
                    Ok(BackgroundBuilder {
                        info: None,
                        length: image_count(&p),
                        path: Some(p),
                    })
                } else {
                    Err(TemplateError("Can not found background file or config".to_string()))
                }
            }
        }
    }

    pub fn create_background(&self, avatar_sizes: Vec<OriginSize>) -> Result<(Surface, Cow<Vec<Image>>), Error> {
        let file_images = match &self.path {
            Some(path) => {
                load_cached_background(path)?
            }
            None => &EMPTY_VEC
        };
        let size = match &self.info {
            Some((size, _)) => eval_background_size(size, avatar_sizes)?,
            None => (file_images[0].width(), file_images[0].height())
        };
        let info = ImageInfo::new(
            size,
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );

        //TODO: cache surface
        Ok(match &self.info {
            Some((_, color)) => {
                let mut s = skia_safe::surfaces::raster(&info, 0, None).unwrap();
                let mut images = Vec::with_capacity(self.length);
                if self.path.is_none() {
                    for _ in 0..self.length {
                        s.canvas().clear(*color);
                        images.push(s.image_snapshot());
                    }
                    return Ok((s, Cow::Owned(images)));
                }
                (s, Cow::Borrowed(file_images))
            }
            None => (
                skia_safe::surfaces::raster(&info, 0, None).unwrap(),
                Cow::Borrowed(file_images)
            )
        })
    }

    pub fn repeat_for_avatar_length(bgs: Cow<Vec<Image>>, avatar_length: usize) -> Cow<Vec<Image>>{
        if bgs.len() > 1 || avatar_length <= 1 {
            return bgs
        }

        let new_bgs = (0..avatar_length)
            .collect::<Vec<_>>().par_iter()
            // TODO: Cow Image
            .map(|_| bgs[0].clone())
            .collect();
        Cow::Owned(new_bgs)
    }
}

pub type OriginSize = (i32, i32);
