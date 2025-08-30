use super::{GetWeather, RustormyError, Weather};
use crate::config::Config;
use crate::models::{Location, Provider};
use crate::weather::open_meteo::OpenMeteo;
use crate::weather::open_weather_map::OpenWeatherMap;
use crate::weather::world_weather_online::WorldWeatherOnline;
use enum_dispatch::enum_dispatch;

#[enum_dispatch(GetWeather)]
pub enum GetWeatherProvider {
    OpenMeteo,
    OpenWeatherMap,
    WorldWeatherOnline,
}

impl From<Provider> for GetWeatherProvider {
    fn from(provider: Provider) -> Self {
        match provider {
            Provider::OpenMeteo => OpenMeteo::default().into(),
            Provider::OpenWeatherMap => OpenWeatherMap::default().into(),
            Provider::WorldWeatherOnline => WorldWeatherOnline::default().into(),
        }
    }
}

impl GetWeatherProvider {
    pub fn new(provider_type: Provider) -> Self {
        provider_type.into()
    }
}
