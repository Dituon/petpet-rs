use skia_safe::{Color, Image, Surface};

use crate::core::builder::pos_builder::{compile_size, CompiledSize, eval_background_size};
use crate::core::errors::Error;
use crate::core::errors::Error::TemplateError;
use crate::core::loader::image_loader::load_cached_background;
use crate::core::template::backgroud_template::BackgroundTemplate;

pub struct BackgroundBuilder<'a> {
    pub template: Option<(CompiledSize, Color)>,
    pub path: Option<&'a str>,
}

static EMPTY_VEC: Vec<Image> = vec![];

impl BackgroundBuilder<'_> {
   pub fn new(
       template: Option<BackgroundTemplate>,
       path: Option<&str>
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
                   Err(TemplateError("Background color error"))
               }
           },
           None => {
               if let Some(_) = path {
                   Ok(BackgroundBuilder {
                       template: None,
                       path
                   })
               } else {
                   Err(TemplateError("Can not found background file or config"))
               }
           }
       }
   }

    pub fn create_background(&self, avatar_sizes: Vec<OriginSize>) -> Result<(Surface, &Vec<Image>), Error> {
        let images = match self.path {
            Some(path) => {
                load_cached_background(path)?
            },
            None => &EMPTY_VEC
        };

        //TODO: cache surface
        let surface = match &self.template {
            Some((size, color)) => {
                //TODO: color
                let size = eval_background_size(size, avatar_sizes)?;
                skia_safe::surfaces::raster_n32_premul(size).unwrap()
            },
            None => {
                skia_safe::surfaces::raster_n32_premul((
                    images[0].width(),
                    images[0].height()
                )).unwrap()
            }
        };

        Ok((surface, images))
    }
}

pub type OriginSize = (i32, i32);
