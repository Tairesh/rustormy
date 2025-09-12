use crate::config::{Config, FormatterConfig};
use crate::display::color::colored_text;
use crate::display::theme::condition_color;
use crate::display::translations::ll;
use crate::errors::RustormyError;
use crate::models::{AnsiColor, OutputFormat, TextMode, Units, Weather};
use std::fmt::Display;

pub struct WeatherFormatter {
    config: FormatterConfig,
}

fn make_line(
    i: &str,
    l: &'static str,
    value: impl Display,
    color: AnsiColor,
    config: &FormatterConfig,
) -> String {
    let value = if config.use_colors {
        colored_text(value, color)
    } else {
        value.to_string()
    };

    if config.text_mode == TextMode::Compact {
        format!("{i} {value}")
    } else {
        format!("{i} {} {value}", label(l, config))
    }
}

fn label(text: &'static str, config: &FormatterConfig) -> String {
    let lang = config.language;
    let translated = ll(lang, text).to_string() + ":";
    let padded = if config.align_right {
        format!("{translated:>12}")
    } else {
        format!("{translated:<12}")
    };
    if config.use_colors {
        colored_text(padded, config.color_theme.label)
    } else {
        padded
    }
}

const fn wind_deg_to_symbol(deg: u16) -> &'static str {
    let symbols = ["↓", "↙", "←", "↖", "↑", "↗", "→", "↘"];
    let index = ((deg as f32 + 22.5) / 45.0) as usize % 8;
    symbols[index]
}

