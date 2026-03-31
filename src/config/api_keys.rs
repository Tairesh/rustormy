use crate::errors::RustormyError;
use crate::models::Provider;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ApiKeys {
    #[serde(default)]
    pub open_weather_map: String,
    #[serde(default)]
    pub world_weather_online: String,
    #[serde(default)]
    pub weather_api: String,
    #[serde(default)]
    pub weather_bit: String,
    #[serde(default)]
    pub tomorrow_io: String,
    #[serde(default)]
    pub open_uv: String,
}

impl ApiKeys {
    pub fn validate(&self, provider: Provider) -> Result<(), RustormyError> {
        let has_api_key = match provider {
            Provider::OpenMeteo | Provider::Yr => return Ok(()),
            Provider::OpenWeatherMap => !self.open_weather_map.is_empty(),
            Provider::WorldWeatherOnline => !self.world_weather_online.is_empty(),
            Provider::WeatherApi => !self.weather_api.is_empty(),
            Provider::WeatherBit => !self.weather_bit.is_empty(),
            Provider::TomorrowIo => !self.tomorrow_io.is_empty(),
        };
        if has_api_key {
            Ok(())
        } else {
            Err(RustormyError::MissingApiKey(provider))
        }
    }
}
