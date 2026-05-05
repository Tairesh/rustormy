use super::{GetWeather, RustormyError, Weather};
use crate::config::Config;
use crate::models::Provider;
use enum_dispatch::enum_dispatch;
use open_meteo::OpenMeteo;
use open_weather_map::OpenWeatherMap;
use reqwest::blocking::Client;
use tomorrow_io::TomorrowIo;
use weather_api::WeatherApi;
use weather_bit::WeatherBit;
use world_weather_online::WorldWeatherOnline;
use yr::Yr;

mod open_meteo;
mod open_weather_map;
mod tomorrow_io;
mod weather_api;
mod weather_bit;
mod world_weather_online;
mod yr;

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
    WeatherBit,
    TomorrowIo,
    Yr,
}

provider_conversions!(
    GetWeatherProvider,
    Provider,
    OpenMeteo,
    OpenWeatherMap,
    WorldWeatherOnline,
    WeatherApi,
    WeatherBit,
    TomorrowIo,
    Yr
);

impl GetWeatherProvider {
    pub fn new(provider_type: Provider) -> Self {
        provider_type.into()
    }
}
