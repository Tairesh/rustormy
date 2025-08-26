use clap::ValueEnum;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
#[clap(rename_all = "snake_case")]
pub enum Provider {
    OpenMeteo,
    OpenWeatherMap,
}

impl Default for Provider {
    fn default() -> Self {
        Self::OpenMeteo
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum Units {
    Metric,
    Imperial,
}

impl Default for Units {
    fn default() -> Self {
        Self::Metric
    }
}

impl Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let unit_str = match self {
            Units::Metric => "metric",
            Units::Imperial => "imperial",
        };
        write!(f, "{unit_str}")
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    Text,
    Json,
}

impl Default for OutputFormat {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherConditionIcon {
    Unknown,
    Sunny,
    PartlyCloudy,
    Cloudy,
    LightShowers,
    HeavyShowers,
    LightSnow,
    HeavySnow,
    Thunderstorm,
    Fog,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: u8,
    pub precipitation: f64,
    pub pressure: u32,
    pub wind_speed: f64,
    pub wind_direction: u16,
    pub description: String,
    pub icon: WeatherConditionIcon,
    pub location_name: String,
}

#[derive(Debug)]
pub struct Location {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, ValueEnum, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Language {
    #[value(name = "en")]
    #[default]
    English,
    #[value(name = "ru")]
    Russian,
    #[value(name = "es")]
    Spanish,
}

impl Language {
    pub fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Russian => "ru",
            Self::Spanish => "es",
        }
    }
}
