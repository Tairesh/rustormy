use crate::config::Config;
use crate::errors::RustormyError;
use crate::models::{Weather, WeatherConditionIcon};
use crate::weather::GetWeather;

pub struct OpenWeatherMapProvider {
    client: reqwest::Client,
}

#[derive(Debug, serde::Deserialize)]
struct Location {
    lat: f64,
    lon: f64,
}

#[derive(Debug, serde::Deserialize)]
struct WeatherResponse {
    weather: Vec<WeatherInfo>,
    main: MainInfo,
    wind: WindInfo,
    rain: Option<PrecipitationInfo>,
    snow: Option<PrecipitationInfo>,
    name: Option<String>,
}

impl WeatherResponse {
    pub fn precipitation(&self) -> f64 {
        let rain = self.rain.as_ref().map_or(0.0, |r| r.one_hour);
        let snow = self.snow.as_ref().map_or(0.0, |s| s.one_hour);
        rain + snow
    }

    pub fn description(&self) -> String {
        if let Some(weather) = self.weather.first() {
            format!("{} ({})", weather.main, weather.description)
        } else {
            "Unknown".to_string()
        }
    }

    pub fn icon(&self) -> WeatherConditionIcon {
        if let Some(weather) = self.weather.first() {
            WeatherConditionIcon::from_owm_code(weather.id)
        } else {
            WeatherConditionIcon::Unknown
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct WeatherInfo {
    id: u32,
    main: String,
    description: String,
}

#[derive(Debug, serde::Deserialize)]
struct MainInfo {
    temp: f64,
    feels_like: f64,
    humidity: u8,
    pressure: u32,
}

#[derive(Debug, serde::Deserialize)]
struct WindInfo {
    speed: f64,
    deg: u16,
}

#[derive(Debug, serde::Deserialize)]
struct PrecipitationInfo {
    #[serde(rename = "1h")]
    one_hour: f64,
}

impl OpenWeatherMapProvider {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    async fn lookup_city(&self, city: &str, config: &Config) -> Result<(f64, f64), RustormyError> {
        let url = format!(
            "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",
            urlencoding::encode(city),
            config.api_key().ok_or(RustormyError::MissingApiKey)?
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(RustormyError::RequestFailed)?;

        let data: Vec<Location> = response
            .json()
            .await
            .map_err(RustormyError::RequestFailed)?;

        if let Some(location) = data.first() {
            Ok((location.lat, location.lon))
        } else {
            Err(RustormyError::CityNotFound(city.to_string()))
        }
    }
}
impl WeatherConditionIcon {
    pub fn from_owm_code(code: u32) -> Self {
        match code {
            200..=232 => WeatherConditionIcon::Thunderstorm,
            300..=321 | 500 | 520 => WeatherConditionIcon::LightShowers,
            500..=531 => WeatherConditionIcon::HeavyShowers,
            600 | 612 | 615 | 620 => WeatherConditionIcon::LightSnow,
            601..=622 => WeatherConditionIcon::HeavySnow,
            701..=781 => WeatherConditionIcon::Fog,
            800 => WeatherConditionIcon::Sunny,
            801 | 802 => WeatherConditionIcon::PartlyCloudy,
            803 | 804 => WeatherConditionIcon::Cloudy,
            _ => WeatherConditionIcon::Unknown,
        }
    }
}

#[async_trait::async_trait]
impl GetWeather for OpenWeatherMapProvider {
    async fn get_weather(&self, config: &Config) -> anyhow::Result<Weather, RustormyError> {
        let (lat, lon) = if let Some(coords) = config.coordinates() {
            coords
        } else if let Some(city) = config.city() {
            self.lookup_city(city, config).await?
        } else {
            // Should not reach here due to prior validation
            return Err(RustormyError::NoLocationProvided);
        };

        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&units={}&appid={}",
            config.units(),
            config.api_key().ok_or(RustormyError::MissingApiKey)?
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(RustormyError::RequestFailed)?;

        let data: WeatherResponse = response
            .json()
            .await
            .map_err(RustormyError::RequestFailed)?;

        let weather = Weather {
            temperature: data.main.temp,
            feels_like: data.main.feels_like,
            humidity: data.main.humidity,
            precipitation: data.precipitation(),
            pressure: data.main.pressure,
            wind_speed: data.wind.speed,
            wind_direction: data.wind.deg,
            description: data.description(),
            icon: data.icon(),
            city: data.name.or(config.city().map(String::from)),
        };

        Ok(weather)
    }
}
