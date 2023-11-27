use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextPos {
    XY((i32, i32)),
    XYW((i32, i32, i32)),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextAlign {
    LEFT,
    RIGHT,
    CENTER
}

impl TextAlign {
    pub fn to_skia_align(&self) -> skia_safe::utils::text_utils::Align {
        match self {
            TextAlign::LEFT => skia_safe::utils::text_utils::Align::Left,
            TextAlign::RIGHT => skia_safe::utils::text_utils::Align::Right,
            TextAlign::CENTER => skia_safe::utils::text_utils::Align::Center
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextTemplate {
    pub text: String,
    pub pos: TextPos,

    #[serde(default = "size_default")]
    pub size: f32,
    #[serde(default = "align_default")]
    pub align: TextAlign,
    #[serde(default = "color_default")]
    pub color: String,
}

fn size_default() -> f32 {
    24.0
}

fn align_default() -> TextAlign {
    TextAlign::LEFT
}

fn color_default() -> String {
    "#ffffff".to_string()
}
