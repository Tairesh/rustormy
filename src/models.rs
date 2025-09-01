use clap::ValueEnum;
use serde::ser::SerializeStruct;
use serde_derive::{Deserialize, Serialize};
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

#[derive(Debug)]
pub struct Weather {
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: u8,
    pub precipitation: f64,
    pub pressure: u32,
    pub wind_speed: f64,
    pub wind_direction: u16,
    pub uv_index: Option<u8>,
    pub description: String,
    pub icon: WeatherConditionIcon,
    pub location_name: String,
}

impl serde::Serialize for Weather {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Weather", 11)?;
        state.serialize_field("temperature", &self.temperature)?;
        state.serialize_field("feels_like", &self.feels_like)?;
        state.serialize_field("humidity", &self.humidity)?;
        state.serialize_field("precipitation", &self.precipitation)?;
        state.serialize_field("pressure", &self.pressure)?;
        state.serialize_field("wind_speed", &self.wind_speed)?;
        state.serialize_field("wind_direction", &self.wind_direction)?;
        state.serialize_field("dew_point", &self.dew_point())?;
        if let Some(uv) = self.uv_index {
            state.serialize_field("uv_index", &uv)?;
        }
        state.serialize_field("description", &self.description)?;
        state.serialize_field("icon", &self.icon)?;
        state.serialize_field("location_name", &self.location_name)?;
        state.end()
    }
}

/// Convert Celsius to Fahrenheit
fn c_to_f(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

impl Weather {
    /// Calculate dew point using the Magnus formula
    pub fn dew_point(&self) -> f64 {
        const B: f64 = 17.625;
        const C: f64 = 243.04;
        let t = self.temperature;
        let h: f64 = self.humidity.into();
        let gamma = (B * t) / (C + t) + (h / 100.0).ln();
        let result = (C * gamma) / (B - gamma);
        (result * 10.0).round() / 10.0 // Round to one decimal place
    }

    /// Dew point in Fahrenheit
    pub fn dew_point_f(&self) -> f64 {
        c_to_f(self.dew_point())
    }
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

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.code())
    }
}
