use skia_safe::Paint;
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
    pub paint: Paint,
    pub align: Align,
}

impl TextBuilder {
    pub fn new(template: TextTemplate) -> Result<Self, Error> {
        let color = parse_color(&template.color)?;
        let mut paint = Paint::default();
        paint.set_color(color);
        let align = template.align.to_skia_align();

        Ok(TextBuilder {
            built_template: TextBuiltTemplate{
                raw: template,
                paint,
                align,
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