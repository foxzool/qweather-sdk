use serde::{Deserialize, Serialize};

/// RGBA颜色
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
