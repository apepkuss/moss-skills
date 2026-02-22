# moss-weather-cli Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 在 `skills/moss-weather-cli/` 创建一个独立的 Rust CLI 二进制，通过 OpenWeatherMap API 查询城市当前天气。

**Architecture:** 三文件结构（main.rs / weather.rs / types.rs），复用 `cardea-weather` 的两步请求逻辑：先调用 Geocoding API 将城市名转为经纬度，再调用 Weather API 获取天气数据。用 `clap` 解析命令行参数，`anyhow` 统一错误处理。

**Tech Stack:** Rust 2024, clap 4.x, reqwest 0.12, tokio 1.x, serde/serde_json 1.0, chrono 0.4, anyhow 1.x

---

## 参考资料

- 源码参考：`/Users/sam/workspace/mcp/cardea-weather/src/`（weather.rs, types.rs）
- 设计文档：`docs/plans/2026-02-22-moss-weather-cli-design.md`
- OpenWeatherMap Geocoding API：`http://api.openweathermap.org/geo/1.0/direct`
- OpenWeatherMap Current Weather API：`http://api.openweathermap.org/data/2.5/weather`

---

### Task 1: 初始化 Cargo 项目

**Files:**
- Create: `moss-weather-cli/Cargo.toml`

**Step 1: 创建项目目录并写入 Cargo.toml**

在 `/Volumes/Dev/secondstate/me/skills/` 根目录下执行：

```bash
mkdir moss-weather-cli
```

创建 `moss-weather-cli/Cargo.toml`：

```toml
[package]
name = "moss-weather"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "moss-weather"
path = "src/main.rs"

[dependencies]
clap = { version = "4", features = ["derive"] }
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = "0.4"
anyhow = "1"
```

**Step 2: 验证配置可解析**

```bash
cd /Volumes/Dev/secondstate/me/skills/moss-weather-cli
cargo check
```

预期输出：编译依赖下载，结尾无 error（可能有 "missing src/main.rs" 警告，正常）

**Step 3: 提交**

```bash
git add moss-weather-cli/Cargo.toml
git commit -m "feat(moss-weather-cli): init Cargo project"
```

---

### Task 2: 定义数据结构（types.rs）

**Files:**
- Create: `moss-weather-cli/src/types.rs`

**Step 1: 写失败测试**

创建 `moss-weather-cli/src/types.rs`，先写测试模块：

```rust
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl TemperatureUnit {
    pub fn to_owm_unit(&self) -> &'static str {
        match self {
            TemperatureUnit::Celsius => "metric",
            TemperatureUnit::Fahrenheit => "imperial",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            TemperatureUnit::Celsius => "°C",
            TemperatureUnit::Fahrenheit => "°F",
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GeoLocation {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Deserialize)]
pub struct WeatherCondition {
    pub main: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct MainData {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: f64,
    pub humidity: f64,
}

#[derive(Debug, Deserialize)]
pub struct Wind {
    pub speed: f64,
    pub deg: f64,
    pub gust: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct Clouds {
    pub all: u32,
}

#[derive(Debug, Deserialize)]
pub struct Sys {
    pub country: Option<String>,
    pub sunrise: Option<i64>,
    pub sunset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct Precipitation {
    #[serde(rename = "1h")]
    pub one_hour: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub coord: serde_json::Value,
    pub weather: Vec<WeatherCondition>,
    pub main: MainData,
    pub visibility: Option<u32>,
    pub wind: Wind,
    pub clouds: Clouds,
    pub rain: Option<Precipitation>,
    pub snow: Option<Precipitation>,
    pub sys: Sys,
    pub timezone: i32,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_owm_unit() {
        assert_eq!(TemperatureUnit::Celsius.to_owm_unit(), "metric");
    }

    #[test]
    fn test_fahrenheit_to_owm_unit() {
        assert_eq!(TemperatureUnit::Fahrenheit.to_owm_unit(), "imperial");
    }

    #[test]
    fn test_celsius_symbol() {
        assert_eq!(TemperatureUnit::Celsius.symbol(), "°C");
    }

    #[test]
    fn test_deserialize_weather_response() {
        let json = r#"{
            "coord": {"lon": 116.3972, "lat": 39.9075},
            "weather": [{"id": 800, "main": "Clear", "description": "clear sky", "icon": "01d"}],
            "main": {
                "temp": 20.5, "feels_like": 19.8,
                "temp_min": 18.0, "temp_max": 22.0,
                "pressure": 1013.0, "humidity": 45.0
            },
            "visibility": 10000,
            "wind": {"speed": 3.5, "deg": 180},
            "clouds": {"all": 5},
            "sys": {"country": "CN", "sunrise": 1700000000, "sunset": 1700040000},
            "timezone": 28800,
            "name": "Beijing",
            "id": 1816670, "base": "stations", "cod": 200, "dt": 1700020000
        }"#;

        let resp: WeatherResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.name, "Beijing");
        assert_eq!(resp.main.temp, 20.5);
        assert_eq!(resp.weather[0].main, "Clear");
        assert_eq!(resp.sys.country.as_deref(), Some("CN"));
    }
}
```

