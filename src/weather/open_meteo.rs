use crate::config::Config;
use crate::display::icons::WeatherCondition;
use crate::models::Units;
use crate::weather::{GetWeather, RustormyError, Weather};
use serde::Deserialize;

pub struct OpenMeteoProvider {
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct OpenMeteoResponse {
    current: CurrentWeather,
    error: Option<bool>,
    reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temperature_2m: f64,
    apparent_temperature: f64,
    relative_humidity_2m: f64,
    precipitation: f64,
    surface_pressure: f64,
    wind_speed_10m: f64,
    wind_direction_10m: f64,
    weather_code: u32,
}

#[derive(Debug, Deserialize)]
struct GeocodingResponse {
    results: Option<Vec<Location>>,
    error: Option<bool>,
    reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Location {
    latitude: f64,
    longitude: f64,
}

impl OpenMeteoProvider {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn lookup_city(&self, city: &str) -> Result<(f64, f64), RustormyError> {
        let url = format!(
            "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1",
            urlencoding::encode(city)
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(RustormyError::RequestFailed)?;

        let data: GeocodingResponse = response
            .json()
            .await
            .map_err(RustormyError::RequestFailed)?;

        if data.error.unwrap_or(false) {
            return Err(RustormyError::Other(anyhow::anyhow!(
                "Geocoding API error: {}",
                data.reason.unwrap_or_else(|| "Unknown error".to_string())
            )));
        }

        let location = data
            .results
            .and_then(|mut results| results.pop())
            .ok_or_else(|| RustormyError::CityNotFound(city.to_string()))?;

        Ok((location.latitude, location.longitude))
    }

    fn weather_code_to_description(code: u32) -> String {
        match code {
            0 => "Clear sky".to_string(),
            1 => "Mainly clear".to_string(),
            2 => "Partly cloudy".to_string(),
            3 => "Overcast".to_string(),
            45 => "Fog".to_string(),
            48 => "Depositing rime fog".to_string(),
            51 => "Light drizzle".to_string(),
            53 => "Moderate drizzle".to_string(),
            55 => "Dense drizzle".to_string(),
            56 => "Light freezing drizzle".to_string(),
            57 => "Dense freezing drizzle".to_string(),
            61 => "Slight rain".to_string(),
            63 => "Moderate rain".to_string(),
            65 => "Heavy rain".to_string(),
            66 => "Light freezing rain".to_string(),
            67 => "Heavy freezing rain".to_string(),
            71 => "Slight snow fall".to_string(),
            73 => "Moderate snow fall".to_string(),
            75 => "Heavy snow fall".to_string(),
            77 => "Snow grains".to_string(),
            80 => "Slight rain showers".to_string(),
            81 => "Moderate rain showers".to_string(),
            82 => "Violent rain showers".to_string(),
            85 => "Slight snow showers".to_string(),
            86 => "Heavy snow showers".to_string(),
            95 => "Thunderstorm".to_string(),
            96 => "Thunderstorm with slight hail".to_string(),
            99 => "Thunderstorm with heavy hail".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}

impl WeatherCondition {
    pub fn from_wmo_code(code: u32) -> WeatherCondition {
        match code {
            0 => WeatherCondition::Sunny,
            1..=2 => WeatherCondition::PartlyCloudy,
            3 => WeatherCondition::Cloudy,
            45 | 48 => WeatherCondition::Fog,
            51..=57 | 80 => WeatherCondition::LightShowers,
            61..=67 | 81 | 82 => WeatherCondition::HeavyShowers,
            71..=73 => WeatherCondition::LightSnow,
            75 | 77 | 85 | 86 => WeatherCondition::HeavySnow,
            95 | 96 | 99 => WeatherCondition::Thunderstorm,
            _ => WeatherCondition::Unknown,
        }
    }
}

#[async_trait::async_trait]
impl GetWeather for OpenMeteoProvider {
    async fn get_weather(&self, config: &Config) -> Result<Weather, RustormyError> {
        let (lat, lon) = if let Some(coords) = config.coordinates() {
            coords
        } else if let Some(city) = config.city() {
            self.lookup_city(city).await?
        } else {
            // Should not reach here due to prior validation
            return Err(RustormyError::Other(anyhow::anyhow!(
                "Neither city nor coordinates provided"
            )));
        };

        let (temp_unit, wind_unit, precip_unit) = match config.units() {
            Units::Metric => ("celsius", "ms", "mm"),
            Units::Imperial => ("fahrenheit", "mph", "inch"),
        };

        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&current=temperature_2m,apparent_temperature,relative_humidity_2m,precipitation,surface_pressure,wind_speed_10m,wind_direction_10m,weather_code&temperature_unit={temp_unit}&wind_speed_unit={wind_unit}&precipitation_unit={precip_unit}"
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(RustormyError::RequestFailed)?;

        let data: OpenMeteoResponse = response
            .json()
            .await
            .map_err(RustormyError::RequestFailed)?;

        if data.error.unwrap_or(false) {
            return Err(RustormyError::Other(anyhow::anyhow!(
                "Weather API error: {}",
                data.reason.unwrap_or_else(|| "Unknown error".to_string())
            )));
        }

        Ok(Weather {
            temperature: data.current.temperature_2m,
            feels_like: data.current.apparent_temperature,
            humidity: data.current.relative_humidity_2m as u8,
            precipitation: data.current.precipitation,
            pressure: data.current.surface_pressure as u32,
            wind_speed: data.current.wind_speed_10m,
            wind_direction: data.current.wind_direction_10m as u16,
            description: Self::weather_code_to_description(data.current.weather_code),
            condition: WeatherCondition::from_wmo_code(data.current.weather_code),
            city: config.city().map(String::from),
        })
    }
}
