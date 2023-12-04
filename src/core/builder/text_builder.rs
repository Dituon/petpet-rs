use skia_safe::{Paint, PaintStyle};
use skia_safe::textlayout::ParagraphStyle;

use crate::core::errors::Error;
use crate::core::loader::color_util::parse_color;
use crate::core::model::text_model::TextModel;
use crate::core::template::text_template::{TextData, TextTemplate};

pub struct TextBuilder {
    pub built_template: TextBuiltTemplate,
}

pub struct TextBuiltTemplate {
    pub raw: TextTemplate,
    pub fill_paint: Option<Paint>,
    pub stroke_paint: Option<Paint>,
    pub paragraph_style: ParagraphStyle,
}

impl TextBuilder {
    pub fn new(mut template: TextTemplate) -> Result<Self, Error> {
        template.angle %= 360.0;
        let fill_color = parse_color(&template.color)?;
        let fill_paint = if fill_color.a() != 0 {
            let mut paint = Paint::default();
            paint.set_color(fill_color);
            paint.set_style(PaintStyle::StrokeAndFill);
            Some(paint)
        } else {
            None
        };
        let stroke_paint = if template.stroke_size != 0.0 {
            let stroke_color = parse_color(&template.stroke_color)?;
            let mut paint = Paint::default();
            paint.set_color(stroke_color);
            paint.set_style(PaintStyle::Stroke);
            paint.set_stroke(true);
            paint.set_stroke_width(template.stroke_size / 2.0);
            Some(paint)
        } else {
            None
        };
        let mut paragraph_style = ParagraphStyle::new();
        paragraph_style.set_text_align(template.align.to_skia_align());

        Ok(TextBuilder {
            built_template: TextBuiltTemplate {
                raw: template,
                fill_paint,
                stroke_paint,
                paragraph_style,
            },
        })
    }

    pub fn build<'a>(&'a self, data: &'a TextData) -> TextModel {
        TextModel::new(&self.built_template, data)
    }
}

pub struct TextBuilderList {
    pub builders: Vec<TextBuilder>
}

impl TextBuilderList {
    pub fn new(templates: Vec<TextTemplate>) -> Result<Self, Error> {
        let mut builders = Vec::with_capacity(templates.len());
        for template in templates {
            builders.push(TextBuilder::new(template)?)
        }

        Ok(TextBuilderList {
            builders
        })
    }

    pub fn build<'a>(&'a self, text_data: &'a TextData) -> Result<Vec<TextModel>, Error> {
        Ok(
            self.builders.iter()
                .map(|builder| builder.build(text_data))
                .collect()
        )
    }
}