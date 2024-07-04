use std::env;

use dotenvy::dotenv;

use qweather_sdk::client::QWeatherClient;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let id = env::var("QWEATHER_ID").unwrap();
    let key = env::var("QWEATHER_KEY").unwrap();

    let client = QWeatherClient::new(id, key, false, "zh");
    let resp = client.geo_city_top(None, None).await.unwrap();
    println!("{:#?}", resp);
}
