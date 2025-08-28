use crate::config::Config;
use crate::errors::RustormyError;
use crate::models::{Location, Weather};
use anyhow::Result;
use enum_dispatch::enum_dispatch;
pub use open_meteo::OpenMeteo;
pub use open_weather_map::OpenWeatherMap;
pub use provider::GetWeatherProvider;

#[async_trait::async_trait]
#[enum_dispatch]
pub trait GetWeather {
    async fn get_weather(&self, config: &Config) -> Result<Weather, RustormyError>;
    async fn lookup_city(&self, city: &str, config: &Config) -> Result<Location, RustormyError>;

    async fn get_location(&self, config: &Config) -> Result<Location, RustormyError> {
        match (config.coordinates(), config.city()) {
            (Some((lat, lon)), _) => Ok(Location {
                name: config.location_name(),
                latitude: lat,
                longitude: lon,
            }),
            (None, Some(city)) => self.lookup_city(city, config).await,
            // Should not reach here due to prior validation
            (None, None) => Err(RustormyError::NoLocationProvided),
        }
    }
}

mod open_meteo;
mod open_weather_map;
mod provider;
