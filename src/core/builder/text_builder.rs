use skia_safe::{Paint, Typeface};
use skia_safe::utils::text_utils::Align;

use crate::core::errors::Error;
use crate::core::loader::color_util::parse_color;
use crate::core::model::text_model::TextModel;
use crate::core::template::text_template::TextTemplate;

pub struct TextBuilder {
    pub built_template: TextBuiltTemplate,
}

pub struct TextBuiltTemplate {
    pub raw: TextTemplate,
    pub fill_paint: Option<Paint>,
    pub stroke_paint: Option<Paint>,
    pub align: Align,
    pub typeface: Typeface,
}

impl TextBuilder {
    pub fn new(mut template: TextTemplate) -> Result<Self, Error> {
        template.angle %= 360.0;
        let fill_color = parse_color(&template.color)?;
        let fill_paint = if fill_color.a() != 0 {
            let mut paint = Paint::default();
            paint.set_color(fill_color);
            Some(paint)
        } else {
            None
        };
        let stroke_paint = if template.stroke_size != 0.0 {
            let stroke_color = parse_color(&template.stroke_color)?;
            let mut paint = Paint::default();
            paint.set_color(stroke_color);
            paint.set_stroke(true);
            paint.set_stroke_width(template.stroke_size);
            Some(paint)
        } else {
            None
        };
        let align = template.align.to_skia_align();
        let typeface = Typeface::new(
            &template.font,
            template.style.to_skia_text_style()
        ).unwrap();

        Ok(TextBuilder {
            built_template: TextBuiltTemplate{
                raw: template,
                fill_paint,
                stroke_paint,
                align,
                typeface,
            },
        })
    }

    pub fn build(&self) -> TextModel {
        TextModel::new(&self.built_template)
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

    pub fn build(&self) -> Result<Vec<TextModel>, Error> {
        Ok(
            self.builders.iter()
                .map(|builder| builder.build())
                .collect()
        )
    }
}