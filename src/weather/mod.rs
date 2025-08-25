use crate::config::Config;
use crate::errors::RustormyError;
use crate::models::Weather;
use anyhow::Result;
pub use open_meteo::OpenMeteoProvider;
pub use open_weather_map::OpenWeatherMapProvider;
pub use provider::WeatherProvider;

#[async_trait::async_trait]
pub trait GetWeather {
    async fn get_weather(&self, config: &Config) -> Result<Weather, RustormyError>;
}

mod open_meteo;
mod open_weather_map;
mod provider;
