use super::{GetWeather, OpenMeteo, OpenWeatherMap, RustormyError, Weather};
use crate::config::Config;
use crate::models::{Location, Provider};
use enum_dispatch::enum_dispatch;

#[enum_dispatch(GetWeather)]
pub enum GetWeatherProvider {
    OpenMeteo,
    OpenWeatherMap,
}

impl From<Provider> for GetWeatherProvider {
    fn from(provider: Provider) -> Self {
        match provider {
            Provider::OpenMeteo => OpenMeteo::default().into(),
            Provider::OpenWeatherMap => OpenWeatherMap::default().into(),
        }
    }
}

impl GetWeatherProvider {
    pub fn new(provider_type: Provider) -> Self {
        provider_type.into()
    }
}
