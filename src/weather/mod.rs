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
            (None, Some(city)) if !city.is_empty() => self.lookup_city_cached(client, config),
            _ => Err(RustormyError::NoLocationProvided),
        }
    }

    fn lookup_city_cached(
        &self,
        client: &Client,
        config: &Config,
    ) -> Result<Location, RustormyError> {
        let city = config.city().ok_or(RustormyError::NoLocationProvided)?;
        if config.use_geocoding_cache() {
            let cached_location = crate::cache::get_cached_location(city, config.language())?;
            if let Some(location) = cached_location {
                return Ok(location);
            }
        }
        let location = self.lookup_city(client, config)?;
        if config.use_geocoding_cache() {
            crate::cache::cache_location(city, config.language(), &location)?;
        }
        Ok(location)
    }
}

mod open_meteo;
mod open_weather_map;
mod openuv;
mod provider;
mod tomorrow_io;
pub mod tools;
mod weather_api;
mod weather_bit;
mod world_weather_online;
mod yr;
