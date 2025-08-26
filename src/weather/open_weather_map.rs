use crate::config::Config;
use crate::errors::RustormyError;
use crate::models::{Location, Units, Weather, WeatherConditionIcon};
use crate::weather::GetWeather;

const GEO_API_URL: &str = "https://api.openweathermap.org/geo/1.0/direct";
const WEATHER_API_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

#[derive(Debug, Default)]
pub struct OpenWeatherMapProvider {
    client: reqwest::Client,
}

#[derive(Debug, serde::Deserialize)]
struct GeocodingLocation {
    lat: f64,
    lon: f64,
    name: String,
}

impl From<GeocodingLocation> for Location {
    fn from(loc: GeocodingLocation) -> Self {
        Location {
            name: loc.name,
            latitude: loc.lat,
            longitude: loc.lon,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[non_exhaustive]
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
            match weather.id {
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

#[derive(Debug, serde::Serialize)]
struct WeatherAPIRequest<'a> {
    lat: f64,
    lon: f64,
    units: Units,
    appid: &'a str,
}

#[async_trait::async_trait]
impl GetWeather for OpenWeatherMapProvider {
    async fn get_weather(&self, config: &Config) -> anyhow::Result<Weather, RustormyError> {
        let location = self.get_location(config).await?;

        let api_key = config.api_key().ok_or(RustormyError::MissingApiKey)?;
        let response = self
            .client
            .get(WEATHER_API_URL)
            .query(&WeatherAPIRequest {
                lat: location.latitude,
                lon: location.longitude,
                units: config.units(),
                appid: api_key,
            })
            .send()
            .await?;

        let data: WeatherResponse = response.json().await?;

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
            location_name: data.name.unwrap_or(location.name),
        };

        Ok(weather)
    }

    async fn lookup_city(&self, city: &str, config: &Config) -> Result<Location, RustormyError> {
        let api_key = config.api_key().ok_or(RustormyError::MissingApiKey)?;

        let response = self
            .client
            .get(GEO_API_URL)
            .query(&[("q", city), ("limit", "1"), ("appid", api_key)])
            .send()
            .await?;

        let mut data: Vec<GeocodingLocation> = response.json().await?;
        let location = data
            .pop()
            .ok_or_else(|| RustormyError::CityNotFound(city.to_string()))?;

        Ok(location.into())
    }
}
