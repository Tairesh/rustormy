use crate::display::color::AnsiColor;
use crate::models::WeatherConditionIcon;

pub fn condition_color(icon: WeatherConditionIcon) -> AnsiColor {
    match icon {
        WeatherConditionIcon::Unknown
        | WeatherConditionIcon::Fog
        | WeatherConditionIcon::Cloudy => AnsiColor::White,
        WeatherConditionIcon::Clear => AnsiColor::BrightYellow,
        WeatherConditionIcon::PartlyCloudy => AnsiColor::Yellow,
        WeatherConditionIcon::LightShowers | WeatherConditionIcon::HeavyShowers => {
            AnsiColor::BrightBlue
        }
        WeatherConditionIcon::LightSnow | WeatherConditionIcon::HeavySnow => AnsiColor::Cyan,
        WeatherConditionIcon::Thunderstorm => AnsiColor::BrightRed,
    }
}

#[derive(Debug)]
pub struct ColorTheme {
    pub label: AnsiColor,
    pub location: AnsiColor,
    pub temperature: AnsiColor,
    pub wind: AnsiColor,
    pub precipitation: AnsiColor,
    pub pressure: AnsiColor,
    pub humidity: AnsiColor,
}

impl Default for ColorTheme {
    fn default() -> Self {
        Self::simple()
    }
}

impl ColorTheme {
    pub fn simple() -> Self {
        Self {
            label: AnsiColor::BrightBlue,
            location: AnsiColor::BrightWhite,
            temperature: AnsiColor::BrightYellow,
            wind: AnsiColor::BrightRed,
            precipitation: AnsiColor::BrightCyan,
            pressure: AnsiColor::BrightGreen,
            humidity: AnsiColor::Blue,
        }
    }
}
