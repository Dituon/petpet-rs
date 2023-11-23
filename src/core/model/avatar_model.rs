use std::borrow::Cow;
use std::f32;
use std::sync::Arc;

use rayon::prelude::*;
use skia_safe::{Canvas, Image, M44, Matrix, Paint, Path, Point, Rect, scalar};
use skia_safe::canvas::SrcRectConstraint;
use crate::core::builder::avatar_builder::AvatarBuiltTemplate;

use crate::core::builder::background_builder::OriginSize;
use crate::core::builder::pos_builder::{CompiledNumberPosDimension, CompiledPos, eval_size, XYWH};
use crate::core::errors::Error;
use crate::core::errors::Error::{AvatarLoadError, TemplateError};
use crate::core::template::avatar_template::{AvatarCropType, AvatarFit, AvatarTemplate, CropPos, TransformOrigin};

pub struct AvatarModel<'a> {
    pub template: &'a AvatarBuiltTemplate,
    images: Arc<Vec<Image>>,
    pub pos: Cow<'a, CompiledNumberPosDimension>,

    src_rect: Option<Rect>,
}

pub trait Drawable {
    fn draw(&self, index: u8);
}

impl<'a> AvatarModel<'a> {
    pub fn new(
        template: &'a AvatarBuiltTemplate,
        images: Arc<Vec<Image>>,
    ) -> Result<AvatarModel<'a>, Error> {
        if images.as_ref().is_empty() {
            return Err(AvatarLoadError("avatars vec is empty".to_string()));
        }
        let (num_pos, expr_pos) = &template.pos;

        let built_images: Arc<Vec<Image>> = if template.raw.round {
            Arc::new(images.par_iter()
                .map(|img| Self::crop_to_circle(img)).collect()
            )
        } else { Arc::clone(&images) };

        let src_rect = match template.raw.crop_type {
            AvatarCropType::NONE => None,
            AvatarCropType::PIXEL => {
                if let CropPos::XYXY((x1, y1, x2, y2)) = template.raw.crop.as_ref().unwrap() {
                    Some(Rect::from_xywh(*x1, *y1, x2 - x1, y2 - y1))
                } else { None }
            }
            AvatarCropType::PERCENT => {
                if let CropPos::XYXY((x1, y1, x2, y2)) = template.raw.crop.as_ref().unwrap() {
                    let (sw, sh) = Self::get_image_size(&images[0]);
                    let x1 = (x1 / 100.0) * sw as f32;
                    let y1 = (y1 / 100.0) * sh as f32;
                    let x2 = (x2 / 100.0) * sw as f32;
                    let y2 = (y2 / 100.0) * sh as f32;
                    Some(Rect::from_xywh(x1, y1, x2 - x1, y2 - y1))
                } else { None }
            }
        };

        if !expr_pos.is_empty() {
            let size = Self::get_image_size(&images[0]);
            let pos = eval_size((num_pos, expr_pos), size)?;
            return Ok(AvatarModel {
                template,
                images: built_images,
                pos: Cow::Owned(pos),
                src_rect,
            })
        }

