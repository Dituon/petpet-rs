use serde::{Deserialize, Serialize};

use crate::core::template::avatar_template::PosItem;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BackgroundTemplate {
    pub size: (PosItem, PosItem),
    #[serde(default = "color_default")]
    pub color: String,
    #[serde(default = "length_default")]
    pub length: u16,
}

fn color_default() -> String {
    "#00000000".to_string()
}

fn length_default() -> u16 {
    return 1
}