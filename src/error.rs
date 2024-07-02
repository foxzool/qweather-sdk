#[derive(Debug, thiserror::Error)]
pub enum QWeatherError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),

}