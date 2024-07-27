use alloc::borrow::Cow;

use once_cell::sync::Lazy;
use regex::Regex;
use skia_safe::{Canvas, FontMgr, Paint, Point};
use skia_safe::textlayout::{FontCollection, Paragraph, ParagraphBuilder, TextStyle};

use crate::core::builder::text_builder::TextBuiltTemplate;
use crate::core::template::petpet_template::TransformOrigin;
use crate::core::template::text_template::{TextAlign, TextData, TextPos, TextWrap};

static TEXT_VAR_REGEX: Lazy<Regex> = Lazy::new(||
    Regex::new(r#"\$txt([1-9]\d*)\[(.*?)]"#).unwrap()
);

pub struct TextModel<'a> {
    pub template: &'a TextBuiltTemplate,
    // Paragraph is neither Send nor Sync
    // <https://github.com/rust-skia/rust-skia/issues/537>
    // pub paragraph: Arc<RwLock<Paragraph>>,
    text: String,
}

impl<'a> TextModel<'a> {
    pub fn new(template: &'a TextBuiltTemplate, text_data: &'a TextData) -> Self {
        let mut text = template.raw.text.replace("$from", &text_data.from)
            .replace("$to", &text_data.to)
            .replace("$group", &text_data.group);

        let text_list_len = text_data.text_list.len();
        for cap in TEXT_VAR_REGEX.captures_iter(&text.clone()) {
            if let (Some(num), Some(content)) = (cap.get(1), cap.get(2)) {
                let i: usize = num.as_str().parse().unwrap_or_default();
                let replace_text: Cow<str> = if i > text_list_len {
                    Cow::Borrowed(content.as_str())
                } else {
                    Cow::Borrowed(&text_data.text_list[i - 1])
                };

                text = text.replacen(cap.get(0).unwrap().as_str(), &replace_text, 1);
            }
        }

        TextModel {
            template,
            text,
        }
    }

    pub fn draw(&self, canvas: &Canvas) {
        let (x, y, width) = match self.template.raw.pos {
            TextPos::XY((x, y)) =>
                (x, y, match self.template.raw.align {
                    TextAlign::LEFT => canvas.image_info().width() - x,
                    TextAlign::CENTER => canvas.image_info().width() / 2,
                    TextAlign::RIGHT => x,
                }),
            TextPos::XYW((x, y, w)) => (x, y, w),
        };
        let (mut fill_p, mut stroke_p) = self.build_paragraph(width as f32);

        if let TextWrap::ZOOM = self.template.raw.wrap {
            if let Some(mut p) = fill_p {
                p.layout(f32::MAX);
                let font_size = f32::floor(self.template.raw.size * (width as f32 / p.max_intrinsic_width()));
                fill_p = Some(single_paragraph(
                    &self.template, &self.text, font_size,
                    &self.template.fill_paint.as_ref().unwrap(), width as f32,
                ))
            }
            if let Some(mut p) = stroke_p {
                p.layout(f32::MAX);
                let font_size = f32::floor(self.template.raw.size * (width as f32 / p.max_intrinsic_width()));
                stroke_p = Some(single_paragraph(
                    &self.template, &self.text, font_size,
                    &self.template.stroke_paint.as_ref().unwrap(), width as f32,
                ))
            }
        }

        let has_angle = self.template.raw.angle != 0.0;
        if has_angle {
            canvas.save();
            let p = match self.template.raw.origin {
                TransformOrigin::DEFAULT => Point::from((x, y)),
                TransformOrigin::CENTER => Point::from((
                    x + width / 2,
                    y + if let Some(fp) = &fill_p {
                        fp.height()
                    } else {
                        stroke_p.as_ref().unwrap().height()
                    } as i32 / 2
                )),
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

    pub fn get_size(&self) -> (i32, i32) {
        let width = match self.template.raw.pos {
            TextPos::XY(_) => 200,
            TextPos::XYW((_, _, w)) => w,
        };
        let (fill_p, stroke_p) = self.build_paragraph(width as f32);
        let height = if fill_p.is_some() {
            fill_p.unwrap().height()
        } else {
            stroke_p.unwrap().height()
        } as i32;
        (width, height)
    }

    fn build_paragraph(&self, max_width: f32) -> (Option<Paragraph>, Option<Paragraph>) {
        let mut result = (None, None);
        if let Some(paint) = &self.template.fill_paint {
            result.0 = Some(single_paragraph(
                &self.template, &self.text, self.template.raw.size, paint, max_width,
            ))
        }
        if let Some(paint) = &self.template.stroke_paint {
            result.1 = Some(single_paragraph(
                &self.template, &self.text, self.template.raw.size, paint, max_width,
            ))
        }
        result
    }
}

fn single_paragraph(
    template: &TextBuiltTemplate,
    text: &str,
    size: f32,
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
    ts.set_font_size(size);
    ts.set_foreground_paint(paint);
    ts.set_font_style(template.raw.style.to_skia_text_style());
    paragraph_builder.push_style(&ts);
    paragraph_builder.add_text(text);
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(max_width);
    paragraph
}
