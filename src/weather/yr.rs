use crate::config::Config;
use crate::display::translations::ll;
use crate::errors::RustormyError;
use crate::models::{Language, Weather, WeatherConditionIcon};
use crate::weather::Location;
use crate::weather::open_meteo::OpenMeteo;
use crate::weather::openuv::get_uv_index;
use crate::weather::tools::{apparent_temperature, dew_point};
use crate::weather::{GetWeather, LookUpCity};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

const YR_API_URL: &str = "https://api.met.no/weatherapi/locationforecast/2.0/compact";
const YR_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Default)]
pub struct Yr {}

#[derive(Debug, Serialize)]
struct YrRequest {
    lat: f64,
    lon: f64,
}

impl YrRequest {
    pub fn new(location: &Location) -> Self {
        Self {
            lat: location.latitude,
            lon: location.longitude,
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct YrResponse {
    pub properties: YrProperties,
}

#[derive(Debug, Deserialize)]
pub struct YrProperties {
    pub timeseries: Vec<YrTimeseries>,
}

#[derive(Debug, Deserialize)]
pub struct YrTimeseries {
    // pub time: String,
    pub data: YrData,
}

#[derive(Debug, Deserialize)]
pub struct YrData {
    pub instant: YrInstant,
    pub next_1_hours: Option<YrNextHours>,
    // pub next_6_hours: Option<YrNextHours>,
    // pub next_12_hours: Option<YrNextHours>,
}

#[derive(Debug, Deserialize)]
pub struct YrInstant {
    pub details: YrDetails,
}

#[derive(Debug, Deserialize)]
pub struct YrDetails {
    pub air_temperature: f64,
    pub relative_humidity: f64,
    pub wind_speed: f64,
    pub wind_from_direction: Option<f64>,
    // pub cloud_area_fraction: Option<f64>,
    pub precipitation_amount: Option<f64>,
    pub air_pressure_at_sea_level: f64,
}

#[derive(Debug, Deserialize)]
pub struct YrNextHours {
    pub summary: YrSummary,
    pub details: YrPrecipitationDetails,
}

#[derive(Debug, Deserialize)]
pub struct YrSummary {
    pub symbol_code: String,
}

#[derive(Debug, Deserialize)]
pub struct YrPrecipitationDetails {
    pub precipitation_amount: Option<f64>,
}

impl YrResponse {
    pub fn into_weather(
        self,
        client: &Client,
        config: &Config,
        location: &Location,
    ) -> Result<Weather, RustormyError> {
        let timeseries = self
            .first_timeseries()
            .ok_or(RustormyError::ApiReturnedError(
                "No timeseries returned".to_string(),
            ))?;
        let details = &timeseries.data.instant.details;
        let next_hours =
            timeseries
                .data
                .next_1_hours
                .as_ref()
                .ok_or(RustormyError::ApiReturnedError(
                    "No forecast data returned".to_string(),
                ))?;
        let description =
            symbol_code_to_description(&next_hours.summary.symbol_code, config.language());
        let icon = symbol_code_to_icon(&next_hours.summary.symbol_code);

        Ok(Weather {
            temperature: details.air_temperature,
            wind_speed: details.wind_speed,
            wind_direction: details
                .wind_from_direction
                .ok_or(RustormyError::ApiReturnedError(
                    "No wind direction returned".to_string(),
                ))?
                .round() as u16,
            uv_index: get_uv_index(client, config, location)?,
            description,
            icon,
            humidity: details.relative_humidity.round() as u8,
            pressure: details.air_pressure_at_sea_level.round() as u32,
            dew_point: dew_point(
                details.air_temperature,
                details.relative_humidity,
                config.units(),
            ),
            feels_like: apparent_temperature(
                details.air_temperature,
                details.wind_speed,
                details.relative_humidity,
            ),
            precipitation: details
                .precipitation_amount
                .unwrap_or_else(|| next_hours.details.precipitation_amount.unwrap_or(0.0)),
            location_name: location.name.clone(),
        })
    }

    #[inline]
    fn first_timeseries(&self) -> Option<&YrTimeseries> {
        self.properties.timeseries.first()
    }
}

impl GetWeather for Yr {
    fn get_weather(&self, client: &Client, config: &Config) -> Result<Weather, RustormyError> {
        let location = get_location(client, config)?;
        let response = client
            .get(YR_API_URL)
            .query(&YrRequest::new(&location))
            .header("User-Agent", YR_USER_AGENT)
            .send()?;
        let data: YrResponse = response.json()?;
        data.into_weather(client, config, &location)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum YrWeatherCode {
    ClearSky,
    Fair,
    PartlyCloudy,
    Cloudy,
    LightRainShowers,
    RainShowers,
    HeavyRainShowers,
    LightRainShowersAndThunder,
    RainShowersAndThunder,
    HeavyRainShowersAndThunder,
    LightSleetShowers,
    SleetShowers,
    HeavySleetShowers,
    LightSleetShowersAndThunder,
    SleetShowersAndThunder,
    HeavySleetShowersAndThunder,
    LightSnowShowers,
    SnowShowers,
    HeavySnowShowers,
    LightSnowShowersAndThunder,
    SnowShowersAndThunder,
    HeavySnowShowersAndThunder,
    LightRain,
    Rain,
    HeavyRain,
    LightRainAndThunder,
    RainAndThunder,
    HeavyRainAndThunder,
    LightSleet,
    Sleet,
    HeavySleet,
    LightSleetAndThunder,
    SleetAndThunder,
    HeavySleetAndThunder,
    LightSnow,
    Snow,
    HeavySnow,
    LightSnowAndThunder,
    SnowAndThunder,
    HeavySnowAndThunder,
    Fog,
}

impl YrWeatherCode {
    fn description(self, lang: Language) -> String {
        let key = match self {
            Self::ClearSky => "Clear",
            Self::Fair => "Mostly clear",
            Self::PartlyCloudy => "Partly cloudy",
            Self::Cloudy => "Cloudy",
            Self::LightRainShowers => "Slight rain showers",
            Self::RainShowers => "Moderate rain showers",
            Self::HeavyRainShowers => "Violent rain showers",
            Self::LightSleetShowers => "Light sleet showers",
            Self::SleetShowers => "Sleet showers",
            Self::HeavySleetShowers => "Heavy sleet showers",
            Self::LightSnowShowers => "Slight snow showers",
            Self::SnowShowers => "Snow showers",
            Self::HeavySnowShowers => "Heavy snow showers",
            Self::LightRain => "Light rain",
            Self::Rain => "Rain",
            Self::HeavyRain => "Heavy rain",
            Self::LightSleet => "Light sleet",
            Self::Sleet => "Sleet",
            Self::HeavySleet => "Heavy sleet",
            Self::LightSnow => "Light snow",
            Self::Snow => "Snow",
            Self::HeavySnow => "Heavy snow",
            Self::Fog => "Fog",
            Self::LightRainShowersAndThunder
            | Self::RainShowersAndThunder
            | Self::HeavyRainShowersAndThunder
            | Self::LightSleetShowersAndThunder
            | Self::SleetShowersAndThunder
            | Self::HeavySleetShowersAndThunder
            | Self::LightSnowShowersAndThunder
            | Self::SnowShowersAndThunder
            | Self::HeavySnowShowersAndThunder
            | Self::LightRainAndThunder
            | Self::RainAndThunder
            | Self::HeavyRainAndThunder
            | Self::LightSleetAndThunder
            | Self::SleetAndThunder
            | Self::HeavySleetAndThunder
            | Self::LightSnowAndThunder
            | Self::SnowAndThunder
            | Self::HeavySnowAndThunder => "Thunderstorm",
        };
        ll(lang, key).to_string()
    }

    fn to_icon(self) -> WeatherConditionIcon {
        match self {
            Self::ClearSky => WeatherConditionIcon::Clear,
            Self::Fair | Self::PartlyCloudy => WeatherConditionIcon::PartlyCloudy,
            Self::Cloudy => WeatherConditionIcon::Cloudy,
            Self::LightRainShowers | Self::RainShowers | Self::LightRain | Self::Rain => {
                WeatherConditionIcon::LightShowers
            }
            Self::HeavyRainShowers
            | Self::HeavyRain
            | Self::HeavySleetShowers
            | Self::HeavySleet => WeatherConditionIcon::HeavyShowers,
            Self::LightSleetShowers
            | Self::SleetShowers
            | Self::LightSleet
            | Self::Sleet
            | Self::LightSnowShowers
            | Self::SnowShowers
            | Self::LightSnow
            | Self::Snow => WeatherConditionIcon::LightSnow,
            Self::HeavySnowShowers | Self::HeavySnow => WeatherConditionIcon::HeavySnow,
            Self::Fog => WeatherConditionIcon::Fog,
            Self::LightRainShowersAndThunder
            | Self::RainShowersAndThunder
            | Self::HeavyRainShowersAndThunder
            | Self::LightSleetShowersAndThunder
            | Self::SleetShowersAndThunder
            | Self::HeavySleetShowersAndThunder
            | Self::LightSnowShowersAndThunder
            | Self::SnowShowersAndThunder
            | Self::HeavySnowShowersAndThunder
            | Self::LightRainAndThunder
            | Self::RainAndThunder
            | Self::HeavyRainAndThunder
            | Self::LightSleetAndThunder
            | Self::SleetAndThunder
            | Self::HeavySleetAndThunder
            | Self::LightSnowAndThunder
            | Self::SnowAndThunder
            | Self::HeavySnowAndThunder => WeatherConditionIcon::Thunderstorm,
        }
    }
}

fn yr_weather_code(code: &str) -> Option<YrWeatherCode> {
    let base = code
        .strip_suffix("_day")
        .or_else(|| code.strip_suffix("_night"))
        .unwrap_or(code);
    match base {
        "clearsky" => Some(YrWeatherCode::ClearSky),
        "fair" => Some(YrWeatherCode::Fair),
        "partlycloudy" => Some(YrWeatherCode::PartlyCloudy),
        "cloudy" => Some(YrWeatherCode::Cloudy),
        "lightrainshowers" => Some(YrWeatherCode::LightRainShowers),
        "rainshowers" => Some(YrWeatherCode::RainShowers),
        "heavyrainshowers" => Some(YrWeatherCode::HeavyRainShowers),
        "lightrainshowersandthunder" => Some(YrWeatherCode::LightRainShowersAndThunder),
        "rainshowersandthunder" => Some(YrWeatherCode::RainShowersAndThunder),
        "heavyrainshowersandthunder" => Some(YrWeatherCode::HeavyRainShowersAndThunder),
        "lightsleetshowers" => Some(YrWeatherCode::LightSleetShowers),
        "sleetshowers" => Some(YrWeatherCode::SleetShowers),
        "heavysleetshowers" => Some(YrWeatherCode::HeavySleetShowers),
        // "lightssleet..." is a typo in the legend CSV; match both spellings
        "lightsleetshowersandthunder" | "lightssleetshowersandthunder" => {
            Some(YrWeatherCode::LightSleetShowersAndThunder)
        }
        "sleetshowersandthunder" => Some(YrWeatherCode::SleetShowersAndThunder),
        "heavysleetshowersandthunder" => Some(YrWeatherCode::HeavySleetShowersAndThunder),
        "lightsnowshowers" => Some(YrWeatherCode::LightSnowShowers),
        "snowshowers" => Some(YrWeatherCode::SnowShowers),
        "heavysnowshowers" => Some(YrWeatherCode::HeavySnowShowers),
        // "lightssnow..." is a typo in the legend CSV; match both spellings
        "lightsnowshowersandthunder" | "lightssnowshowersandthunder" => {
            Some(YrWeatherCode::LightSnowShowersAndThunder)
        }
        "snowshowersandthunder" => Some(YrWeatherCode::SnowShowersAndThunder),
        "heavysnowshowersandthunder" => Some(YrWeatherCode::HeavySnowShowersAndThunder),
        "lightrain" => Some(YrWeatherCode::LightRain),
        "rain" => Some(YrWeatherCode::Rain),
        "heavyrain" => Some(YrWeatherCode::HeavyRain),
        "lightrainandthunder" => Some(YrWeatherCode::LightRainAndThunder),
        "rainandthunder" => Some(YrWeatherCode::RainAndThunder),
        "heavyrainandthunder" => Some(YrWeatherCode::HeavyRainAndThunder),
        "lightsleet" => Some(YrWeatherCode::LightSleet),
        "sleet" => Some(YrWeatherCode::Sleet),
        "heavysleet" => Some(YrWeatherCode::HeavySleet),
        "lightsleetandthunder" => Some(YrWeatherCode::LightSleetAndThunder),
        "sleetandthunder" => Some(YrWeatherCode::SleetAndThunder),
        "heavysleetandthunder" => Some(YrWeatherCode::HeavySleetAndThunder),
        "lightsnow" => Some(YrWeatherCode::LightSnow),
        "snow" => Some(YrWeatherCode::Snow),
        "heavysnow" => Some(YrWeatherCode::HeavySnow),
        "lightsnowandthunder" => Some(YrWeatherCode::LightSnowAndThunder),
        "snowandthunder" => Some(YrWeatherCode::SnowAndThunder),
        "heavysnowandthunder" => Some(YrWeatherCode::HeavySnowAndThunder),
        "fog" => Some(YrWeatherCode::Fog),
        _ => None,
    }
}

fn symbol_code_to_description(code: &str, lang: Language) -> String {
    yr_weather_code(code).map_or_else(
        || format!("{} ({code})", ll(lang, "Unknown")),
        |c| c.description(lang),
    )
}

fn symbol_code_to_icon(code: &str) -> WeatherConditionIcon {
    yr_weather_code(code).map_or(WeatherConditionIcon::Unknown, YrWeatherCode::to_icon)
}

fn get_location(client: &Client, config: &Config) -> Result<Location, RustormyError> {
    match (config.coordinates(), config.city()) {
        (Some((lat, lon)), _) => Ok(Location {
            name: config.location_name(),
            latitude: lat,
            longitude: lon,
        }),
        (None, Some(city)) if !city.is_empty() => (OpenMeteo {}).lookup_city_cached(client, config),
        _ => Err(RustormyError::NoLocationProvided),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_API_RESPONSE: &str = include_str!("../../tests/data/yr.json");

    #[test]
    fn test_yr_weather_code_known() {
        assert_eq!(yr_weather_code("clearsky"), Some(YrWeatherCode::ClearSky));
        assert_eq!(
            yr_weather_code("clearsky_day"),
            Some(YrWeatherCode::ClearSky)
        );
        assert_eq!(
            yr_weather_code("clearsky_night"),
            Some(YrWeatherCode::ClearSky)
        );
        assert_eq!(yr_weather_code("fair"), Some(YrWeatherCode::Fair));
        assert_eq!(yr_weather_code("fair_day"), Some(YrWeatherCode::Fair));
        assert_eq!(yr_weather_code("fair_night"), Some(YrWeatherCode::Fair));
        assert_eq!(yr_weather_code("fog"), Some(YrWeatherCode::Fog));
        assert_eq!(yr_weather_code("heavyrain"), Some(YrWeatherCode::HeavyRain));
        assert_eq!(
            yr_weather_code("snowandthunder"),
            Some(YrWeatherCode::SnowAndThunder)
        );
    }

    #[test]
    fn test_yr_weather_code_unknown() {
        assert_eq!(yr_weather_code("notacode"), None);
        assert_eq!(yr_weather_code(""), None);
    }

    #[test]
    fn test_yr_weather_code_csv_typos() {
        assert_eq!(
            yr_weather_code("lightssleetshowersandthunder"),
            Some(YrWeatherCode::LightSleetShowersAndThunder)
        );
        assert_eq!(
            yr_weather_code("lightssnowshowersandthunder"),
            Some(YrWeatherCode::LightSnowShowersAndThunder)
        );
    }

    #[test]
    fn test_fair_night_description_and_icon() {
        assert_eq!(
            symbol_code_to_description("fair_night", Language::English),
            "Mostly clear"
        );
        assert_eq!(
            symbol_code_to_icon("fair_night"),
            WeatherConditionIcon::PartlyCloudy
        );
    }

    #[test]
    fn test_unknown_code_fallback() {
        assert_eq!(
            symbol_code_to_description("xyzzy", Language::English),
            "Unknown (xyzzy)"
        );
        assert_eq!(symbol_code_to_icon("xyzzy"), WeatherConditionIcon::Unknown);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_parse_yr_response() {
        let data: YrResponse =
            serde_json::from_str(TEST_API_RESPONSE).expect("Failed to parse JSON");
        let weather = data
            .into_weather(
                &Client::new(),
                &Config::default(),
                &Location {
                    name: "Test Location".to_string(),
                    latitude: 0.0,
                    longitude: 0.0,
                },
            )
            .expect("Failed to convert to Weather");

        assert_eq!(weather.temperature, 6.4);
        assert_eq!(weather.humidity, 94);
        assert_eq!(weather.description, "Heavy rain".to_string());
        assert_eq!(weather.icon, WeatherConditionIcon::HeavyShowers);
        assert_eq!(weather.dew_point, 5.4);
        assert_eq!(weather.precipitation, 1.2);
    }
}
