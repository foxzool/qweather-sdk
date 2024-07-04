use std::env;

use dotenvy::dotenv;

use qweather_sdk::{
    api::geo::GeoPoiLookupInput,
    client::{ClientConfig, QWeatherClient},
};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let id = env::var("QWEATHER_ID").unwrap();
    let key = env::var("QWEATHER_KEY").unwrap();
    let client_config = ClientConfig::new(id, key);
    let client = QWeatherClient::with_config(client_config);

    let geo_poi_lookup_input = GeoPoiLookupInput {
        location: "jings",
        type_: "scenic",
        city: None,
        number: None,
    };

    let resp = client.geo_poi_lookup(geo_poi_lookup_input).await.unwrap();
    println!("{:#?}", resp);
}
