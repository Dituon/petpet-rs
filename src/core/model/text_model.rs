use skia_safe::{Canvas, Font, Point};

use crate::core::builder::text_builder::TextBuiltTemplate;
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
            skia_safe::utils::text_utils::draw_str(
                canvas,
                &self.template.raw.text,
                Point::from((x, y)),
                &font,
                &self.template.paint,
                //see https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/renderer/core/html/canvas/text_metrics.cc
                self.template.align
            );
        } else {
            panic!()
        }
        ()
    }
}

