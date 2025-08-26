use super::{GetWeather, OpenMeteoProvider, OpenWeatherMapProvider, RustormyError, Weather};
use crate::config::Config;
use crate::models::{Location, Provider};

pub enum WeatherProvider {
    OpenMeteo(OpenMeteoProvider),
    OpenWeatherMap(OpenWeatherMapProvider),
}

impl WeatherProvider {
    pub fn new(provider_type: Provider) -> Self {
        match provider_type {
            Provider::OpenMeteo => Self::OpenMeteo(OpenMeteoProvider::default()),
            Provider::OpenWeatherMap => Self::OpenWeatherMap(OpenWeatherMapProvider::default()),
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

    async fn lookup_city(
        &self,
        city: &str,
        config: &Config,
    ) -> anyhow::Result<Location, RustormyError> {
        match self {
            Self::OpenMeteo(provider) => provider.lookup_city(city, config).await,
            Self::OpenWeatherMap(provider) => provider.lookup_city(city, config).await,
        }
    }
}
