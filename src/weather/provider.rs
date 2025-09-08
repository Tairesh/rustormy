use super::{GetWeather, RustormyError, Weather};
use crate::config::Config;
use crate::models::Provider;
use crate::weather::open_meteo::OpenMeteo;
use crate::weather::open_weather_map::OpenWeatherMap;
use crate::weather::weather_api::WeatherApi;
use crate::weather::world_weather_online::WorldWeatherOnline;
use enum_dispatch::enum_dispatch;
use reqwest::blocking::Client;

macro_rules! provider_conversions {
    ($enum_impl:ident, $enum_config:ident, $($variant:ident),*) => {
        impl From<$enum_config> for $enum_impl {
            fn from(provider: $enum_config) -> Self {
                match provider {
                    $($enum_config::$variant => $enum_impl::$variant(Default::default()),)*
                }
            }
        }

        impl From<&$enum_impl> for $enum_config {
            fn from(provider: &$enum_impl) -> Self {
                match provider {
                    $($enum_impl::$variant(..) => $enum_config::$variant,)*
                }
            }
        }
    };
}

#[enum_dispatch(GetWeather)]
pub enum GetWeatherProvider {
    OpenMeteo,
    OpenWeatherMap,
    WorldWeatherOnline,
    WeatherApi,
}

provider_conversions!(
    GetWeatherProvider,
    Provider,
    OpenMeteo,
    OpenWeatherMap,
    WorldWeatherOnline,
    WeatherApi
);

impl GetWeatherProvider {
    pub fn new(provider_type: Provider) -> Self {
        provider_type.into()
    }
}
