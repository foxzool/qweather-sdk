use dotenvy::dotenv;
use qweather_sdk::client::QWeatherClient;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let id = env::var("QWEATHER_ID").unwrap();
    let key = env::var("QWEATHER_KEY").unwrap();

    let client = QWeatherClient::new(id, key, false);
    let resp = client.weather_warning_city_list("cn").await.unwrap();
    println!("{:#?}", resp);
}
