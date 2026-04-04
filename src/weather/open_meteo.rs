use crate::config::Config;
use crate::display::translations::ll;
use crate::errors::RustormyError;
use crate::models::{Language, Location, Units, Weather, WeatherConditionIcon};
use crate::weather::openuv::get_uv_index;
use crate::weather::{GetWeather, LookUpCity, tools};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

const GEO_API_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";
const WEATHER_API_URL: &str = "https://api.open-meteo.com/v1/forecast";

#[derive(Debug, Default)]
pub struct OpenMeteo {}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ApiResponse<T> {
    Ok(T),
    Err {
        _error: bool,
        reason: Option<String>,
    },
}

impl<T> ApiResponse<T> {
    fn into_result(self) -> Result<T, RustormyError> {
        match self {
            Self::Ok(data) => Ok(data),
            Self::Err { reason, .. } => Err(RustormyError::ApiReturnedError(
                reason.unwrap_or_else(|| "Unknown error".to_string()),
            )),
        }
    }
}

#[derive(Debug, Deserialize)]
struct OpenMeteoResponse {
    current: CurrentWeather,
}

impl OpenMeteoResponse {
    pub fn into_weather(
        self,
        client: &Client,
        config: &Config,
        location: &Location,
    ) -> Result<Weather, RustormyError> {
        Ok(Weather {
            temperature: self.current.temperature,
            feels_like: self.current.apparent_temperature,
            humidity: self.current.humidity,
            dew_point: self.dew_point(config.units()),
            precipitation: self.current.precipitation,
            pressure: self.current.pressure as u32,
            wind_speed: self.current.wind_speed,
            wind_direction: self.current.wind_direction,
            uv_index: get_uv_index(client, config, location)?,
            description: self.description(config.language()).to_string(),
            icon: self.icon(),
            location_name: location.name.clone(),
        })
    }

    fn description(&self, lang: Language) -> &'static str {
        ll(
            lang,
            match self.current.weather_code {
                0 => "Clear",
                1 => "Mostly clear",
                2 => "Partly cloudy",
                3 => "Overcast",
                45 => "Fog",
                48 => "Depositing rime fog",
                51 => "Light drizzle",
                53 => "Moderate drizzle",
                55 => "Dense drizzle",
                56 => "Light freezing drizzle",
                57 => "Dense freezing drizzle",
                61 => "Light rain",
                63 => "Moderate rain",
                65 => "Heavy rain",
                66 => "Light freezing rain",
                67 => "Heavy freezing rain",
                71 => "Slight snow fall",
                73 => "Moderate snow fall",
                75 => "Heavy snow fall",
                77 => "Snow grains",
                80 => "Slight rain showers",
                81 => "Moderate rain showers",
                82 => "Violent rain showers",
                85 => "Slight snow showers",
                86 => "Heavy snow showers",
                95 => "Thunderstorm",
                96 => "Thunderstorm with slight hail",
                99 => "Thunderstorm with heavy hail",
                _ => "Unknown",
            },
        )
    }

    fn icon(&self) -> WeatherConditionIcon {
        match self.current.weather_code {
            0 => WeatherConditionIcon::Clear,
            1..=2 => WeatherConditionIcon::PartlyCloudy,
            3 => WeatherConditionIcon::Cloudy,
            45 | 48 => WeatherConditionIcon::Fog,
            51..=57 | 80 => WeatherConditionIcon::LightShowers,
            61..=67 | 81 | 82 => WeatherConditionIcon::HeavyShowers,
            71..=73 => WeatherConditionIcon::LightSnow,
            75 | 77 | 85 | 86 => WeatherConditionIcon::HeavySnow,
            95 | 96 | 99 => WeatherConditionIcon::Thunderstorm,
            _ => WeatherConditionIcon::Unknown,
        }
    }

    fn dew_point(&self, units: Units) -> f64 {
        let t = self.current.temperature;
        let h = self.current.humidity.into();

        tools::dew_point(t, h, units)
    }
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    #[serde(rename = "temperature_2m")]
    temperature: f64,
    apparent_temperature: f64,
    #[serde(rename = "relative_humidity_2m")]
    humidity: u8,
    precipitation: f64,
    #[serde(rename = "surface_pressure")]
    pressure: f64,
    #[serde(rename = "wind_speed_10m")]
    wind_speed: f64,
    #[serde(rename = "wind_direction_10m")]
    wind_direction: u16,
    weather_code: u8,
}

#[derive(Debug, Serialize)]
struct GeocodingRequest<'a> {
    name: &'a str,
    count: u8,
    language: &'a str,
}

