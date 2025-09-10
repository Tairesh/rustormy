use crate::config::Config;
use crate::display::translations::ll;
use crate::errors::RustormyError;
use crate::models::{Language, Location, Units, Weather, WeatherConditionIcon};
use crate::weather::openuv::get_uv_index;
use crate::weather::{GetWeather, LookUpCity, tools};
use capitalize::Capitalize;
use reqwest::blocking::Client;

const GEO_API_URL: &str = "https://api.openweathermap.org/geo/1.0/direct";
const WEATHER_API_URL: &str = "https://api.openweathermap.org/data/2.5/weather";

#[derive(Debug, Default)]
pub struct OpenWeatherMap {}

#[derive(Debug, serde::Serialize)]
struct GeocodingApiRequest<'a> {
    q: &'a str,
    limit: u8,
    appid: &'a str,
    lang: &'a str,
}

impl<'a> GeocodingApiRequest<'a> {
    pub fn new(city: &'a str, config: &'a Config) -> Self {
        Self {
            q: city,
            limit: 1,
            appid: &config.api_keys().open_weather_map,
            lang: config.language().code(),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum GeocodingApiResponse {
    Ok(Vec<GeocodingLocation>),
    Err { message: String },
}

impl GeocodingApiResponse {
    pub fn into_location(self) -> Option<Location> {
        if let GeocodingApiResponse::Ok(mut locations) = self {
            return locations.pop().map(Location::from);
        }

        None
    }
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
                800 => WeatherConditionIcon::Clear,
                801 | 802 => WeatherConditionIcon::PartlyCloudy,
                803 | 804 => WeatherConditionIcon::Cloudy,
                _ => WeatherConditionIcon::Unknown,
            }
        } else {
            WeatherConditionIcon::Unknown
        }
    }

    fn dew_point(&self, units: Units) -> f64 {
        let t = self.main.temp;
        let h = self.main.humidity.into();

        tools::dew_point(t, h, units)
    }

    pub fn into_weather(
        self,
        client: &Client,
        config: &Config,
        location: Location,
    ) -> Result<Weather, RustormyError> {
        Ok(Weather {
            temperature: self.main.temp,
            feels_like: self.main.feels_like,
            humidity: self.main.humidity,
            dew_point: self.dew_point(config.units()),
            precipitation: self.precipitation(),
            pressure: self.main.pressure,
            wind_speed: self.wind.speed,
            wind_direction: self.wind.deg,
            uv_index: get_uv_index(client, config, &location)?,
            description: self
                .description()
                .unwrap_or_else(|| ll(config.language(), "Unknown").to_string()),
            icon: self.icon(),
            location_name: self.name.unwrap_or(location.name),
        })
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
    pub fn new(location: &Location, config: &'a Config) -> Self {
        Self {
            lat: location.latitude,
            lon: location.longitude,
            units: config.units(),
            lang: config.language(),
            appid: &config.api_keys().open_weather_map,
        }
    }
}

impl LookUpCity for OpenWeatherMap {
    fn lookup_city(&self, client: &Client, config: &Config) -> Result<Location, RustormyError> {
        let city = config.city().ok_or(RustormyError::NoLocationProvided)?;

        let request = GeocodingApiRequest::new(city, config);
        let response = client.get(GEO_API_URL).query(&request).send()?;
        let data: GeocodingApiResponse = response.json()?;

        if let GeocodingApiResponse::Err { message } = data {
            return Err(RustormyError::ApiReturnedError(message));
        }
        let location = data
            .into_location()
            .ok_or(RustormyError::CityNotFound(city.to_string()))?;

        Ok(location)
    }
}

impl GetWeather for OpenWeatherMap {
    fn get_weather(&self, client: &Client, config: &Config) -> Result<Weather, RustormyError> {
        let location = self.get_location(client, config)?;

        let request = WeatherAPIRequest::new(&location, config);
        let response = client.get(WEATHER_API_URL).query(&request).send()?;

        let response: WeatherApiResponse = response.json()?;
        match response {
            WeatherApiResponse::Err { message } => Err(RustormyError::ApiReturnedError(message)),
            WeatherApiResponse::Ok(data) => Ok(data.into_weather(client, config, location)?),
        }
    }
}
