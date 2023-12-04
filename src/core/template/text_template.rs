use core::fmt::Write;
use std::fmt;

use serde::{de, Deserialize, Deserializer, Serialize};
use serde::de::{SeqAccess, value, Visitor};
use skia_safe::Point;
use skia_safe::textlayout::Paragraph;

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
    pub fn to_skia_align(&self) -> skia_safe::textlayout::TextAlign {
        match self {
            TextAlign::LEFT => skia_safe::textlayout::TextAlign::Left,
            TextAlign::RIGHT => skia_safe::textlayout::TextAlign::Right,
            TextAlign::CENTER => skia_safe::textlayout::TextAlign::Center,
        }
    }

    pub fn get_by_pos(&self, paragraph: &Paragraph, (x, y): (i32, i32)) -> Point {
        Point::from(match self {
            TextAlign::LEFT => (x, y - paragraph.alphabetic_baseline() as i32),
            TextAlign::CENTER => (
                x - paragraph.max_width() as i32 / 2,
                y - paragraph.height() as i32 / 2
            ),
            TextAlign::RIGHT => (
                x - paragraph.max_width() as i32,
                y - paragraph.alphabetic_baseline() as i32
            ),
        })
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
    #[serde(default = "font_default", deserialize_with = "string_or_vec")]
    pub font: Vec<String>,
    #[serde(default = "style_default")]
    pub style: TextStyle,
    #[serde(rename = "strokeColor", default = "stroke_color_default")]
    pub stroke_color: String,
    #[serde(rename = "strokeSize", default = "stroke_size_default")]
    pub stroke_size: f32,
    #[serde(default = "origin_default")]
    pub origin: TransformOrigin,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TextData {
    #[serde(default = "from_default")]
    pub from: String,
    #[serde(default = "to_default")]
    pub to: String,
    #[serde(default = "group_default")]
    pub group: String,
    #[serde(rename = "textList", default = "text_list_default")]
    pub text_list: Vec<String>,
}

impl Default for TextData {
    fn default() -> Self {
        TextData {
            from: from_default(),
            to: to_default(),
            group: group_default(),
            text_list: text_list_default(),
        }
    }
}

fn string_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where D: Deserializer<'de>
{
    struct StringOrVec;

    impl<'de> Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where E: de::Error
        {
            Ok(vec![s.to_string()])
        }

        fn visit_seq<S>(self, seq: S) -> Result<Self::Value, S::Error>
            where S: SeqAccess<'de>
        {
            Deserialize::deserialize(value::SeqAccessDeserializer::new(seq))
        }
    }

    deserializer.deserialize_any(StringOrVec)
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

fn font_default() -> Vec<String> {
    vec!["Arial".to_string()]
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

fn from_default() -> String {
    "from".to_string()
}

fn to_default() -> String {
    "to".to_string()
}

fn group_default() -> String {
    "group".to_string()
}

fn text_list_default() -> Vec<String> {
    Vec::new()
}