**Step 2: 运行测试，确认失败**

```bash
cd /Volumes/Dev/secondstate/me/skills/moss-weather-cli
cargo test
```

预期：编译错误（main.rs 不存在），先创建空的 `src/main.rs`：

```rust
fn main() {}
```

再运行：

```bash
cargo test --lib
```

预期：测试全部通过（PASS）

**Step 3: 提交**

```bash
git add moss-weather-cli/src/types.rs moss-weather-cli/src/main.rs
git commit -m "feat(moss-weather-cli): add data types with tests"
```

---

### Task 3: 实现天气格式化函数（weather.rs）

**Files:**
- Create: `moss-weather-cli/src/weather.rs`

**Step 1: 写失败测试**

创建 `moss-weather-cli/src/weather.rs`，先写 `format_weather_info` 函数及其测试：

```rust
use crate::types::{TemperatureUnit, WeatherResponse};
use chrono::{TimeZone, Utc};

pub fn format_weather_info(resp: &WeatherResponse, unit: &TemperatureUnit) -> String {
    let symbol = unit.symbol();

    let condition = resp
        .weather
        .first()
        .map(|w| format!("{} ({})", w.main, w.description))
        .unwrap_or_else(|| "Unknown".to_string());

    let country = resp.sys.country.as_deref().unwrap_or("--");

    let timezone_offset = chrono::Duration::seconds(resp.timezone as i64);

    let sunrise = resp
        .sys
        .sunrise
        .map(|ts| {
            let utc = Utc.timestamp_opt(ts, 0).single().unwrap_or_default();
            let local = utc + timezone_offset;
            local.format("%H:%M").to_string()
        })
        .unwrap_or_else(|| "--".to_string());

    let sunset = resp
        .sys
        .sunset
        .map(|ts| {
            let utc = Utc.timestamp_opt(ts, 0).single().unwrap_or_default();
            let local = utc + timezone_offset;
            local.format("%H:%M").to_string()
        })
        .unwrap_or_else(|| "--".to_string());

    let visibility = resp
        .visibility
        .map(|v| format!("{:.1} km", v as f64 / 1000.0))
        .unwrap_or_else(|| "--".to_string());

    let wind_dir = wind_direction(resp.wind.deg);
    let gust = resp
        .wind
        .gust
        .map(|g| format!(", gust {:.1} m/s", g))
        .unwrap_or_default();

    let mut output = format!(
        "Weather for {}, {} \n\
         Condition  : {}\n\
         Temperature: {:.1}{} (feels like {:.1}{})\n\
         Range      : {:.1}{} ~ {:.1}{}\n\
         Humidity   : {}%\n\
         Pressure   : {:.0} hPa\n\
         Wind       : {:.1} m/s {} ({}°){}\n\
         Clouds     : {}%\n\
         Visibility : {}\n\
         Sunrise    : {}  Sunset: {}",
        resp.name,
        country,
        condition,
        resp.main.temp,
        symbol,
        resp.main.feels_like,
        symbol,
        resp.main.temp_min,
        symbol,
        resp.main.temp_max,
        symbol,
        resp.main.humidity as u32,
        resp.main.pressure,
        resp.wind.speed,
        wind_dir,
        resp.wind.deg as u32,
        gust,
        resp.clouds.all,
        visibility,
        sunrise,
        sunset,
    );

    if let Some(rain) = &resp.rain {
        if let Some(mm) = rain.one_hour {
            output.push_str(&format!("\nRain (1h)  : {:.1} mm", mm));
        }
    }

    if let Some(snow) = &resp.snow {
        if let Some(mm) = snow.one_hour {
            output.push_str(&format!("\nSnow (1h)  : {:.1} mm", mm));
        }
    }

    output
}

fn wind_direction(deg: f64) -> &'static str {
    let dirs = ["N", "NE", "E", "SE", "S", "SW", "W", "NW"];
    let index = ((deg + 22.5) / 45.0) as usize % 8;
    dirs[index]
}

pub async fn get_weather(
    location: &str,
    unit: &TemperatureUnit,
) -> anyhow::Result<WeatherResponse> {
    let api_key = std::env::var("OPENWEATHERMAP_API_KEY").map_err(|_| {
        anyhow::anyhow!(
            "OPENWEATHERMAP_API_KEY environment variable is not set.\n\
             Get a free API key at https://openweathermap.org/"
        )
    })?;

    let owm_unit = unit.to_owm_unit();

    // Step 1: Geocoding
    let geocode_url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={}&appid={}&limit=1",
        urlencoding_encode(location),
        api_key
    );
    let client = reqwest::Client::new();
    let geo_resp: serde_json::Value = client.get(&geocode_url).send().await?.json().await?;

    let locations = geo_resp
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Unexpected geocoding response format"))?;

    if locations.is_empty() {
        anyhow::bail!("City '{}' not found. Please check the city name.", location);
    }

    let lat = locations[0]["lat"]
        .as_f64()
        .ok_or_else(|| anyhow::anyhow!("Missing latitude in geocoding response"))?;
    let lon = locations[0]["lon"]
        .as_f64()
        .ok_or_else(|| anyhow::anyhow!("Missing longitude in geocoding response"))?;

    // Step 2: Current weather
    let weather_url = format!(
        "http://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units={}",
        lat, lon, api_key, owm_unit
    );

    let weather: WeatherResponse = client.get(&weather_url).send().await?.json().await?;

    Ok(weather)
}

fn urlencoding_encode(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            ' ' => "+".to_string(),
            c if c.is_alphanumeric() || "-_.~".contains(c) => c.to_string(),
            c => format!("%{:02X}", c as u32),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        Clouds, MainData, Precipitation, Sys, TemperatureUnit, WeatherCondition, WeatherResponse,
        Wind,
    };

    fn make_weather_response() -> WeatherResponse {
        WeatherResponse {
            coord: serde_json::json!({"lat": 39.9, "lon": 116.4}),
            weather: vec![WeatherCondition {
                main: "Clear".to_string(),
                description: "clear sky".to_string(),
            }],
            main: MainData {
                temp: 20.0,
                feels_like: 19.0,
                temp_min: 17.0,
                temp_max: 23.0,
                pressure: 1013.0,
                humidity: 50.0,
            },
            visibility: Some(10000),
            wind: Wind {
                speed: 3.0,
                deg: 180.0,
                gust: None,
            },
            clouds: Clouds { all: 10 },
            rain: None,
            snow: None,
            sys: Sys {
                country: Some("CN".to_string()),
                sunrise: Some(1700000000),
                sunset: Some(1700040000),
            },
            timezone: 28800,
            name: "Beijing".to_string(),
        }
    }

    #[test]
    fn test_format_contains_city_name() {
        let resp = make_weather_response();
        let output = format_weather_info(&resp, &TemperatureUnit::Celsius);
        assert!(output.contains("Beijing"));
        assert!(output.contains("CN"));
    }

    #[test]
    fn test_format_contains_temperature() {
        let resp = make_weather_response();
        let output = format_weather_info(&resp, &TemperatureUnit::Celsius);
        assert!(output.contains("20.0°C"));
        assert!(output.contains("50%")); // humidity
    }

    #[test]
    fn test_format_fahrenheit_symbol() {
        let resp = make_weather_response();
        let output = format_weather_info(&resp, &TemperatureUnit::Fahrenheit);
        assert!(output.contains("°F"));
    }

    #[test]
    fn test_format_includes_rain() {
        let mut resp = make_weather_response();
        resp.rain = Some(Precipitation {
            one_hour: Some(2.5),
        });
        let output = format_weather_info(&resp, &TemperatureUnit::Celsius);
        assert!(output.contains("Rain (1h)"));
        assert!(output.contains("2.5 mm"));
    }

    #[test]
    fn test_wind_direction_north() {
        assert_eq!(wind_direction(0.0), "N");
        assert_eq!(wind_direction(360.0), "N");
    }

    #[test]
    fn test_wind_direction_south() {
        assert_eq!(wind_direction(180.0), "S");
    }

    #[test]
    fn test_wind_direction_northeast() {
        assert_eq!(wind_direction(45.0), "NE");
    }
}
```

