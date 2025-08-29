use crate::config::Config;
use crate::display::translations::ll;
use crate::errors::RustormyError;
use crate::models::{Language, Location, Units, Weather, WeatherConditionIcon};
use crate::weather::GetWeather;
use capitalize::Capitalize;

const GEO_API_URL: &str = "https://api.openweathermap.org/geo/1.0/direct";
const WEATHER_API_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

#[derive(Debug, Default)]
pub struct OpenWeatherMap {
    client: reqwest::Client,
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum GeocodingApiResponse {
    Ok(Vec<GeocodingLocation>),
    Err { message: String },
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
#[serde(untagged)]
enum WeatherApiResponse {
    Ok(WeatherResponseData),
    Err { message: String },
}

#[derive(Debug, serde::Deserialize)]
struct WeatherResponseData {
    weather: Vec<WeatherInfo>,
    main: MainInfo,
    wind: WindInfo,
    rain: Option<PrecipitationInfo>,
    snow: Option<PrecipitationInfo>,
    name: Option<String>,
}

impl WeatherResponseData {
    pub fn precipitation(&self) -> f64 {
        let rain = self.rain.as_ref().map_or(0.0, |r| r.one_hour);
        let snow = self.snow.as_ref().map_or(0.0, |s| s.one_hour);
        rain + snow
    }

    pub fn description(&self) -> Option<String> {
        self.weather.first().map(|w| w.description.capitalize())
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

    pub fn into_weather(self, config: &Config, location: Location) -> Weather {
        Weather {
            temperature: self.main.temp,
            feels_like: self.main.feels_like,
            humidity: self.main.humidity,
            precipitation: self.precipitation(),
            pressure: self.main.pressure,
            wind_speed: self.wind.speed,
            wind_direction: self.wind.deg,
            description: self
                .description()
                .unwrap_or_else(|| ll(config.language(), "Unknown").to_string()),
            icon: self.icon(),
            location_name: self.name.unwrap_or(location.name),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct WeatherInfo {
    id: u32,
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
    lang: Language,
    appid: &'a str,
}

impl<'a> WeatherAPIRequest<'a> {
    pub fn new(location: &Location, config: &'a Config) -> Result<Self, RustormyError> {
        let api_key = config.api_key().ok_or(RustormyError::MissingApiKey)?;
        Ok(Self {
            lat: location.latitude,
            lon: location.longitude,
            units: config.units(),
            lang: config.language(),
            appid: api_key,
        })
    }
}

#[async_trait::async_trait]
impl GetWeather for OpenWeatherMap {
    async fn get_weather(&self, config: &Config) -> Result<Weather, RustormyError> {
        let location = self.get_location(config).await?;

        let request = WeatherAPIRequest::new(&location, config)?;
        let response = self
            .client
            .get(WEATHER_API_URL)
            .query(&request)
            .send()
            .await?;

        let response: WeatherApiResponse = response.json().await?;
        match response {
            WeatherApiResponse::Err { message } => Err(RustormyError::ApiReturnedError(message)),
            WeatherApiResponse::Ok(data) => Ok(data.into_weather(config, location)),
        }
    }

    async fn lookup_city(&self, city: &str, config: &Config) -> Result<Location, RustormyError> {
        let api_key = config.api_key().ok_or(RustormyError::MissingApiKey)?;

        let response = self
            .client
            .get(GEO_API_URL)
            .query(&[
                ("q", city),
                ("limit", "1"),
                ("appid", api_key),
                ("lang", config.language().code()),
            ])
            .send()
            .await?;

        let response: GeocodingApiResponse = response.json().await?;

        match response {
            GeocodingApiResponse::Err { message } => Err(RustormyError::ApiReturnedError(message)),
            GeocodingApiResponse::Ok(mut locations) => {
                if let Some(location) = locations.pop() {
                    Ok(location.into())
                } else {
                    Err(RustormyError::CityNotFound(city.to_string()))
                }
            }
        }
    }
}