        Ok(AvatarModel {
            template,
            images: built_images,
            pos: Cow::Borrowed(num_pos),
            src_rect,
        })
    }

    pub fn get_src_rect(&self) -> Option<(&Rect, SrcRectConstraint)> {
        self.src_rect.as_ref().map(|rect| (rect, SrcRectConstraint::Fast))
    }

    pub fn get_size(&self) -> OriginSize {
        if let Some(rect) = self.src_rect {
            (rect.width() as i32, rect.height() as i32)
        } else {
            Self::get_image_size(&self.images[0])
        }
    }

    fn get_image_size(image: &Image) -> OriginSize {
        (image.width(), image.height())
    }

    fn get_image(&self, index: usize) -> &Image {
        return &self.images.as_ref()[index % self.images.len()];
    }

    pub fn draw(&self, canvas: &Canvas, index: usize) -> Result<(), Error> {
        match &self.pos.as_ref() {
            CompiledNumberPosDimension::P2D(p2d) => {
                let p2d = p2d[index % p2d.len()];
                let img = self.get_image(index);
                self.draw_zoom(canvas, img, p2d);
            }
            CompiledNumberPosDimension::P3D(p3d) => {
                let img = self.get_image(index);
                let m = Matrix::from_poly_to_poly(&[
                    Point::new(0.0, 0.0),
                    Point::new(0.0, img.height() as f32),
                    Point::new(img.width() as f32, img.height() as f32),
                    Point::new(img.width() as f32, 0.0),
                ], &p3d[index % p3d.len()]).ok_or(TemplateError(
                    format!("can not build Matrix, {:?}", &p3d[index % p3d.len()])
                ))?;
                canvas.set_matrix(&(M44::from(m)));
                canvas.draw_image(img, (0, 0), None);
                canvas.reset_matrix();
            }
        };
        Ok(())
    }

    fn draw_zoom(&self, canvas: &Canvas, img: &Image, (x, y, w, h): XYWH) {
        let mut paint = Paint::default();
        paint.set_alpha((self.template.raw.opacity * 255.0) as u8);

        let has_angle = self.template.raw.angle % 360.0 != 0.0;
        if has_angle {
            canvas.save();
            let p = match self.template.raw.origin {
                TransformOrigin::DEFAULT => Point::from((x, y)),
                TransformOrigin::CENTER => Point::from((x + w / 2, y + h / 2))
            };
            canvas.rotate(self.template.raw.angle as scalar, Some(p));
        }
        match self.template.raw.fit {
            AvatarFit::FILL => {
                let rect = Rect::from_xywh(x as f32, y as f32, w as f32, h as f32);
                canvas.draw_image_rect(img, self.get_src_rect(), rect, &paint);
            }
            AvatarFit::CONTAIN => {
                let iw = img.width() as f32;
                let ih = img.height() as f32;
                let scale = f32::min(w as f32 / iw, h as f32 / ih);

                let scaled_width = iw * scale;
                let scaled_height = ih * scale;
                let offset_x = x as f32 + (w as f32 - scaled_width) / 2.0;
                let offset_y = y as f32 + (h as f32 - scaled_height) / 2.0;

                let rect = Rect::from_xywh(offset_x, offset_y, scaled_width, scaled_height);
                canvas.draw_image_rect(img, self.get_src_rect(), rect, &paint);
            }
            AvatarFit::COVER => {
                let iw = img.width() as f32;
                let ih = img.height() as f32;
                let scale = f32::max(w as f32 / iw, h as f32 / ih);

                let scaled_width = iw * scale;
                let scaled_height = ih * scale;
                let offset_x = x as f32 + (w as f32 - scaled_width) / 2.0;
                let offset_y = y as f32 + (h as f32 - scaled_height) / 2.0;
                let dx = scaled_width - w as f32;
                let dy = scaled_height - h as f32;
                let pdx: f32 = dx / scale / 2.0;
                let pdy: f32 = dy / scale / 2.0;

                let src_rect = Rect::from_xywh(
                    offset_x,
                    offset_y,
                    scaled_width,
                    scaled_height,
                );
                let dst_rect = Rect::from_xywh(
                    pdx, pdy,
                    scaled_width, scaled_height,
                );
                canvas.draw_image_rect(
                    img,
                    Some((&src_rect, SrcRectConstraint::Strict)),
                    dst_rect,
                    &paint,
                );
            }
        }
        if has_angle {
            canvas.restore();
        }
    }

    fn crop_to_circle(image: &Image) -> Image {
        let mut surface = skia_safe::surfaces::raster_n32_premul((image.width(), image.height())).unwrap();
        let w = surface.width() as f32;
        let h = surface.height() as f32;
        let canvas = surface.canvas();
        let mut clip_path = Path::new();
        clip_path.add_circle(Point::new(image.width() as f32 / 2.0, image.height() as f32 / 2.0), image.width() as f32 / 2.0, None);
        canvas.clip_path(&clip_path, None, false);

        let dest_rect = Rect::from_xywh(0.0, 0.0, w, h);
        canvas.draw_image_rect(image, None, dest_rect, &Paint::default());
        surface.image_snapshot()
    }
}