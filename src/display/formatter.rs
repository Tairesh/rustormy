use crate::config::Config;
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

fn colored_text(text: impl Into<String>, color: AnsiColor, use_colors: bool) -> String {
    if use_colors {
        format!("\x1b[{}m{}\x1b[0m", color, text.into())
    } else {
        text.into()
    }
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
            eprintln!(
                "{} {}",
                colored_text("Error:", AnsiColor::Red, self.config.use_colors()),
                colored_text(
                    format!("{error}"),
                    AnsiColor::BrightRed,
                    self.config.use_colors()
                )
            );
        }
        std::process::exit(1);
    }

    fn display_text(&self, weather: Weather) {
        let icon = if self.config.use_colors() {
            weather.icon.colored_icon()
        } else {
            weather.icon.icon()
        };

        let use_colors = self.config.use_colors();

        let (temp_unit, wind_unit, precip_unit) = match self.config.units() {
            Units::Metric => ("C", "m/s", "mm"),
            Units::Imperial => ("F", "mph", "inch"),
        };

        let color_for_desc = |cond| match cond {
            WeatherConditionIcon::Unknown | WeatherConditionIcon::Fog => AnsiColor::White,
            WeatherConditionIcon::Sunny => AnsiColor::BrightYellow,
            WeatherConditionIcon::PartlyCloudy | WeatherConditionIcon::Cloudy => AnsiColor::Yellow,
            WeatherConditionIcon::LightShowers | WeatherConditionIcon::HeavyShowers => {
                AnsiColor::BrightBlue
            }
            WeatherConditionIcon::LightSnow | WeatherConditionIcon::HeavySnow => AnsiColor::Cyan,
            WeatherConditionIcon::Thunderstorm => AnsiColor::BrightRed,
        };

        if self.config.show_city_name() {
            eprintln!(
                "{} {} {}",
                icon[0],
                self.label("Location:"),
                colored_text(weather.location_name, AnsiColor::White, use_colors)
            );
        } else {
            eprintln!("{}", icon[0]);
        }

        let print_line = |i: &str, label: &str, value: String| {
            println!("{} {} {}", i, self.label(label), value);
        };

        print_line(
            icon[1],
            "Weather:",
            colored_text(
                &weather.description,
                color_for_desc(weather.icon),
                use_colors,
            ),
        );

        println!(
            "{} {} {} {}",
            icon[2],
            self.label("Temp:"),
            colored_text(
                format!("{}°{temp_unit}", weather.temperature),
                AnsiColor::BrightYellow,
                use_colors
            ),
            colored_text(
                format!("(feels like {}°{temp_unit})", weather.feels_like),
                AnsiColor::Yellow,
                use_colors
            )
        );

        print_line(
            icon[3],
            "Wind:",
            colored_text(
                format!(
                    "{} {wind_unit} at {}°",
                    weather.wind_speed, weather.wind_direction
                ),
                AnsiColor::BrightRed,
                use_colors,
            ),
        );

        println!(
            "{} {} {} | {}",
            icon[4],
            self.label("Humidity:"),
            colored_text(
                format!("{}%", weather.humidity),
                AnsiColor::Cyan,
                use_colors
            ),
            colored_text(
                format!("{} {precip_unit}", weather.precipitation),
                AnsiColor::BrightBlue,
                use_colors
            )
        );

        print_line(
            icon[5],
            "Pressure:",
            colored_text(
                format!("{} hPa", weather.pressure),
                AnsiColor::Green,
                use_colors,
            ),
        );

        println!("{}", icon[6]);
    }

    fn display_json(&self, weather: &Weather) {
        let json = serde_json::to_string_pretty(weather).unwrap_or_else(|e| {
            self.display_error(&RustormyError::Other(anyhow::anyhow!(
                "Failed to serialize weather data to JSON: {e}"
            )))
        });
        println!("{json}");
    }

    fn label(&self, text: &str) -> String {
        colored_text(
            format!("{text:<9}"),
            AnsiColor::Blue,
            self.config.use_colors(),
        )
    }
}
