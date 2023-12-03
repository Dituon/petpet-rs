use skia_safe::{Canvas, FontMgr, Paint, Point};
use skia_safe::textlayout::{FontCollection, Paragraph, ParagraphBuilder, TextStyle};

use crate::core::builder::text_builder::TextBuiltTemplate;
use crate::core::template::petpet_template::TransformOrigin;
use crate::core::template::text_template::TextPos;

pub struct TextModel<'a> {
    pub template: &'a TextBuiltTemplate,
    // Paragraph is neither Send nor Sync
    // <https://github.com/rust-skia/rust-skia/issues/537>
    // pub paragraph: Arc<RwLock<Paragraph>>,
}

impl<'a> TextModel<'a> {
    pub fn new(template: &'a TextBuiltTemplate) -> Self {
        TextModel {
            template,
        }
    }

    pub fn draw(&self, canvas: &Canvas) {
        let (x, y, width) = match self.template.raw.pos {
            TextPos::XY((x, y)) => (x, y, canvas.image_info().width() / 2),
            TextPos::XYW((x, y, w)) => (x, y, w),
        };
        let (fill_p, stroke_p) =
            Self::build_paragraph(self.template, width as f32);

        let has_angle = self.template.raw.angle != 0.0;
        if has_angle {
            canvas.save();
            let p = match self.template.raw.origin {
                TransformOrigin::DEFAULT => Point::from((x, y)),
                TransformOrigin::CENTER => todo!(),
            };
            canvas.rotate(self.template.raw.angle, Some(p));
        }

        if let Some(p) = fill_p {
            p.paint(canvas, self.template.raw.align.get_by_pos(&p, (x, y)));
        }
        if let Some(p) = stroke_p {
            p.paint(canvas, self.template.raw.align.get_by_pos(&p, (x, y)));
        }

        if has_angle {
            canvas.restore();
        }
        ()
    }

    fn build_paragraph(template: &TextBuiltTemplate, max_width: f32) -> (Option<Paragraph>, Option<Paragraph>) {
        let mut result = (None, None);
        if let Some(paint) = &template.fill_paint {
            result.0 = Some(single_paragraph(template, paint, max_width))
        }
        if let Some(paint) = &template.stroke_paint {
            result.1 = Some(single_paragraph(template, paint, max_width))
        }
        result
    }
}

fn single_paragraph(
    template: &TextBuiltTemplate,
    paint: &Paint,
    max_width: f32,
) -> Paragraph {
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager_and_family_names(
        FontMgr::default(),
        //TODO: check font
        &template.raw.font,
    );
    let mut paragraph_builder = ParagraphBuilder::new(
        &template.paragraph_style, font_collection,
    );
    let mut ts = TextStyle::new();
    ts.set_font_size(template.raw.size);
    ts.set_foreground_paint(paint);
    ts.set_font_style(template.raw.style.to_skia_text_style());
    paragraph_builder.push_style(&ts);
    paragraph_builder.add_text(&template.raw.text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(max_width);
    paragraph
}

