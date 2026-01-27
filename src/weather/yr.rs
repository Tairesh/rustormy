use crate::config::Config;
use crate::display::translations::ll;
use crate::errors::RustormyError;
use crate::models::{Language, Weather, WeatherConditionIcon};
use crate::weather::GetWeather;
use crate::weather::Location;
use crate::weather::openuv::get_uv_index;
use crate::weather::tools::{apparent_temperature, dew_point};
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
        let next_hours = timeseries.data.next_1_hours.as_ref().unwrap();
        let description =
            symbol_code_to_description(&next_hours.summary.symbol_code, config.language());
        let icon = symbol_code_to_icon(&next_hours.summary.symbol_code);

        Ok(Weather {
            temperature: details.air_temperature,
            wind_speed: details.wind_speed,
            wind_direction: details.wind_from_direction.unwrap().round() as u16,
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
        let location = get_location(config)?;
        let response = client
            .get(YR_API_URL)
            .query(&YrRequest::new(&location))
            .header("User-Agent", YR_USER_AGENT)
            .send()?;
        let data: YrResponse = response.json()?;
        data.into_weather(client, config, &location)
    }
}

fn symbol_code_to_description(code: &str, lang: Language) -> String {
    match code {
        "clearsky" | "clearsky_day" | "clearsky_night" => ll(lang, "Clear sky").to_string(),
        "partlycloudy" | "partlycloudy_day" | "partlycloudy_night" => {
            ll(lang, "Partly cloudy").to_string()
        }
        "cloudy" => ll(lang, "Cloudy").to_string(),
        "rain" | "lightrain" | "lightrainshowers_day" => ll(lang, "Rain").to_string(),
        "heavyrain" => ll(lang, "Heavy rain").to_string(),
        "snow" | "lightsnow" | "heavysnow" => ll(lang, "Snow").to_string(),
        "fog" => ll(lang, "Fog").to_string(),
        _ => format!("{} ({code})", ll(lang, "Unknown")),
    }
}

fn symbol_code_to_icon(code: &str) -> WeatherConditionIcon {
    match code {
        "clearsky" | "clearsky_day" | "clearsky_night" => WeatherConditionIcon::Clear,
        "partlycloudy" | "partlycloudy_day" | "partlycloudy_night" => {
            WeatherConditionIcon::PartlyCloudy
        }
        "cloudy" => WeatherConditionIcon::Cloudy,
        "rain" | "lightrain" | "lightrainshowers_day" => WeatherConditionIcon::LightShowers,
        "heavyrain" => WeatherConditionIcon::HeavyShowers,
        "snow" | "lightsnow" => WeatherConditionIcon::LightSnow,
        "heavysnow" => WeatherConditionIcon::HeavySnow,
        "fog" => WeatherConditionIcon::Fog,
        _ => WeatherConditionIcon::Unknown,
    }
}

fn get_location(config: &Config) -> Result<Location, RustormyError> {
    match (config.coordinates(), config.city()) {
        (Some((lat, lon)), _) => Ok(Location {
            name: config.location_name(),
            latitude: lat,
            longitude: lon,
        }),
        (None, Some(city)) if !city.is_empty() => Err(RustormyError::InvalidConfiguration(
            "City name lookup not implemented for Yr provider",
        )),
        _ => Err(RustormyError::NoLocationProvided),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_API_RESPONSE: &str = include_str!("../../tests/data/yr.json");

    #[test]
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
