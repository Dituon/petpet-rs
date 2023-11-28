use serde::{Deserialize, Serialize};

use crate::core::template::filter_template::AvatarFilter;
use crate::core::template::petpet_template::TransformOrigin;

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
pub enum AvatarType {
    FROM,
    TO,
    GROUP,
    BOT,
    RANDOM,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum AvatarPosType {
    ZOOM,
    DEFORM,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum AvatarCropType {
    NONE,
    PIXEL,
    PERCENT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum AvatarFit {
    CONTAIN,
    COVER,
    FILL,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize)]
pub enum AvatarStyle {
    MIRROR,
    FLIP,
    GRAY,
    BINARIZATION,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarTemplate {
    #[serde(rename = "type")]
    pub _type: AvatarType,
    pub pos: PosDimension,
    #[serde(rename = "posType", default = "pos_type_default")]
    pub pos_type: AvatarPosType,
    #[serde(default = "crop_default")]
    pub crop: Option<CropPos>,
    #[serde(rename = "cropType", default = "crop_type_default")]
    pub crop_type: AvatarCropType,
    #[serde(default = "style_default")]
    pub style: Vec<AvatarStyle>,
    #[serde(default = "filter_default")]
    pub filter: Vec<AvatarFilter>,
    #[serde(default = "fit_default")]
    pub fit: AvatarFit,
    #[serde(default = "round_default")]
    pub round: bool,
    #[serde(default = "rotate_default")]
    pub rotate: bool,
    #[serde(default = "origin_default")]
    pub origin: TransformOrigin,
    #[serde(rename = "avatarOnTop", default = "avatar_on_top_default")]
    pub avatar_on_top: bool,
    #[serde(default = "antialias_default")]
    pub antialias: bool,
    #[serde(default = "resampling_default")]
    pub resampling: bool,
    #[serde(default = "angle_default")]
    pub angle: f64,
    #[serde(default = "opacity_default")]
    pub opacity: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum PosItem {
    Expr(String),
    Num(i32)
}

pub trait PosLike {}
pub type P1D = Vec<PosItem>;
impl PosLike for P1D {}
pub type P2D = Vec<Vec<PosItem>>;
impl PosLike for P2D {}
pub type P3D = Vec<Vec<Vec<PosItem>>>;
impl PosLike for P3D {}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum PosDimension {
    P1D(P1D),
    P2D(P2D),
    P3D(P3D),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum CropPos {
    WH((f32, f32)),
    XYXY((f32, f32, f32, f32)),
}

fn pos_type_default() -> AvatarPosType {
    AvatarPosType::ZOOM
}

fn crop_default() -> Option<CropPos> {
    None
}

fn crop_type_default() -> AvatarCropType {
    AvatarCropType::NONE
}

fn style_default() -> Vec<AvatarStyle> {
    Vec::new()
}

fn filter_default() -> Vec<AvatarFilter> {
    Vec::new()
}

fn fit_default() -> AvatarFit {
    AvatarFit::FILL
}

fn round_default() -> bool {
    false
}

fn rotate_default() -> bool {
    false
}

fn origin_default() -> TransformOrigin {
    TransformOrigin::DEFAULT
}

fn avatar_on_top_default() -> bool {
    true
}

fn antialias_default() -> bool {
    true
}

fn resampling_default() -> bool {
    true
}

fn angle_default() -> f64 {
    0.0
}

fn opacity_default() -> f64 {
    1.0
}