**Step 2: 运行测试**

```bash
cd /Volumes/Dev/secondstate/me/skills/moss-weather-cli
cargo test --lib 2>&1
```

预期：所有单元测试通过（weather.rs 和 types.rs 的测试全部 PASS）

**Step 3: 提交**

```bash
git add moss-weather-cli/src/weather.rs
git commit -m "feat(moss-weather-cli): add weather API client and formatter"
```

---

### Task 4: 实现 CLI 入口（main.rs）

**Files:**
- Modify: `moss-weather-cli/src/main.rs`

**Step 1: 实现 main.rs**

```rust
mod types;
mod weather;

use clap::{Parser, ValueEnum};
use types::TemperatureUnit;

#[derive(Parser)]
#[command(
    name = "moss-weather",
    about = "Query current weather for any city",
    version
)]
struct Args {
    /// City name to query (e.g. "Beijing", "New York")
    city: String,

    /// Temperature unit
    #[arg(short, long, value_enum, default_value_t = UnitArg::Celsius)]
    unit: UnitArg,
}

#[derive(Clone, ValueEnum)]
enum UnitArg {
    Celsius,
    Fahrenheit,
}

impl From<UnitArg> for TemperatureUnit {
    fn from(u: UnitArg) -> Self {
        match u {
            UnitArg::Celsius => TemperatureUnit::Celsius,
            UnitArg::Fahrenheit => TemperatureUnit::Fahrenheit,
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let unit: TemperatureUnit = args.unit.into();

    match weather::get_weather(&args.city, &unit).await {
        Ok(resp) => {
            println!("{}", weather::format_weather_info(&resp, &unit));
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
```

