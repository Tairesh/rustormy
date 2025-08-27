use crate::config::Config;
use crate::display::translations::ll;
use crate::errors::RustormyError;
use crate::models::{OutputFormat, Units, Weather, WeatherConditionIcon};
use std::fmt::Display;

pub struct WeatherFormatter {
    config: Config,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum AnsiColor {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

impl From<WeatherConditionIcon> for AnsiColor {
    fn from(icon: WeatherConditionIcon) -> Self {
        match icon {
            WeatherConditionIcon::Unknown
            | WeatherConditionIcon::Fog
            | WeatherConditionIcon::Cloudy => AnsiColor::White,
            WeatherConditionIcon::Sunny => AnsiColor::BrightYellow,
            WeatherConditionIcon::PartlyCloudy => AnsiColor::Yellow,
            WeatherConditionIcon::LightShowers | WeatherConditionIcon::HeavyShowers => {
                AnsiColor::BrightBlue
            }
            WeatherConditionIcon::LightSnow | WeatherConditionIcon::HeavySnow => AnsiColor::Cyan,
            WeatherConditionIcon::Thunderstorm => AnsiColor::BrightRed,
        }
    }
}

fn make_line(
    i: &str,
    l: &'static str,
    value: impl Display,
    color: AnsiColor,
    config: &Config,
) -> String {
    let value = if config.use_colors() {
        colored_text(value, color)
    } else {
        value.to_string()
    };

    if config.compact_mode() {
        format!("{i} {value}")
    } else {
        format!("{i} {} {value}", label(l, config))
    }
}

fn label(text: &'static str, config: &Config) -> String {
    let lang = config.language();
    let translated = ll(lang, text).to_string() + ":";
    if config.use_colors() {
        colored_text(format!("{translated:<12}"), AnsiColor::BrightBlue)
    } else {
        format!("{translated:<12}")
    }
}

fn colored_text(text: impl Display, color: AnsiColor) -> String {
    format!("\x1b[{color}m{text}\x1b[0m")
}

const fn wind_deg_to_symbol(deg: u16) -> &'static str {
    let symbols = ["↑", "↗", "→", "↘", "↓", "↙", "←", "↖"];
    let index = ((deg as f32 + 22.5) / 45.0) as usize % 8;
    symbols[index]
}

impl WeatherFormatter {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn display(&self, weather: Weather) {
        match self.config.output_format() {
            OutputFormat::Json => self.display_json(&weather),
            OutputFormat::Text => self.display_text(weather),
        }
    }

    pub fn display_error(&self, error: &RustormyError) -> ! {
        if self.config.output_format() == OutputFormat::Json {
            let error_json = serde_json::json!({ "error": format!("{}", error) });
            eprintln!("{error_json}");
        } else {
            eprintln!("Error: {error}");
        }
        std::process::exit(1);
    }

    fn display_text(&self, weather: Weather) {
        let lines = self.format_text(weather);
        for line in lines {
            println!("{line}");
        }
    }

    fn format_text(&self, weather: Weather) -> Vec<String> {
        let (compact, colors, name, lang) = (
            self.config.compact_mode(),
            self.config.use_colors(),
            self.config.show_city_name(),
            self.config.language(),
        );
        let (temp_unit, wind_unit, precip_unit) = match self.config.units() {
            Units::Metric => ("°C", ll(lang, "m/s"), ll(lang, "mm")),
            Units::Imperial => ("°F", ll(lang, "mph"), ll(lang, "inch")),
        };
        let icon = if colors {
            weather.icon.colored_icon()
        } else {
            weather.icon.icon()
        };

        let mut output = Vec::with_capacity(if compact { 5 } else { 7 });

        if name {
            output.push(make_line(
                icon[0],
                "Location",
                weather.location_name,
                AnsiColor::BrightWhite,
                &self.config,
            ));
        } else if !compact {
            output.push(icon[0].to_string());
        }

        output.push(make_line(
            icon[1],
            "Condition",
            weather.description,
            weather.icon.into(),
            &self.config,
        ));

        output.push(make_line(
            icon[2],
            "Temperature",
            format!(
                "{}{temp_unit} ({} {}{temp_unit})",
                weather.temperature,
                ll(lang, "feels like"),
                weather.feels_like
            ),
            AnsiColor::BrightYellow,
            &self.config,
        ));

        output.push(make_line(
            icon[3],
            "Wind",
            if self.config.use_degrees_for_wind() {
                format!(
                    "{} {wind_unit} {}°",
                    weather.wind_speed, weather.wind_direction
                )
            } else {
                format!(
                    "{} {wind_unit} {}",
                    weather.wind_speed,
                    wind_deg_to_symbol(weather.wind_direction)
                )
            },
            AnsiColor::BrightRed,
            &self.config,
        ));

        output.push(make_line(
            icon[4],
            "Humidity",
            format!(
                "{}% | {} {precip_unit}",
                weather.humidity, weather.precipitation
            ),
            AnsiColor::Cyan,
            &self.config,
        ));

        output.push(make_line(
            icon[5],
            "Pressure",
            format!("{} {}", weather.pressure, ll(lang, "hPa")),
            AnsiColor::Green,
            &self.config,
        ));

        if !compact {
            output.push(icon[6].to_string());
        }

        output
    }

