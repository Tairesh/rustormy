use super::{GetWeather, OpenMeteoProvider, OpenWeatherMapProvider, RustormyError, Weather};
use crate::config::Config;
use crate::models::Provider;

pub enum WeatherProvider {
    OpenMeteo(OpenMeteoProvider),
    OpenWeatherMap(OpenWeatherMapProvider),
}

impl WeatherProvider {
    pub fn new(provider_type: Provider) -> Self {
        match provider_type {
            Provider::OpenMeteo => Self::OpenMeteo(OpenMeteoProvider::new()),
            Provider::OpenWeatherMap => Self::OpenWeatherMap(OpenWeatherMapProvider::new()),
        }
    }
}

#[async_trait::async_trait]
impl GetWeather for WeatherProvider {
    async fn get_weather(&self, config: &Config) -> Result<Weather, RustormyError> {
        match self {
            Self::OpenMeteo(provider) => provider.get_weather(config).await,
            Self::OpenWeatherMap(provider) => provider.get_weather(config).await,
        }
    }
}
