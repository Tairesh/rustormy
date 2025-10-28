use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
#[clap(rename_all = "snake_case")]
pub enum Provider {
    #[default]
    #[serde(alias = "om")]
    #[value(alias = "om")]
    OpenMeteo,
    #[serde(alias = "owm")]
    #[value(alias = "owm")]
    OpenWeatherMap,
    #[serde(alias = "wwo")]
    #[value(alias = "wwo")]
    WorldWeatherOnline,
    #[serde(alias = "wa")]
    #[value(alias = "wa")]
    WeatherApi,
    #[serde(alias = "wb")]
    #[value(alias = "wb")]
    WeatherBit,
    #[serde(alias = "ti")]
    #[value(alias = "ti")]
    TomorrowIo,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
#[clap(rename_all = "snake_case")]
pub enum Units {
    #[default]
    Metric,
    Imperial,
}

impl Display for Units {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let unit_str = match self {
            Units::Metric => "metric",
            Units::Imperial => "imperial",
        };
        write!(f, "{unit_str}")
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
#[clap(rename_all = "snake_case")]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
#[clap(rename_all = "snake_case")]
pub enum TextMode {
    #[default]
    Full,
    Compact,
    OneLine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WeatherConditionIcon {
    Unknown,
    Clear,
    PartlyCloudy,
    Cloudy,
    LightShowers,
    HeavyShowers,
    LightSnow,
    HeavySnow,
    Thunderstorm,
    Fog,
}

#[derive(Debug, Serialize)]
pub struct Weather {
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: u8,
    pub dew_point: f64,
    pub precipitation: f64,
    pub pressure: u32,
    pub wind_speed: f64,
    pub wind_direction: u16,
    pub uv_index: Option<u8>,
    pub description: String,
    pub icon: WeatherConditionIcon,
    pub location_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, ValueEnum, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Language {
    #[value(name = "en")]
    #[serde(rename = "en", alias = "English")]
    #[default]
    English,
    #[value(name = "ru")]
    #[serde(rename = "ru", alias = "Russian")]
    Russian,
    #[value(name = "es")]
    #[serde(rename = "es", alias = "Spanish")]
    Spanish,
    #[value(name = "ko")]
    #[serde(rename = "ko", alias = "Korean")]
    Korean,
}

impl Language {
    pub fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Russian => "ru",
            Self::Spanish => "es",
            Self::Korean => "ko",
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.code())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum AnsiColor {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightWhite = 97,
}

impl Display for AnsiColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", *self as u8)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorTheme {
    pub label: AnsiColor,
    pub location: AnsiColor,
    pub temperature: AnsiColor,
    pub wind: AnsiColor,
    pub precipitation: AnsiColor,
    pub pressure: AnsiColor,
    pub humidity: AnsiColor,
}
