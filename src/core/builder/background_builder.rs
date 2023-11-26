use alloc::borrow::Cow;

use skia_safe::{AlphaType, Color, ColorType, Image, ImageInfo, Surface};

use crate::core::builder::pos_builder::{compile_size, CompiledSize, eval_background_size};
use crate::core::errors::Error;
use crate::core::errors::Error::TemplateError;
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
                if let Ok(color_u32) = u32::from_str_radix(&template.color[1..], 16) {
                    Ok(BackgroundBuilder {
                        info: Some((
                            compile_size(&template.size),
                            Color::from(color_u32)
                        )),
                        length: template.length as usize,
                        path,
                    })
                } else {
                    Err(TemplateError(
                        format!("Background color error: {}", template.color)
                    ))
                }
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
                let mut images = vec![];
                if self.path.is_none() {
                    for _ in 0..self.length {
                        s.canvas().clear(*color);
                        images.push(s.image_snapshot());
                    }
                    return Ok((s, Cow::Owned(images)))
                }
                (s, Cow::Borrowed(file_images))
            }
            None => (
                skia_safe::surfaces::raster(&info, 0, None).unwrap(),
                Cow::Borrowed(file_images)
            )
        })
    }
}

pub type OriginSize = (i32, i32);