impl WeatherFormatter {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.format().clone(),
        }
    }

    pub fn display(&self, weather: Weather) {
        match self.config.output_format {
            OutputFormat::Json => self.display_json(&weather),
            OutputFormat::Text => self.display_text(weather),
        }
    }

    pub fn display_error(&self, error: &RustormyError) -> ! {
        if self.config.output_format == OutputFormat::Json {
            let error_json = serde_json::json!({ "error": format!("{}", error) });
            eprintln!("{error_json}");
        } else {
            eprintln!("Error: {error}");
        }
        std::process::exit(1);
    }

    fn display_text(&self, weather: Weather) {
        if self.config.text_mode == TextMode::OneLine {
            println!("{}", self.format_one_line(weather));
            return;
        }

        self.format_text(weather)
            .iter()
            .for_each(|line| println!("{line}"));
    }

    fn format_one_line(&self, weather: Weather) -> String {
        let color_theme = &self.config.color_theme;
        let (temp_unit, wind_unit) = match self.config.units {
            Units::Metric => ("°C", ll(self.config.language, "m/s")),
            Units::Imperial => ("°F", ll(self.config.language, "mph")),
        };
        let emoji = weather.icon.emoji();
        let mut temperature = format!("{:.1}{}", weather.temperature, temp_unit);
        if self.config.use_colors {
            temperature = colored_text(temperature, color_theme.temperature);
        }
        let wind = if self.config.wind_in_degrees {
            format!(
                "{:.1} {wind_unit} {}°",
                weather.wind_speed, weather.wind_direction
            )
        } else {
            format!(
                "{:.1} {wind_unit} {}",
                weather.wind_speed,
                wind_deg_to_symbol(weather.wind_direction)
            )
        };
        let wind = if self.config.use_colors {
            colored_text(wind, color_theme.wind)
        } else {
            wind
        };
        let value = format!("{emoji} {temperature} {wind}");

        if self.config.show_city_name {
            let location = if self.config.use_colors {
                colored_text(weather.location_name, color_theme.location)
            } else {
                weather.location_name
            };
            format!("{location}: {value}")
        } else {
            value
        }
    }

    fn format_text(&self, weather: Weather) -> Vec<String> {
        let (compact, colors, name, lang) = (
            self.config.text_mode == TextMode::Compact,
            self.config.use_colors,
            self.config.show_city_name,
            self.config.language,
        );
        let (temp_unit, wind_unit, precip_unit) = match self.config.units {
            Units::Metric => ("°C", ll(lang, "m/s"), ll(lang, "mm")),
            Units::Imperial => ("°F", ll(lang, "mph"), ll(lang, "inch")),
        };
        let icon = if colors {
            weather.icon.colored_icon()
        } else {
            weather.icon.icon()
        };
        let color_theme = &self.config.color_theme;

        let mut output = Vec::with_capacity(if compact { 6 } else { 7 });

        if name {
            output.push(make_line(
                icon[0],
                "Location",
                weather.location_name,
                color_theme.location,
                &self.config,
            ));
        } else if !compact {
            output.push(icon[0].to_string());
        }

        output.push(make_line(
            icon[1],
            "Condition",
            if let Some(uv) = weather.uv_index {
                format!("{} ({} {uv})", weather.description, ll(lang, "UV index"))
            } else {
                weather.description
            },
            condition_color(weather.icon),
            &self.config,
        ));

        output.push(make_line(
            icon[2],
            "Temperature",
            format!(
                "{:.1}{temp_unit} ({} {:.1}{temp_unit})",
                weather.temperature,
                ll(lang, "feels like"),
                weather.feels_like
            ),
            color_theme.temperature,
            &self.config,
        ));

        output.push(make_line(
            icon[3],
            "Wind",
            if self.config.wind_in_degrees {
                format!(
                    "{:.1} {wind_unit} {}°",
                    weather.wind_speed, weather.wind_direction
                )
            } else {
                format!(
                    "{:.1} {wind_unit} {}",
                    weather.wind_speed,
                    wind_deg_to_symbol(weather.wind_direction)
                )
            },
            color_theme.wind,
            &self.config,
        ));

        output.push(make_line(
            icon[4],
            "Precipitation",
            format!("{} {precip_unit}", weather.precipitation),
            color_theme.precipitation,
            &self.config,
        ));

        output.push(make_line(
            icon[5],
            "Pressure",
            format!("{} {}", weather.pressure, ll(lang, "hPa")),
            color_theme.pressure,
            &self.config,
        ));

        output.push(make_line(
            icon[6],
            "Humidity",
            format!(
                "{}% ({} {:.1}{temp_unit})",
                weather.humidity,
                ll(lang, "dew point"),
                weather.dew_point,
            ),
            color_theme.humidity,
            &self.config,
        ));

        output
    }

    fn display_json(&self, weather: &Weather) {
        let json = serde_json::to_string_pretty(weather).unwrap_or_else(|e| {
            self.display_error(&RustormyError::JsonSerializeError(e));
        });
        println!("{json}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::models::{Language, TextMode, Units, WeatherConditionIcon};
    use std::collections::HashMap;

    fn sample_weather() -> Weather {
        Weather {
            temperature: 22.49,
            feels_like: 21.51,
            humidity: 60,
            dew_point: 14.34,
            precipitation: 0.5,
            pressure: 1013,
            wind_speed: 5.0,
            wind_direction: 90,
            uv_index: None,
            description: "Partly cloudy".to_string(),
            icon: WeatherConditionIcon::PartlyCloudy,
            location_name: "Test City".to_string(),
        }
    }

    #[test]
    fn test_format_text_default() {
        let weather = sample_weather();
        let config = Config::default();
        let formatter = WeatherFormatter::new(&config);
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
            !lines[1].contains("UV index"),
            "Did not expect 'UV index' in line 1, got '{}'",
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
            lines[3].contains("5.0 m/s"),
            "Expected '5.0 m/s' in line 3, got '{}'",
            lines[3]
        );
        assert!(
            lines[3].contains("←"),
            "Expected wind direction symbol in line 3, got '{}'",
            lines[3]
        );
        assert!(
            lines[4].contains("Precip"),
            "Expected 'Precip' in line 4, got '{}'",
            lines[4]
        );
        assert!(
            lines[4].contains("0.5 mm"),
            "Expected '0.5 mm' in line 4, got '{}'",
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
        assert!(
            lines[6].contains("Humidity"),
            "Expected 'Humidity' in line 6, got '{}'",
            lines[6]
        );
        assert!(
            lines[6].contains("60%"),
            "Expected '60%' in line 6, got '{}'",
            lines[6]
        );
        assert!(
            lines[6].contains("dew point"),
            "Expected 'dew point' in line 6, got '{}'",
            lines[6]
        );
        assert!(
            lines[6].contains("14.3°C"),
            "Expected '14.3°C' in line 6, got '{}'",
            lines[6]
        );
    }

    #[test]
    fn test_format_text_compact() {
        let weather = sample_weather();
        let mut config = Config::default();
        config.set_format(FormatterConfig {
            text_mode: TextMode::Compact,
            ..Default::default()
        });
        let formatter = WeatherFormatter::new(&config);
        let lines = formatter.format_text(weather);

        assert_eq!(lines.len(), 6);
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
            lines[2].contains("5.0 m/s"),
            "Expected '5.0 m/s' in line 2, got '{}'",
            lines[2]
        );
        assert!(
            lines[2].contains("←"),
            "Expected wind direction symbol in line 2, got '{}'",
            lines[2]
        );
        assert!(
            !lines[3].contains("Precip"),
            "Expected no 'Precip' label in compact mode, got '{}'",
            lines[3]
        );
        assert!(
            lines[3].contains("0.5 mm"),
            "Expected '0.5 mm' in line 3, got '{}'",
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
        assert!(
            !lines[5].contains("Humidity"),
            "Expected no 'Humidity' label in compact mode, got '{}'",
            lines[5]
        );
        assert!(
            lines[5].contains("60%"),
            "Expected '60%' in line 5, got '{}'",
            lines[5]
        );
        assert!(
            lines[5].contains("dew point"),
            "Expected 'dew point' in line 5, got '{}'",
            lines[5]
        );
        assert!(
            lines[5].contains("14.3°C"),
            "Expected '14.3°C' in line 6, got '{}'",
            lines[6]
        );
    }

    #[test]
    fn test_uv_index() {
        let mut weather = sample_weather();
        weather.uv_index = Some(7);
        let config = Config::default();
        let formatter = WeatherFormatter::new(&config);
        let lines = formatter.format_text(weather);

        assert_eq!(lines.len(), 7);
        assert!(
            lines[1].contains("UV index 7"),
            "Expected 'UV index 7' in condition line, got '{}'",
            lines[1]
        );
    }

    #[test]
    fn test_format_text_with_city_name() {
        let weather = sample_weather();
        let mut config = Config::default();
        config.set_format(FormatterConfig {
            show_city_name: true,
            ..Default::default()
        });
        let formatter = WeatherFormatter::new(&config);
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
        config.set_format(FormatterConfig {
            use_colors: true,
            ..Default::default()
        });
        let formatter = WeatherFormatter::new(&config);
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
        config.set_format(FormatterConfig {
            units: Units::Imperial,
            ..Default::default()
        });
        let formatter = WeatherFormatter::new(&config);
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
        config.set_format(FormatterConfig {
            wind_in_degrees: true,
            ..Default::default()
        });
        let formatter = WeatherFormatter::new(&config);
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
    fn test_format_text_wind_degrees_symbol() {
        let test_cases: HashMap<u16, char> = HashMap::from([
            (0, '↓'),
            (45, '↙'),
            (90, '←'),
            (135, '↖'),
            (180, '↑'),
            (225, '↗'),
            (270, '→'),
            (315, '↘'),
        ]);

        for (got, want) in test_cases.into_iter() {
            let mut weather = sample_weather();

            weather.wind_direction = got;

            let mut config = Config::default();
            config.set_format(FormatterConfig {
                wind_in_degrees: false,
                ..Default::default()
            });
            let formatter = WeatherFormatter::new(&config);
            let lines = formatter.format_text(weather);

            assert_eq!(lines.len(), 7);
            assert!(
                lines[3].contains(want),
                "Expected '{}' in wind line, got '{}'",
                want,
                lines[3]
            );
            assert!(
                !lines[3].contains("90°"),
                "Did not expect wind direction symbol in wind line, got '{}'",
                lines[3]
            );
        }
    }

    #[test]
    fn test_format_text_different_language() {
        let weather = sample_weather();
        let mut config = Config::default();
        config.set_format(FormatterConfig {
            language: Language::Russian,
            ..Default::default()
        });
        let formatter = WeatherFormatter::new(&config);
        let lines = formatter.format_text(weather);

        assert_eq!(lines.len(), 7);
        assert!(
            lines[1].contains("Погода"),
            "Expected 'Погода' in condition line, got '{}'",
            lines[1]
        );
    }

    #[test]
    fn test_one_line_mode_with_city_name() {
        let weather = sample_weather();
        let mut config = Config::default();
        config.set_format(FormatterConfig {
            show_city_name: true,
            text_mode: TextMode::OneLine,
            ..Default::default()
        });
        let formatter = WeatherFormatter::new(&config);
        let line = formatter.format_one_line(weather);

        assert!(
            line.contains("Test City"),
            "Expected 'Test City' in one-line output, got '{}'",
            line
        );
        assert!(
            line.contains("⛅"),
            "Expected weather icon in one-line output, got '{}'",
            line
        );
        assert!(
            line.contains("22.5°C"),
            "Expected temperature in one-line output, got '{}'",
            line
        );
        assert!(
            line.contains("5.0 m/s ←"),
            "Expected wind info in one-line output, got '{}'",
            line
        );
    }

    #[test]
    fn test_one_line_mode_without_city_name() {
        let weather = sample_weather();
        let mut config = Config::default();
        config.set_format(FormatterConfig {
            text_mode: TextMode::OneLine,
            ..Default::default()
        });
        let formatter = WeatherFormatter::new(&config);
        let line = formatter.format_one_line(weather);

        assert!(
            !line.contains("Test City"),
            "Did not expect 'Test City' in one-line output, got '{}'",
            line
        );
        assert!(
            line.contains("⛅"),
            "Expected weather icon in one-line output, got '{}'",
            line
        );
        assert!(
            line.contains("22.5°C"),
            "Expected temperature in one-line output, got '{}'",
            line
        );
    }

    #[test]
    fn test_align_right() {
        let weather = sample_weather();
        let mut config = Config::default();
        config.set_format(FormatterConfig {
            align_right: true,
            ..Default::default()
        });
        let formatter = WeatherFormatter::new(&config);
        let lines = formatter.format_text(weather);

        // Check if there are no extra spaces between the label and the value
        assert!(
            lines[1].contains("Condition: Partly cloudy"),
            "Expected 'Condition: Partly cloudy' in line 1, got '{}'",
            lines[1]
        );
        assert!(
            lines[2].contains("Temperature: 22.5°C"),
            "Expected 'Temperature: 22.5°C' in line 2, got '{}'",
            lines[2]
        );
        assert!(
            lines[3].contains("Wind: 5.0 m/s ←"),
            "Expected 'Wind: 5.0 m/s →' in line 3, got '{}'",
            lines[3]
        );
    }

    #[test]
    fn test_uv_index_display() {
        let mut weather = sample_weather();
        weather.uv_index = Some(5);
        let config = Config::default();
        let formatter = WeatherFormatter::new(&config);
        let lines = formatter.format_text(weather);

        assert_eq!(lines.len(), 7);
        assert!(
            lines[1].contains("UV index 5"),
            "Expected 'UV index 5' in condition line, got '{}'",
            lines[1]
        );
    }
}
