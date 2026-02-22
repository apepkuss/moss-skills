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
    pub pressure: u32,
    pub humidity: u8,
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
    pub coord: GeoLocation,
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
    fn test_fahrenheit_symbol() {
        assert_eq!(TemperatureUnit::Fahrenheit.symbol(), "°F");
    }

    #[test]
    fn test_deserialize_weather_response() {
        let json = r#"{
            "coord": {"lon": 116.3972, "lat": 39.9075},
            "weather": [{"id": 800, "main": "Clear", "description": "clear sky", "icon": "01d"}],
            "main": {
                "temp": 20.5, "feels_like": 19.8,
                "temp_min": 18.0, "temp_max": 22.0,
                "pressure": 1013, "humidity": 45
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
        assert_eq!(resp.main.pressure, 1013u32);
        assert_eq!(resp.main.humidity, 45u8);
        assert_eq!(resp.weather[0].main, "Clear");
        assert_eq!(resp.sys.country.as_deref(), Some("CN"));
    }
}
