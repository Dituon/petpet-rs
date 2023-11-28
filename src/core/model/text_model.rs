use skia_safe::{Canvas, Font, Point};

use crate::core::builder::text_builder::TextBuiltTemplate;
use crate::core::template::petpet_template::TransformOrigin;
use crate::core::template::text_template::TextPos;

pub struct TextModel<'a> {
    pub template: &'a TextBuiltTemplate,
}

impl<'a> TextModel<'a> {
    pub fn new(template: &'a TextBuiltTemplate) -> Self {
        TextModel {
            template
        }
    }

    pub fn draw(&self, canvas: &Canvas) {
        let font = Font::from_typeface(&self.template.typeface, self.template.raw.size);

        if let TextPos::XY((x, y)) = self.template.raw.pos {
            let has_angle = self.template.raw.angle != 0.0;
            if has_angle {
                canvas.save();
                let p = match self.template.raw.origin {
                    TransformOrigin::DEFAULT => Point::from((x, y)),
                    TransformOrigin::CENTER => todo!(),
                };
                canvas.rotate(self.template.raw.angle, Some(p));
            }

            if let Some(fill_paint) = &self.template.fill_paint {
                skia_safe::utils::text_utils::draw_str(
                    canvas,
                    &self.template.raw.text,
                    Point::from((x, y)),
                    &font,
                    fill_paint,
                    //see https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/html/canvas/text_metrics.cc
                    self.template.align
                );
            }
            if let Some(stroke_paint) = &self.template.stroke_paint {
                skia_safe::utils::text_utils::draw_str(
                    canvas,
                    &self.template.raw.text,
                    Point::from((x, y)),
                    &font,
                    stroke_paint,
                    self.template.align
                );
            }

            if has_angle {
                canvas.restore();
            }
        } else {
            todo!()
        }
        ()
    }
}

