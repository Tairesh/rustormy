use crate::config::Config;
use crate::errors::RustormyError;
use crate::models::{Location, Weather};
use enum_dispatch::enum_dispatch;
pub use provider::GetWeatherProvider;
use reqwest::blocking::Client;

#[enum_dispatch]
pub trait GetWeather {
    fn get_weather(&self, client: &Client, config: &Config) -> Result<Weather, RustormyError>;
}

pub trait LookUpCity {
    fn lookup_city(&self, client: &Client, config: &Config) -> Result<Location, RustormyError>;

    fn get_location(&self, client: &Client, config: &Config) -> Result<Location, RustormyError> {
        match (config.coordinates(), config.city()) {
            (Some((lat, lon)), _) => Ok(Location {
                name: config.location_name(),
                latitude: lat,
                longitude: lon,
            }),
            (None, Some(city)) if !city.is_empty() => self.lookup_city(client, config),
            _ => Err(RustormyError::NoLocationProvided),
        }
    }
}

mod open_meteo;
mod open_weather_map;
mod provider;
mod world_weather_online;
