use dotenvy::dotenv;
use qweather_sdk::client::{ClientConfig, QWeatherClient};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let id = env::var("QWEATHER_ID").unwrap();
    let key = env::var("QWEATHER_KEY").unwrap();
    let client_config = ClientConfig::new(id, key);
    let client = QWeatherClient::with_config(client_config);

    let resp = client.weather_warning("101021600").await.unwrap();
    println!("{:#?}", resp);
}
