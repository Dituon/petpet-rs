use serde::{Deserialize, Serialize};
use crate::core::template::petpet_template::TransformOrigin;

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
    CENTER,
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

#[deny(warnings)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextStyle {
    PLAIN,
    BOLD,
    ITALIC,
    #[allow(non_camel_case_types)]
    BOLD_ITALIC,
}

impl TextStyle {
    pub fn to_skia_text_style(&self) -> skia_safe::font_style::FontStyle {
        match self {
            TextStyle::PLAIN => skia_safe::font_style::FontStyle::normal(),
            TextStyle::BOLD => skia_safe::font_style::FontStyle::bold(),
            TextStyle::ITALIC => skia_safe::font_style::FontStyle::italic(),
            TextStyle::BOLD_ITALIC => skia_safe::font_style::FontStyle::bold_italic()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextTemplate {
    pub text: String,
    pub pos: TextPos,
    #[serde(default = "size_default")]
    pub size: f32,
    #[serde(default = "angle_default")]
    pub angle: f32,
    #[serde(default = "align_default")]
    pub align: TextAlign,
    #[serde(default = "color_default")]
    pub color: String,
    #[serde(default = "font_default")]
    pub font: String,
    #[serde(default = "style_default")]
    pub style: TextStyle,
    #[serde(rename = "strokeColor", default = "stroke_color_default")]
    pub stroke_color: String,
    #[serde(rename = "strokeSize", default = "stroke_size_default")]
    pub stroke_size: f32,
    #[serde(default = "origin_default")]
    pub origin: TransformOrigin,
}

fn size_default() -> f32 {
    24.0
}

fn angle_default() -> f32 {
    0.0
}

fn align_default() -> TextAlign {
    TextAlign::LEFT
}

fn color_default() -> String {
    "#ffffff".to_string()
}

fn font_default() -> String {
    "Arial".to_string()
}

fn style_default() -> TextStyle {
    TextStyle::PLAIN
}

fn stroke_color_default() -> String {
    "#ffffff".to_string()
}

fn stroke_size_default() -> f32 {
    0.0
}

fn origin_default() -> TransformOrigin {
    TransformOrigin::DEFAULT
}
