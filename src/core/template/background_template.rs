use serde::{Deserialize, Serialize};

use crate::core::template::avatar_template::PosItem;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BackgroundTemplate {
    pub size: (PosItem, PosItem),
    #[serde(default = "color_default")]
    pub color: String
}

fn color_default() -> String {
    "#000000".to_string()
}
