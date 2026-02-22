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
        "Weather for {}, {}\n\
         Condition  : {}\n\
         Temperature: {:.1}{} (feels like {:.1}{})\n\
         Range      : {:.1}{} ~ {:.1}{}\n\
         Humidity   : {}%\n\
         Pressure   : {} hPa\n\
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
        resp.main.humidity,
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
        "https://api.openweathermap.org/geo/1.0/direct?q={}&appid={}&limit=1",
        urlencoding_encode(location),
        api_key
    );
    let client = reqwest::Client::new();
    let geo_resp: serde_json::Value = client
        .get(&geocode_url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

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
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units={}",
        lat, lon, api_key, owm_unit
    );

    let weather: WeatherResponse = client
        .get(&weather_url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(weather)
}

fn urlencoding_encode(s: &str) -> String {
    s.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            b' ' => "+".to_string(),
            b => format!("%{:02X}", b),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        Clouds, GeoLocation, MainData, Precipitation, Sys, TemperatureUnit, WeatherCondition,
        WeatherResponse, Wind,
    };

    fn make_weather_response() -> WeatherResponse {
        WeatherResponse {
            coord: GeoLocation {
                lat: 39.9,
                lon: 116.4,
            },
            weather: vec![WeatherCondition {
                main: "Clear".to_string(),
                description: "clear sky".to_string(),
            }],
            main: MainData {
                temp: 20.0,
                feels_like: 19.0,
                temp_min: 17.0,
                temp_max: 23.0,
                pressure: 1013,
                humidity: 50,
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

    #[test]
    fn test_format_includes_snow() {
        let mut resp = make_weather_response();
        resp.snow = Some(Precipitation {
            one_hour: Some(1.2),
        });
        let output = format_weather_info(&resp, &TemperatureUnit::Celsius);
        assert!(output.contains("Snow (1h)"));
        assert!(output.contains("1.2 mm"));
    }
}
