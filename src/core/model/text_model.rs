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
        let face = skia_safe::typeface::Typeface::from_name("Arial", Default::default()).unwrap();
        let font = Font::from_typeface(face, self.template.raw.size);
        if let TextPos::XY((x, y)) = self.template.raw.pos {
            skia_safe::utils::text_utils::draw_str(
                canvas,
                &self.template.raw.text,
                Point::from((x, y)),
                &font,
                &self.template.paint,
                self.template.align
            );
        } else {
            panic!()
        }
        ()
    }
}

