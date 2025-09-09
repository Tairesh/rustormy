use crate::config::Config;
use crate::display::translations::ll;
use crate::errors::RustormyError;
use crate::models::{Language, Units, Weather, WeatherConditionIcon};
use crate::weather::GetWeather;
use reqwest::blocking::Client;

const REALTIME_API_URL: &str = "https://api.tomorrow.io/v4/weather/realtime";

#[derive(Debug, Default)]
pub struct TomorrowIo {}

#[derive(Debug, serde::Serialize)]
struct WeatherRequestParams<'a> {
    location: String,
    units: Units,
    apikey: &'a str,
}

impl<'a> WeatherRequestParams<'a> {
    pub fn new(config: &'a Config) -> Self {
        let apikey = &config.api_keys().tomorrow_io;
        let location = config.location_name();
        let units = config.units();

        Self {
            location,
            units,
            apikey,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum WeatherResponse {
    Ok {
        data: WeatherData,
        location: LocationData,
    },
    Err {
        code: u32,
        #[serde(rename = "type")]
        e_type: String,
        message: String,
    },
}

#[derive(Debug, serde::Deserialize)]
struct WeatherData {
    // time: String,
    values: WeatherValues,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct WeatherValues {
    temperature: f64,
    temperature_apparent: f64,
    humidity: u8,
    rain_intensity: f64,
    sleet_intensity: f64,
    snow_intensity: f64,
    freezing_rain_intensity: f64,
    pressure_surface_level: f64,
    wind_speed: f64,
    wind_direction: u16,
    uv_index: u8,
    weather_code: u16,
    dew_point: f64,
    // pressure_sea_level: f64,
    // visibility: f64,
    // cloud_cover: u8,
    // cloud_base: Option<f64>,
    // cloud_ceiling: Option<f64>,
    // precipitation_probability: u8,
    // wind_gust: f64,
    // altimeter_setting: f64,
    // uv_health_concern: u8,
}

impl WeatherValues {
    pub fn icon(&self) -> WeatherConditionIcon {
        match self.weather_code {
            1000 => WeatherConditionIcon::Clear,
            1100 | 1101 => WeatherConditionIcon::PartlyCloudy,
            1102 | 1001 => WeatherConditionIcon::Cloudy,
            2000 | 2100 => WeatherConditionIcon::Fog,
            4000 | 4200 | 6000 | 6200 => WeatherConditionIcon::LightShowers,
            4001 | 4201 | 6001 | 6201 => WeatherConditionIcon::HeavyShowers,
            5001 | 5100 | 7102 => WeatherConditionIcon::LightSnow,
            5000 | 5101 | 7000 | 7101 => WeatherConditionIcon::HeavySnow,
            8000 => WeatherConditionIcon::Thunderstorm,
            _ => WeatherConditionIcon::Unknown,
        }
    }

    pub fn description(&self, lang: Language) -> &'static str {
        ll(
            lang,
            match self.weather_code {
                1000 => "Clear",
                1100 => "Mostly clear",
                1101 => "Partly cloudy",
                1102 => "Mostly cloudy",
                1001 => "Cloudy",
                2000 => "Fog",
                2100 => "Light fog",
                4000 => "Drizzle",
                4001 => "Rain",
                4200 => "Light rain",
                4201 => "Heavy rain",
                5000 => "Snow",
                5001 => "Flurries",
                5100 => "Light snow",
                5101 => "Heavy snow",
                6000 => "Freezing drizzle",
                6001 => "Freezing rain",
                6200 => "Light freezing rain",
                6201 => "Heavy freezing rain",
                7000 => "Ice pellets",
                7101 => "Heavy ice pellets",
                7102 => "Light ice pellets",
                8000 => "Thunderstorm",
                _ => "Unknown",
            },
        )
    }
}

#[derive(Debug, serde::Deserialize)]
struct LocationData {
    // lat: f64,
    // lon: f64,
    name: String,
    // #[serde(rename = "type")]
    // loc_type: String,
}

impl LocationData {
    pub fn name(self) -> String {
        // Name comes in the local language and can be VERY long
        // e.g. "ბათუმი, აჭარის ავტონომიური რესპუბლიკა, საქართველო"
        // Let's hope that it's at least somewhat standardized like "{city}, [{some}, {regional}, {stuff}, ..] {country}"
        // And return only the city name and country name (if city name isn't too long)
        let parts: Vec<&str> = self.name.split(',').map(str::trim).collect();
        if parts.len() >= 2 {
            let city = parts[0];
            let country = parts.last().unwrap_or(&"");
            if city.len() <= 20 {
                format!("{city}, {country}")
            } else {
                city.to_string()
            }
        } else {
            self.name
        }
    }
}

impl WeatherResponse {
    pub fn into_weather(self, config: &Config) -> Result<Weather, RustormyError> {
        match self {
            Self::Err {
                code,
                e_type,
                message,
            } => Err(RustormyError::ApiReturnedError(format!(
                "#{code} {e_type}: {message}",
            ))),
            WeatherResponse::Ok { data, location } => {
                let precipitation = data.values.rain_intensity
                    + data.values.sleet_intensity
                    + data.values.snow_intensity
                    + data.values.freezing_rain_intensity;
                let pressure = data.values.pressure_surface_level.round() as u32;

                Ok(Weather {
                    temperature: data.values.temperature,
                    feels_like: data.values.temperature_apparent,
                    humidity: data.values.humidity,
                    dew_point: data.values.dew_point,
                    precipitation,
                    pressure,
                    wind_speed: data.values.wind_speed,
                    wind_direction: data.values.wind_direction,
                    uv_index: Some(data.values.uv_index),
                    icon: data.values.icon(),
                    description: data.values.description(config.language()).to_string(),
                    location_name: location.name(),
                })
            }
        }
    }
}

impl GetWeather for TomorrowIo {
    fn get_weather(&self, client: &Client, config: &Config) -> Result<Weather, RustormyError> {
        let request = WeatherRequestParams::new(config);
        let response = client.get(REALTIME_API_URL).query(&request).send()?;
        let data: WeatherResponse = response.json()?;

        data.into_weather(config)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_data() {
        const EXAMPLE_DATA: &str = r#"
            {
              "data": {
                "time": "2025-09-09T11:44:00Z",
                "values": {
                  "altimeterSetting": 1011.27,
                  "cloudBase": 0.4,
                  "cloudCeiling": null,
                  "cloudCover": 73,
                  "dewPoint": 20.3,
                  "freezingRainIntensity": 0,
                  "humidity": 83,
                  "precipitationProbability": 100,
                  "pressureSeaLevel": 1012.33,
                  "pressureSurfaceLevel": 1011.87,
                  "rainIntensity": 2.03,
                  "sleetIntensity": 0,
                  "snowIntensity": 0,
                  "temperature": 23.4,
                  "temperatureApparent": 23.4,
                  "uvHealthConcern": 0,
                  "uvIndex": 2,
                  "visibility": 11.53,
                  "weatherCode": 4200,
                  "windDirection": 219,
                  "windGust": 8,
                  "windSpeed": 5.4
                }
              },
              "location": {
                "lat": 41.6509513854981,
                "lon": 41.6360092163086,
                "name": "ბათუმი, აჭარის ავტონომიური რესპუბლიკა, საქართველო",
                "type": "administrative"
              }
            }
        "#;

        let response: WeatherResponse = serde_json::from_str(EXAMPLE_DATA).unwrap();
        let weather = response.into_weather(&Config::default()).unwrap();
        assert_eq!(weather.temperature, 23.4);
        assert_eq!(weather.feels_like, 23.4);
        assert_eq!(weather.humidity, 83);
        assert_eq!(weather.dew_point, 20.3);
        assert_eq!(weather.precipitation, 2.03);
        assert_eq!(weather.pressure, 1012);
        assert_eq!(weather.wind_speed, 5.4);
        assert_eq!(weather.wind_direction, 219);
        assert_eq!(weather.uv_index, Some(2));
        assert_eq!(weather.icon, WeatherConditionIcon::LightShowers);
        assert_eq!(weather.description, "Light rain");
        assert_eq!(weather.location_name, "ბათუმი, საქართველო"); // shortened name
    }

    #[test]
    fn test_parse_error() {
        const EXAMPLE_DATA: &str = r#"{"code":401001,"type":"Invalid Auth","message":"..."}"#;
        let response: WeatherResponse = serde_json::from_str(EXAMPLE_DATA).unwrap();
        match response {
            WeatherResponse::Err { code, e_type, .. } => {
                assert_eq!(code, 401001);
                assert_eq!(e_type, "Invalid Auth");
            }
            _ => panic!("Expected error response"),
        }
    }
}
