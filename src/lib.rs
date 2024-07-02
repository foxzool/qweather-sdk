/// GEO API URL
pub static GEO_API_URL: &str = "https://geoapi.qweather.com";

/// 标准订阅 API URL
pub static WEATHER_API_URL: &str = "https://api.qweather.com";

/// 免费订阅 API URL
pub static WEATHER_DEV_API_URL: &str = "https://devapi.qweather.com";

pub mod client;
pub mod model;
pub mod api;
pub mod error;

pub type SDKResult<T> = Result<T, error::QWeatherError>;