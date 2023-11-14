use alloc::rc::Rc;
use std::borrow::Cow;
use std::sync::Arc;

use rand::Rng;
use skia_safe::{Canvas, Image, M44, Matrix, Paint, Point, Rect};

use crate::core::builder::avatar_builder::AvatarData;
use crate::core::builder::background_builder::OriginSize;
use crate::core::builder::pos_builder::{CompiledNumberPosDimension, CompiledPos, eval_size};
use crate::core::errors::Error;
use crate::core::errors::Error::{AvatarLoadError, TemplateError};
use crate::core::template::avatar_template::{AvatarTemplate, AvatarType};

pub struct AvatarModel<'a> {
    pub template: &'a AvatarTemplate,
    images: Arc<Vec<Image>>,
    pub pos: Cow<'a, CompiledNumberPosDimension>,
}

pub trait Drawable {
    fn draw(&self, index: u8);
}

impl<'a> AvatarModel<'a> {
    pub async fn new(
        template: &'a AvatarTemplate,
        data: Rc<AvatarData<'a>>,
        (num_pos, expr_pos): &'a CompiledPos
    ) -> Result<AvatarModel<'a>, Error<'a>> {
        let image_item = match &template._type {
            AvatarType::FROM => &data.from,
            AvatarType::TO => &data.to,
            AvatarType::GROUP => &data.group,
            AvatarType::BOT => &data.bot,
            AvatarType::RANDOM => {
                let vec = &data.random;
                let index = rand::thread_rng().gen_range(0..vec.len());
                &vec[index]
            }
        }.as_ref().unwrap();
        let image_item = Arc::clone(&image_item);
        let images = image_item.lock()?.as_mut().await?;

        if images.as_ref().is_empty() {
            return Err(AvatarLoadError(""))
        }

        if !expr_pos.is_empty() {
            let size = Self::get_image_size(&images[0]);
            let pos = eval_size((num_pos, expr_pos), size)?;
            return Ok(AvatarModel {
                template,
                images,
                pos: Cow::Owned(pos),
            })
        }

        Ok(AvatarModel {
            template,
            images,
            pos: Cow::Borrowed(num_pos),
        })
    }

    pub fn get_size(&self) -> OriginSize {
        Self::get_image_size(&self.images[0])
    }

    fn get_image_size(image: &Image) -> OriginSize {
        (image.width(), image.height())
    }

    pub fn draw(&self, canvas: &Canvas, index: usize) -> Result<(), Error<'a>> {
        match &self.pos.as_ref() {
            CompiledNumberPosDimension::P2D(p2d) => {
                let (x,y,w,h) = p2d[index];
                let rect = Rect::from_xywh(x as f32, y as f32, w as f32, h as f32);
                let img = &self.images.as_ref()[index];
                canvas.draw_image_rect(img, None, rect, &Paint::default());
            }
            CompiledNumberPosDimension::P3D(p3d) => {
                let img = &self.images.as_ref()[index];
                let m = Matrix::from_poly_to_poly(&[
                    Point::new(0.0, 0.0),
                    Point::new(0.0, img.height() as f32),
                    Point::new(img.width() as f32, img.height() as f32),
                    Point::new(img.width() as f32, 0.0),
                ], &p3d[index]).ok_or(TemplateError("can not build Matrix"))?;
                canvas.set_matrix(&(M44::from(m)));
                canvas.draw_image(img, (0, 0), None);
                canvas.reset_matrix();
            }
        };
        Ok(())
    }
}