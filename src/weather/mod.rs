use crate::config::Config;
use crate::errors::RustormyError;
use crate::models::{Location, Weather};
use enum_dispatch::enum_dispatch;
pub use provider::GetWeatherProvider;

#[enum_dispatch]
pub trait GetWeather {
    fn get_weather(&self, config: &Config) -> Result<Weather, RustormyError>;
}

pub trait LookUpCity {
    fn lookup_city(&self, config: &Config) -> Result<Location, RustormyError>;
}

mod open_meteo;
mod open_weather_map;
mod provider;
mod world_weather_online;
