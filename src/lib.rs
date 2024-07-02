#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]

/// GEO API URL
pub static GEO_API_URL: &str = "https://geoapi.qweather.com";

/// 标准订阅 API URL
pub static WEATHER_API_URL: &str = "https://api.qweather.com";

/// 免费订阅 API URL
pub static WEATHER_DEV_API_URL: &str = "https://devapi.qweather.com";

pub mod api;
pub mod client;
pub mod model;

pub type SDKResult<T> = Result<T, reqwest::Error>;
