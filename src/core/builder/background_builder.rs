use skia_safe::{AlphaType, Color, ColorTable, ColorType, Image, ImageInfo, Surface};

use crate::core::builder::pos_builder::{compile_size, CompiledSize, eval_background_size};
use crate::core::errors::Error;
use crate::core::errors::Error::TemplateError;
use crate::core::loader::image_loader::load_cached_background;
use crate::core::template::background_template::BackgroundTemplate;

pub struct BackgroundBuilder {
    pub template: Option<(CompiledSize, Color)>,
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
                if let Ok(color_u32) = u32::from_str_radix(&template.color, 16) {
                    Ok(BackgroundBuilder {
                        template: Some((
                            compile_size(&template.size),
                            Color::from(color_u32)
                        )),
                        path,
                    })
                } else {
                    Err(TemplateError(
                        format!("Background color error: {}", template.color)
                    ))
                }
            }
            None => {
                if let Some(_) = path {
                    Ok(BackgroundBuilder {
                        template: None,
                        path,
                    })
                } else {
                    Err(TemplateError("Can not found background file or config".to_string()))
                }
            }
        }
    }

    pub fn create_background(&self, avatar_sizes: Vec<OriginSize>) -> Result<(Surface, &Vec<Image>), Error> {
        let images = match &self.path {
            Some(path) => {
                load_cached_background(path)?
            }
            None => &EMPTY_VEC
        };

        let size = match &self.template {
            Some((size, _color)) => eval_background_size(size, avatar_sizes)?,
            None => (images[0].width(), images[0].height())
        };
        let info = ImageInfo::new(
            size,
            ColorType::RGBA8888,
            AlphaType::Premul,
            None,
        );

        //TODO: cache surface
        let surface = match &self.template {
            Some((_, _color)) => {
                //TODO: color
                skia_safe::surfaces::raster(&info, 0, None).unwrap()
            }
            None => {
                skia_safe::surfaces::raster(&info, 0, None).unwrap()
            }
        };

        Ok((surface, images))
    }
}

pub type OriginSize = (i32, i32);
