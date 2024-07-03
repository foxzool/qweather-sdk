use std::env;

use dotenvy::dotenv;

use qweather_sdk::client::QWeatherClient;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let key = env::var("QWEATHER_KEY").unwrap();

    let client = QWeatherClient::new(key, false);
    let resp = client.air_now("101021600", None, None).await.unwrap();
    println!("{:#?}", resp);
}
