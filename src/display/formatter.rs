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

fn colored_text(text: impl Into<String>, color: AnsiColor, use_colors: bool) -> String {
    if use_colors {
        format!("\x1b[{}m{}\x1b[0m", color, text.into())
    } else {
        text.into()
    }
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

    fn label(&self, text: &'static str) -> String {
        let lang = self.config.language();
        let translated = ll(lang, text).to_string() + ":";
        colored_text(
            format!("{translated:<12}"),
            AnsiColor::BrightBlue,
            self.config.use_colors(),
        )
    }

    fn display_text(&self, weather: Weather) {
        let icon = if self.config.use_colors() {
            weather.icon.colored_icon()
        } else {
            weather.icon.icon()
        };

        let use_colors = self.config.use_colors();
        let lang = self.config.language();

        let (temp_unit, wind_unit, precip_unit) = match self.config.units() {
            Units::Metric => ("°C", ll(lang, "m/s"), ll(lang, "mm")),
            Units::Imperial => ("°F", ll(lang, "mph"), ll(lang, "inch")),
        };

        if self.config.show_city_name() {
            eprintln!(
                "{} {} {}",
                icon[0],
                self.label("Location"),
                colored_text(weather.location_name, AnsiColor::White, use_colors)
            );
        } else {
            eprintln!("{}", icon[0]);
        }

        let print_line = |i: &str, label: &'static str, value: String| {
            println!("{} {} {}", i, self.label(label), value);
        };

        print_line(
            icon[1],
            "Condition",
            colored_text(&weather.description, weather.icon.into(), use_colors),
        );

        println!(
            "{} {} {} {}",
            icon[2],
            self.label("Temperature"),
            colored_text(
                format!("{}{temp_unit}", weather.temperature),
                AnsiColor::BrightYellow,
                use_colors
            ),
            colored_text(
                format!(
                    "({} {}{temp_unit})",
                    ll(lang, "feels like"),
                    weather.feels_like
                ),
                AnsiColor::Yellow,
                use_colors
            )
        );

        print_line(
            icon[3],
            "Wind",
            colored_text(
                format!(
                    "{} {wind_unit} {}",
                    weather.wind_speed,
                    wind_deg_to_symbol(weather.wind_direction)
                ),
                AnsiColor::BrightRed,
                use_colors,
            ),
        );

        println!(
            "{} {} {} | {}",
            icon[4],
            self.label("Humidity"),
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
            "Pressure",
            colored_text(
                format!("{} {}", weather.pressure, ll(lang, "hPa")),
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
}
