use crate::models::{AnsiColor, ColorTheme, WeatherConditionIcon};

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