    fn display_json(&self, weather: &Weather) {
        let json = serde_json::to_string_pretty(weather).unwrap_or_else(|e| {
            self.display_error(&RustormyError::Other(anyhow::anyhow!(
                "Failed to serialize weather data to JSON: {e}"
            )))
        });
        println!("{json}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::models::{Language, Units, WeatherConditionIcon};

    fn sample_weather() -> Weather {
        Weather {
            temperature: 22.5,
            feels_like: 21.0,
            humidity: 60,
            precipitation: 0.5,
            pressure: 1013,
            wind_speed: 5.0,
            wind_direction: 90,
            description: "Partly cloudy".to_string(),
            icon: WeatherConditionIcon::PartlyCloudy,
            location_name: "Test City".to_string(),
        }
    }

    #[test]
    fn test_format_text_default() {
        let weather = sample_weather();
        let config = Config::default();
        let formatter = WeatherFormatter::new(config);
        let lines = formatter.format_text(weather);

        assert_eq!(lines.len(), 7);
        assert_eq!(
            lines[0],
            "             ", // No city name by default);
            "Expected empty line at the top, got '{}'",
            lines[0]
        );
        assert!(
            lines[1].contains("Condition"),
            "Expected 'Condition' in line 1, got '{}'",
            lines[1]
        );
        assert!(
            lines[1].contains("Partly cloudy"),
            "Expected 'Partly cloudy' in line 1, got '{}'",
            lines[1]
        );
        assert!(
            lines[2].contains("Temperature"),
            "Expected 'Temperature' in line 2, got '{}'",
            lines[2]
        );
        assert!(
            lines[2].contains("22.5°C"),
            "Expected '22.5°C' in line 2, got '{}'",
            lines[2]
        );
        assert!(
            lines[3].contains("Wind"),
            "Expected 'Wind' in line 3, got '{}'",
            lines[3]
        );
        assert!(
            lines[3].contains("5 m/s"),
            "Expected '5 m/s' in line 3, got '{}'",
            lines[3]
        );
        assert!(
            lines[3].contains("→"),
            "Expected wind direction symbol in line 3, got '{}'",
            lines[3]
        );
        assert!(
            lines[4].contains("Humidity"),
            "Expected 'Humidity' in line 4, got '{}'",
            lines[4]
        );
        assert!(
            lines[4].contains("60%"),
            "Expected '60%' in line 4, got '{}'",
            lines[4]
        );
        assert!(
            lines[4].contains("0.5 mm"),
            "Expected '0.5 mm' in line 4, got '{}'",
            lines[4]
        );
        assert!(
            lines[5].contains("Pressure"),
            "Expected 'Pressure' in line 5, got '{}'",
            lines[5]
        );
        assert!(
            lines[5].contains("1013 hPa"),
            "Expected '1013 hPa' in line 5, got '{}'",
            lines[5]
        );
        assert_eq!(
            lines[6], "             ",
            "Expected empty line at the end, got '{}'",
            lines[6]
        );
    }

    #[test]
    fn test_format_text_compact() {
        let weather = sample_weather();
        let mut config = Config::default();
        config.set_compact_mode(true);
        let formatter = WeatherFormatter::new(config);
        let lines = formatter.format_text(weather);

        assert_eq!(lines.len(), 5);
        assert!(
            !lines[0].contains("Location"),
            "Expected no 'Location' in line 0, got '{}'",
            lines[0]
        );
        assert!(
            !lines[0].contains("Test City"),
            "Expected no 'Test City' in line 0, got '{}'",
            lines[0]
        );
        assert!(
            !lines[0].contains("Condition"),
            "Expected no 'Condition' label in compact mode, got '{}'",
            lines[0]
        );
        assert!(
            lines[0].contains("Partly cloudy"),
            "Expected 'Partly cloudy' in line 0, got '{}'",
            lines[0]
        );
        assert!(
            !lines[1].contains("Temperature"),
            "Expected no 'Temperature' label in compact mode, got '{}'",
            lines[1]
        );
        assert!(
            lines[1].contains("22.5°C"),
            "Expected '22.5°C' in line 1, got '{}'",
            lines[1]
        );
        assert!(
            !lines[2].contains("Wind"),
            "Expected no 'Wind' label in compact mode, got '{}'",
            lines[2]
        );
        assert!(
            lines[2].contains("5 m/s"),
            "Expected '5 m/s' in line 2, got '{}'",
            lines[2]
        );
        assert!(
            lines[2].contains("→"),
            "Expected wind direction symbol in line 2, got '{}'",
            lines[2]
        );
        assert!(
            !lines[3].contains("Humidity"),
            "Expected no 'Humidity' label in compact mode, got '{}'",
            lines[3]
        );
        assert!(
            lines[3].contains("60%"),
            "Expected '60%' in line 3, got '{}'",
            lines[3]
        );
        assert!(
            lines[3].contains("0.5 mm"),
            "Expected '0.5 mm' in line 3, got '{}'",
            lines[3]
        );
        assert!(
            !lines[4].contains("Pressure"),
            "Expected no 'Pressure' label in compact mode, got '{}'",
            lines[4]
        );
        assert!(
            lines[4].contains("1013 hPa"),
            "Expected '1013 hPa' in line 4, got '{}'",
            lines[4]
        );
    }

    #[test]
    fn test_format_text_with_city_name() {
        let weather = sample_weather();
        let mut config = Config::default();
        config.set_show_city_name(true);
        let formatter = WeatherFormatter::new(config);
        let lines = formatter.format_text(weather);

        assert_eq!(lines.len(), 7);
        assert!(
            lines[0].contains("Location"),
            "Expected 'Location' in line 0, got '{}'",
            lines[0]
        );
        assert!(
            lines[0].contains("Test City"),
            "Expected 'Test City' in line 0, got '{}'",
            lines[0]
        );
    }

    #[test]
    fn test_format_text_with_color() {
        let weather = sample_weather();
        let mut config = Config::default();
        config.set_use_colors(true);
        let formatter = WeatherFormatter::new(config);
        let lines = formatter.format_text(weather);

        assert_eq!(lines.len(), 7);
        // Check colors in every line except the first and the last one
        for line in &lines[1..6] {
            assert!(
                line.contains("\x1b["),
                "Expected ANSI color codes in line, got '{}'",
                line
            );
        }
    }

    #[test]
    fn test_format_text_imperial_units() {
        let weather = sample_weather();
        let mut config = Config::default();
        config.set_units(Units::Imperial);
        let formatter = WeatherFormatter::new(config);
        let lines = formatter.format_text(weather);

        assert_eq!(lines.len(), 7);
        assert!(
            lines[2].contains("°F"),
            "Expected '°F' in temperature line, got '{}'",
            lines[2]
        );
        assert!(
            lines[3].contains("mph"),
            "Expected 'mph' in wind line, got '{}'",
            lines[3]
        );
        assert!(
            lines[4].contains("inch"),
            "Expected 'inch' in precipitation line, got '{}'",
            lines[4]
        );
    }

    #[test]
    fn test_format_text_wind_degrees() {
        let weather = sample_weather();
        let mut config = Config::default();
        config.set_use_degrees_for_wind(true);
        let formatter = WeatherFormatter::new(config);
        let lines = formatter.format_text(weather);

        assert_eq!(lines.len(), 7);
        assert!(
            lines[3].contains("90°"),
            "Expected '90°' in wind line, got '{}'",
            lines[3]
        );
        assert!(
            !lines[3].contains("→"),
            "Did not expect wind direction symbol in wind line, got '{}'",
            lines[3]
        );
    }

    #[test]
    fn test_format_text_different_language() {
        let weather = sample_weather();
        let mut config = Config::default();
        config.set_language(Language::Russian);
        let formatter = WeatherFormatter::new(config);
        let lines = formatter.format_text(weather);

        assert_eq!(lines.len(), 7);
        assert!(
            lines[1].contains("Погода"),
            "Expected 'Погода' in condition line, got '{}'",
            lines[1]
        );
    }
}
