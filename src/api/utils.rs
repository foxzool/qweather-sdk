use serde::{Deserialize, Serialize};

/// RGBA颜色
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}


/// 元数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaData {
    /// 数据标签
    pub tag: String,
    /// 数据来源或提供商名字以及他们的声明，开发者必须将此内容与当前数据一起展示，可能为空
    pub sources: Vec<String>
}