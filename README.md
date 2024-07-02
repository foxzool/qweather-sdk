# 和风天气SDK

## Example

```rust
#[tokio::main]
async fn main() {
    env_logger::init();
    let key = env::var("QWEATHER_KEY").unwrap();

    let client = QWeatherClient::new(key, false);
    let weather_now = client.weather_now("101010100").await.unwrap();
    println!("{:#?}", weather_now.data);
}
```

## 已完成的API

### 城市天气`
    - [x] 实时天气
    - [x] 每日天气预报
    - [x] 逐小时天气预报