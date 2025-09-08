use crate::config::Config;
use crate::errors::RustormyError;
use crate::models::{Units, Weather, WeatherConditionIcon};
use crate::weather::GetWeather;
use reqwest::blocking::Client;

const WEATHER_API_URL: &str = "https://api.weatherapi.com/v1/current.json";

/// Module for interacting with the <https://www.weatherapi.com/> service.
/// Requires an API key, which can be obtained for free by signing up on their website.
/// API documentation: <https://www.weatherapi.com/docs>
#[derive(Debug, Default)]
pub struct WeatherApi {}

#[derive(Debug, serde::Serialize)]
struct WeatherApiRequest<'a> {
    q: String,
    key: &'a str,
    lang: &'a str,
    aqi: &'a str,
}

impl<'a> WeatherApiRequest<'a> {
    pub fn new(config: &'a Config) -> Result<Self, RustormyError> {
        let q = config.location_name();
        let key = config.api_key_wa().ok_or(RustormyError::MissingApiKey)?;
        let lang = config.language().code();

        Ok(Self {
            key,
            q,
            lang,
            // TODO: air quality would be nice to have
            aqi: "no",
        })
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum WeatherApiResponse {
    Ok(WeatherApiData),
    Err { error: WeatherApiError },
}

#[derive(Debug, serde::Deserialize)]
struct WeatherApiError {
    code: i32,
    message: String,
}

#[derive(Debug, serde::Deserialize)]
struct WeatherApiData {
    location: WeatherApiLocation,
    current: WeatherApiCurrent,
}

impl WeatherApiData {
    fn into_weather(self, config: &Config) -> Weather {
        let location_name = self.location.location_name();
        let current = self.current;

        Weather {
            location_name,
            temperature: current.temperature(config.units()),
            feels_like: current.feels_like(config.units()),
            humidity: current.humidity,
            precipitation: current.precipitation(config.units()),
            pressure: current.pressure(config.units()),
            wind_speed: current.wind_speed(config.units()),
            wind_direction: current.wind_degree,
            uv_index: Some(current.uv_index()),
            dew_point: current.dew_point(config.units()),
            description: current.description().to_string(),
            icon: current.icon(),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct WeatherApiLocation {
    name: String,
    region: String,
    country: String,
    lat: f64,
    lon: f64,
    // tz_id: String,
    // localtime_epoch: i64,
    // localtime: String,
}

impl WeatherApiLocation {
    fn location_name(self) -> String {
        match (self.name, self.region, self.country) {
            (name, _, _) if name.is_empty() => format!("{}, {}", self.lat, self.lon),
            (name, region, country) if !region.is_empty() && !country.is_empty() => {
                format!("{name}, {region}, {country}")
            }
            (name, _, country) if !country.is_empty() => format!("{name}, {country}"),
            (name, _, _) => name,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct WeatherApiCurrent {
    temp_c: f64,
    temp_f: f64,
    // is_day: u8,
    condition: WeatherApiCondition,
    wind_mph: f64,
    wind_kph: f64,
    wind_degree: u16,
    // wind_dir: String,
    pressure_mb: f64,
    pressure_in: f64,
    precip_mm: f64,
    precip_in: f64,
    humidity: u8,
    // cloud: u8,
    feelslike_c: f64,
    feelslike_f: f64,
    dewpoint_c: f64,
    dewpoint_f: f64,
    // vis_km: f64,
    // vis_miles: f64,
    uv: f64,
}

impl WeatherApiCurrent {
    fn temperature(&self, units: Units) -> f64 {
        match units {
            Units::Metric => self.temp_c,
            Units::Imperial => self.temp_f,
        }
    }

    fn feels_like(&self, units: Units) -> f64 {
        match units {
            Units::Metric => self.feelslike_c,
            Units::Imperial => self.feelslike_f,
        }
    }

    fn precipitation(&self, units: Units) -> f64 {
        match units {
            Units::Metric => self.precip_mm,
            Units::Imperial => self.precip_in,
        }
    }

    fn pressure(&self, units: Units) -> u32 {
        let value = match units {
            Units::Metric => self.pressure_mb,
            Units::Imperial => self.pressure_in,
        };

        value.round() as u32
    }

    fn wind_speed(&self, units: Units) -> f64 {
        // TODO: move wind speed conversion to tools
        let value = match units {
            Units::Metric => self.wind_kph / 3.6, // Convert kph to m/s
            Units::Imperial => self.wind_mph,
        };

        (value * 10.0).round() / 10.0 // Round to 1 decimal place
    }

    fn uv_index(&self) -> u8 {
        self.uv.round() as u8
    }

    fn dew_point(&self, units: Units) -> f64 {
        match units {
            Units::Metric => self.dewpoint_c,
            Units::Imperial => self.dewpoint_f,
        }
    }

    fn description(&self) -> &str {
        &self.condition.text
    }

    fn icon(&self) -> WeatherConditionIcon {
        // Condition codes: https://www.weatherapi.com/docs/conditions.json
        match self.condition.code {
            // Clear/Sunny
            1000 => WeatherConditionIcon::Clear,
            // Partly cloudy
            1003 => WeatherConditionIcon::PartlyCloudy,
            // Cloudy/Overcast
            1006 | 1009 => WeatherConditionIcon::Cloudy,
            // Mist/Fog
            1030 | 1135 | 1147 => WeatherConditionIcon::Fog,
            // Patchy rain, drizzle, light rain, light sleet
            1063 | 1150 | 1153 | 1180 | 1183 | 1240 | 1249 | 1252 => {
                WeatherConditionIcon::LightShowers
            }
            // Moderate/heavy rain, heavy sleet, heavy snow showers
            1186 | 1189 | 1192 | 1195 | 1243 | 1246 => WeatherConditionIcon::HeavyShowers,
            // Patchy/light snow, ice pellets
            1066 | 1069 | 1072 | 1210 | 1213 | 1216 | 1219 | 1222 | 1225 | 1237 | 1255 => {
                WeatherConditionIcon::LightSnow
            }
            // Blowing/heavy snow, heavy ice pellets
            1114 | 1117 | 1228 | 1231 | 1258 => WeatherConditionIcon::HeavySnow,
            // Thundery outbreaks, thunderstorms
            1087 | 1273 | 1276 | 1279 | 1282 => WeatherConditionIcon::Thunderstorm,
            _ => WeatherConditionIcon::Unknown,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct WeatherApiCondition {
    text: String,
    // icon: String,
    code: i32,
}

impl GetWeather for WeatherApi {
    fn get_weather(&self, client: &Client, config: &Config) -> Result<Weather, RustormyError> {
        let request = WeatherApiRequest::new(config)?;
        let response = client.get(WEATHER_API_URL).query(&request).send()?;
        let data: WeatherApiResponse = response.json()?;
        match data {
            WeatherApiResponse::Ok(data) => Ok(data.into_weather(config)),
            WeatherApiResponse::Err { error } => Err(RustormyError::ApiReturnedError(format!(
                "{} {}",
                error.code, error.message
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_weather() {
        const DATA: &str = r#"
{
  "location": {
    "name": "Batumi",
    "region": "Ajaria",
    "country": "Georgia",
    "lat": 41.6386,
    "lon": 41.6372,
    "tz_id": "Asia/Tbilisi",
    "localtime_epoch": 1757329198,
    "localtime": "2025-09-08 14:59"
  },
  "current": {
    "last_updated_epoch": 1757328300,
    "last_updated": "2025-09-08 14:45",
    "temp_c": 25.3,
    "temp_f": 77.5,
    "is_day": 1,
    "condition": {
      "text": "Переменная облачность",
      "icon": "//cdn.weatherapi.com/weather/64x64/day/116.png",
      "code": 1003
    },
    "wind_mph": 6.5,
    "wind_kph": 10.4,
    "wind_degree": 257,
    "wind_dir": "WSW",
    "pressure_mb": 1011,
    "pressure_in": 29.85,
    "precip_mm": 0.04,
    "precip_in": 0,
    "humidity": 74,
    "cloud": 50,
    "feelslike_c": 27.4,
    "feelslike_f": 81.2,
    "windchill_c": 23.9,
    "windchill_f": 75.1,
    "heatindex_c": 25.8,
    "heatindex_f": 78.4,
    "dewpoint_c": 19.8,
    "dewpoint_f": 67.7,
    "vis_km": 10,
    "vis_miles": 6,
    "uv": 5.3,
    "gust_mph": 7.5,
    "gust_kph": 12,
    "short_rad": 695.06,
    "diff_rad": 110.35,
    "dni": 1037.04,
    "gti": 294.19
  }
}
        "#;
        let data: WeatherApiResponse = serde_json::from_str(DATA).unwrap();
        assert!(
            matches!(data, WeatherApiResponse::Ok(_)),
            "Expected Ok variant, got {:?}",
            data
        );
        let weather = match data {
            WeatherApiResponse::Ok(data) => data.into_weather(&Config::default()),
            _ => panic!("Expected Ok variant"),
        };
        assert_eq!(weather.location_name, "Batumi, Ajaria, Georgia");
        assert_eq!(weather.temperature, 25.3);
        assert_eq!(weather.feels_like, 27.4);
        assert_eq!(weather.humidity, 74);
        assert_eq!(weather.dew_point, 19.8);
        assert_eq!(weather.precipitation, 0.04);
        assert_eq!(weather.pressure, 1011);
        assert_eq!(weather.wind_speed, 2.9); // 10.4 kph to m/s rounded to 1 decimal place
        assert_eq!(weather.wind_direction, 257);
        assert_eq!(weather.uv_index, Some(5));
        assert_eq!(weather.description, "Переменная облачность");
        assert_eq!(weather.icon, WeatherConditionIcon::PartlyCloudy);
    }

    #[test]
    fn test_parse_error() {
        const DATA: &str = r#"{"error":{"code":1006,"message":"No matching location found."}}"#;
        let data: WeatherApiResponse = serde_json::from_str(DATA).unwrap();
        assert!(
            matches!(data, WeatherApiResponse::Err { .. }),
            "Expected Err variant, got {:?}",
            data
        );
        match data {
            WeatherApiResponse::Err { error } => {
                assert_eq!(error.code, 1006);
                assert_eq!(error.message, "No matching location found.");
            }
            _ => panic!("Expected Err variant"),
        }
    }
}
