use crate::config::Config;
use crate::errors::RustormyError;
use crate::models::{Location, Weather};
use enum_dispatch::enum_dispatch;
pub use provider::GetWeatherProvider;

#[enum_dispatch]
pub trait GetWeather {
    fn get_weather(&self, config: &Config) -> Result<Weather, RustormyError>;
    fn lookup_city(&self, city: &str, config: &Config) -> Result<Location, RustormyError>;

    fn get_location(&self, config: &Config) -> Result<Location, RustormyError> {
        match (config.coordinates(), config.city()) {
            (Some((lat, lon)), _) => Ok(Location {
                name: config.location_name(),
                latitude: lat,
                longitude: lon,
            }),
            (None, Some(city)) => self.lookup_city(city, config),
            // Should not reach here due to prior validation
            (None, None) => Err(RustormyError::NoLocationProvided),
        }
    }
}

mod open_meteo;
mod open_weather_map;
mod provider;
mod world_weather_online;
