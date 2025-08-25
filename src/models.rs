use crate::display::icons::WeatherCondition;
use clap::ValueEnum;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum Provider {
    OpenMeteo,
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
    pub condition: WeatherCondition,
    pub city: Option<String>,
}
