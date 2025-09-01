use crate::cache::{cache_location, get_cached_location};
use crate::config::Config;
use crate::display::translations::ll;
use crate::errors::RustormyError;
use crate::models::{Language, Location, Units, Weather, WeatherConditionIcon};
use crate::weather::{GetWeather, LookUpCity};
use reqwest::blocking::Client;
use serde::Deserialize;

const GEO_API_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";
const WEATHER_API_URL: &str = "https://api.open-meteo.com/v1/forecast";
const WEATHER_API_FIELDS: &str = "temperature_2m,apparent_temperature,relative_humidity_2m,precipitation,surface_pressure,wind_speed_10m,wind_direction_10m,weather_code";

#[derive(Debug, Default)]
pub struct OpenMeteo {}

#[derive(Debug, Deserialize)]
struct OpenMeteoResponse {
    current: CurrentWeather,
    error: Option<bool>,
    reason: Option<String>,
}

impl OpenMeteoResponse {
    pub fn is_error(&self) -> bool {
        self.error.unwrap_or(false)
    }

    pub fn error_reason(&self) -> String {
        self.reason
            .clone()
            .unwrap_or_else(|| "Unknown error".to_string())
    }

    pub fn description(&self, lang: Language) -> &'static str {
        ll(
            lang,
            match self.current.weather_code {
                0 => "Clear sky",
                1 => "Mainly clear",
                2 => "Partly cloudy",
                3 => "Overcast",
                45 => "Fog",
                48 => "Depositing rime fog",
                51 => "Light drizzle",
                53 => "Moderate drizzle",
                55 => "Dense drizzle",
                56 => "Light freezing drizzle",
                57 => "Dense freezing drizzle",
                61 => "Slight rain",
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

    pub fn icon(&self) -> WeatherConditionIcon {
        match self.current.weather_code {
            0 => WeatherConditionIcon::Sunny,
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

#[derive(Debug, Deserialize)]
struct GeocodingResponse {
    results: Option<Vec<GeocodingLocation>>,
    error: Option<bool>,
    reason: Option<String>,
}

impl GeocodingResponse {
    pub fn is_error(&self) -> bool {
        self.error.unwrap_or(false)
    }

    pub fn error_reason(&self) -> String {
        self.reason
            .clone()
            .unwrap_or_else(|| "Unknown error".to_string())
    }
}

#[derive(Debug, Deserialize)]
struct GeocodingLocation {
    name: String,
    latitude: f64,
    longitude: f64,
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

impl LookUpCity for OpenMeteo {
    fn lookup_city(&self, client: &Client, config: &Config) -> Result<Location, RustormyError> {
        let city = config.city().ok_or(RustormyError::NoLocationProvided)?;
        if config.use_geocoding_cache() {
            let cached_location = get_cached_location(city, config.language())?;
            if let Some(location) = cached_location {
                return Ok(location);
            }
        }

        let response = client
            .get(GEO_API_URL)
            .query(&[
                ("name", city),
                ("count", "1"),
                ("language", config.language().code()),
            ])
            .send()?;

        let data: GeocodingResponse = response.json()?;

        if data.is_error() {
            return Err(RustormyError::ApiReturnedError(data.error_reason()));
        }

        let location = data
            .results
            .and_then(|mut results| results.pop())
            .ok_or_else(|| RustormyError::CityNotFound(city.to_string()))?
            .into();

        if config.use_geocoding_cache() {
            cache_location(city, config.language(), &location)?;
        }

        Ok(location)
    }
}

impl GetWeather for OpenMeteo {
    fn get_weather(&self, client: &Client, config: &Config) -> Result<Weather, RustormyError> {
        let location = self.get_location(client, config)?;

        let (temperature_unit, wind_speed_unit, precipitation_unit) = match config.units() {
            Units::Metric => ("celsius", "ms", "mm"),
            Units::Imperial => ("fahrenheit", "mph", "inch"),
        };

        let response = client
            .get(WEATHER_API_URL)
            .query(&WeatherAPIRequest {
                latitude: location.latitude,
                longitude: location.longitude,
                current: WEATHER_API_FIELDS,
                temperature_unit,
                wind_speed_unit,
                precipitation_unit,
            })
            .send()?;

        let data: OpenMeteoResponse = response.json()?;

        if data.is_error() {
            return Err(RustormyError::ApiReturnedError(data.error_reason()));
        }

        Ok(Weather {
            temperature: data.current.temperature,
            feels_like: data.current.apparent_temperature,
            humidity: data.current.humidity,
            precipitation: data.current.precipitation,
            pressure: data.current.pressure as u32,
            wind_speed: data.current.wind_speed,
            wind_direction: data.current.wind_direction,
            uv_index: None,
            description: data.description(config.language()).to_string(),
            icon: data.icon(),
            location_name: location.name,
        })
    }
}