**Step 2: 编译验证**

```bash
cd /Volumes/Dev/secondstate/me/skills/moss-weather-cli
cargo build 2>&1
```

预期：编译成功，无 error

**Step 3: 运行测试确认全部通过**

```bash
cargo test 2>&1
```

预期：所有测试通过（PASS）

**Step 4: 提交**

```bash
git add moss-weather-cli/src/main.rs
git commit -m "feat(moss-weather-cli): add CLI entry point with clap"
```

---

### Task 5: 手动测试与验收

**Step 1: 构建 release 版本**

```bash
cd /Volumes/Dev/secondstate/me/skills/moss-weather-cli
cargo build --release 2>&1
```

预期：`target/release/moss-weather` 生成

**Step 2: 测试缺少 API key 时的错误提示**

```bash
unset OPENWEATHERMAP_API_KEY
./target/release/moss-weather "Beijing"
```

预期输出（stderr）：
```
Error: OPENWEATHERMAP_API_KEY environment variable is not set.
Get a free API key at https://openweathermap.org/
```
退出码为 1

**Step 3: 设置 API key 并查询天气**

```bash
export OPENWEATHERMAP_API_KEY=<your-key>
./target/release/moss-weather "Beijing"
```

预期输出示例：
```
Weather for Beijing, CN
Condition  : Clear (clear sky)
Temperature: 5.2°C (feels like 3.1°C)
Range      : 3.0°C ~ 7.0°C
Humidity   : 40%
Pressure   : 1020 hPa
Wind       : 2.5 m/s N (350°)
Clouds     : 0%
Visibility : 10.0 km
Sunrise    : 07:32  Sunset: 17:15
```

**Step 4: 测试华氏度**

```bash
./target/release/moss-weather "New York" --unit fahrenheit
```

预期：温度显示为 `°F`

**Step 5: 测试城市名找不到**

```bash
./target/release/moss-weather "Xyznotacity12345"
```

预期（stderr）：
```
Error: City 'Xyznotacity12345' not found. Please check the city name.
```

**Step 6: 最终提交**

```bash
git add -A
git commit -m "feat(moss-weather-cli): complete implementation and manual verification"
```

---

## 完成标准

- [ ] `cargo test` 全部通过
- [ ] `cargo build --release` 成功
- [ ] 缺少 API key 时输出友好错误信息
- [ ] 城市名无效时输出友好错误信息
- [ ] 使用真实 API key 可成功查询天气
- [ ] `--unit fahrenheit` 切换温度单位正常
