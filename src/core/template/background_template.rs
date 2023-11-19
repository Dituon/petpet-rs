use serde::{Deserialize, Serialize};

use crate::core::template::avatar_template::PosItem;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BackgroundTemplate {
    pub size: (PosItem, PosItem),
    pub color: String
}