impl<'a> GeocodingRequest<'a> {
    pub fn new(name: &'a str, language: Language) -> Self {
        Self {
            name,
            count: 1,
            language: language.code(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct GeocodingResponse {
    results: Option<Vec<GeocodingLocation>>,
}

impl GeocodingResponse {
    pub fn into_location(self) -> Option<Location> {
        self.results
            .and_then(|mut results| results.pop())
            .map(Location::from)
    }
}

#[derive(Debug, Deserialize)]
struct GeocodingLocation {
    name: String,
    latitude: f64,
    longitude: f64,
    // elevation: f64,
    // timezone: String,
}

impl From<GeocodingLocation> for Location {
    fn from(loc: GeocodingLocation) -> Self {
        Location {
            name: loc.name,
            latitude: loc.latitude,
            longitude: loc.longitude,
        }
    }
}

#[derive(Debug, serde::Serialize)]
struct WeatherAPIRequest<'a> {
    latitude: f64,
    longitude: f64,
    current: &'a str,
    temperature_unit: &'a str,
    wind_speed_unit: &'a str,
    precipitation_unit: &'a str,
}

impl<'a> WeatherAPIRequest<'a> {
    pub fn new(location: &Location, config: &'a Config) -> Self {
        const CURRENT: &str = "temperature_2m,apparent_temperature,relative_humidity_2m,precipitation,surface_pressure,wind_speed_10m,wind_direction_10m,weather_code";
        let (temperature_unit, wind_speed_unit, precipitation_unit) = match config.units() {
            Units::Metric => ("celsius", "ms", "mm"),
            Units::Imperial => ("fahrenheit", "mph", "inch"),
        };

        Self {
            latitude: location.latitude,
            longitude: location.longitude,
            current: CURRENT,
            temperature_unit,
            wind_speed_unit,
            precipitation_unit,
        }
    }
}

impl LookUpCity for OpenMeteo {
    fn lookup_city(&self, client: &Client, config: &Config) -> Result<Location, RustormyError> {
        let city = config.city().ok_or(RustormyError::NoLocationProvided)?;

        let request = GeocodingRequest::new(city, config.language());
        let response = client.get(GEO_API_URL).query(&request).send()?;
        let data = response
            .json::<ApiResponse<GeocodingResponse>>()?
            .into_result()?;

        let location = data
            .into_location()
            .ok_or_else(|| RustormyError::CityNotFound(city.to_string()))?;

        Ok(location)
    }
}

impl GetWeather for OpenMeteo {
    fn get_weather(&self, client: &Client, config: &Config) -> Result<Weather, RustormyError> {
        let location = self.get_location(client, config)?;
        let response = client
            .get(WEATHER_API_URL)
            .query(&WeatherAPIRequest::new(&location, config))
            .send()?;
        let data = response
            .json::<ApiResponse<OpenMeteoResponse>>()?
            .into_result()?;

        data.into_weather(client, config, &location)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::models::WeatherConditionIcon;
    use test_case::test_case;

    const GEOCODING_RESPONSE: &str =
        include_str!("../../tests/data/open_meteo_geocoding_response.json");
    const WEATHER_RESPONSE: &str =
        include_str!("../../tests/data/open_meteo_weather_response.json");

    fn make_weather_response(weather_code: u8) -> OpenMeteoResponse {
        let json = format!(
            r#"{{"current":{{"temperature_2m":20.0,"apparent_temperature":18.0,"relative_humidity_2m":50,"precipitation":0.0,"surface_pressure":1013.0,"wind_speed_10m":5.0,"wind_direction_10m":180,"weather_code":{weather_code}}}}}"#
        );
        serde_json::from_str(&json).unwrap()
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_geocoding_into_location() {
        let data: GeocodingResponse = serde_json::from_str(GEOCODING_RESPONSE).unwrap();
        let location = data.into_location().unwrap();
        assert_eq!(location.name, "Da Nang");
        assert_eq!(location.latitude, 16.06778);
        assert_eq!(location.longitude, 108.22083);
    }

    #[test]
    fn test_geocoding_empty_results() {
        let data: GeocodingResponse = serde_json::from_str(r#"{"results": []}"#).unwrap();
        assert!(data.into_location().is_none());
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_parse_weather_response() {
        let data: ApiResponse<OpenMeteoResponse> = serde_json::from_str(WEATHER_RESPONSE).unwrap();
        let response = data.into_result().unwrap();
        assert_eq!(response.current.temperature, 24.2);
        assert_eq!(response.current.humidity, 67);
        assert_eq!(response.current.wind_direction, 258);
        assert_eq!(response.current.weather_code, 1);
    }

    #[test_case(0, "Clear")]
    #[test_case(1, "Mostly clear")]
    #[test_case(95, "Thunderstorm")]
    fn test_description(weather_code: u8, expected: &str) {
        assert_eq!(
            make_weather_response(weather_code).description(Config::default().language()),
            expected
        );
    }

    #[test_case(0, WeatherConditionIcon::Clear)]
    #[test_case(1, WeatherConditionIcon::PartlyCloudy)]
    #[test_case(95, WeatherConditionIcon::Thunderstorm)]
    #[test_case(100, WeatherConditionIcon::Unknown)]
    fn test_icon(weather_code: u8, expected: WeatherConditionIcon) {
        assert_eq!(make_weather_response(weather_code).icon(), expected);
    }

    #[test_case(
        r#"{"_error": true, "reason": "Parameter out of range"}"#,
        "Parameter out of range"
    )]
    #[test_case(r#"{"_error": true}"#, "Unknown error")]
    fn test_api_response_error(json: &str, expected_msg: &str) {
        let data: ApiResponse<OpenMeteoResponse> = serde_json::from_str(json).unwrap();
        match data.into_result() {
            Err(RustormyError::ApiReturnedError(msg)) => assert_eq!(msg, expected_msg),
            _ => panic!("Expected ApiReturnedError"),
        }
    }
}
