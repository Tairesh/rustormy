use crate::models::{Language, OutputFormat, Provider, TextMode, Units};
use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Deserialize, Serialize)]
pub struct LegacyConfig {
    /// Weather data provider (`open_meteo`, `open_weather_map`, `world_weather_online`, or `weather_api`)
    /// Deprecated, kept for migration purposes. Use `providers` instead.
    #[serde(default)]
    pub provider: Option<Provider>,

    /// List of providers to try in order (if the first fails, try the next, etc.)
    /// Example: `["open_meteo", "open_weather_map", "world_weather_online", "weather_api"]`
    #[serde(default)]
    pub providers: Vec<Provider>,

    /// Deprecated, kept for migration purposes. Will fall back to `api_key_owm` if set.
    #[serde(default)]
    pub api_key: Option<String>,

    /// API key for Open Weather Map
    #[serde(default)]
    pub api_key_owm: String,

    /// API key for World Weather Online
    #[serde(default)]
    pub api_key_wwo: String,

    /// API key for WeatherAPI.com
    #[serde(default)]
    pub api_key_wa: String,

    /// City name (required if lat/lon not provided)
    #[serde(default)]
    pub city: Option<String>,

    /// Latitude (required if city not provided)
    #[serde(default)]
    pub lat: Option<f64>,

    /// Longitude (required if city not provided)
    #[serde(default)]
    pub lon: Option<f64>,

    /// Units for temperature and wind speed (`metric` or `imperial`)
    #[serde(default)]
    pub units: Units,

    /// Output format (`text` or `json`)
    #[serde(default)]
    pub output_format: OutputFormat,

    /// Language code for weather output (e.g., `en`, `es`, `ru`, etc.)
    #[serde(default)]
    pub language: Language,

    /// Show city name in output (`true` or `false`)
    #[serde(default)]
    pub show_city_name: bool,

    /// Use colors in output (`true` or `false`)
    #[serde(default)]
    pub use_colors: bool,

    /// Use degrees for wind direction in output instead of arrows (`true` or `false`)
    #[serde(default, alias = "use_degrees_for_wind")]
    pub wind_in_degrees: bool,

    /// Text mode for text output (`full`, `compact`, or `one_line`)
    #[serde(default)]
    pub text_mode: TextMode,

    /// Deprecated, kept for migration purposes. Will fall back to `text_mode = "compact"`.
    #[serde(default)]
    pub compact_mode: Option<bool>,

    /// Live mode - continuously update weather data every `live_mode_interval` seconds (`true` or `false`)
    #[serde(default)]
    pub live_mode: bool,

    /// Live mode update interval in seconds (default: 300)
    #[serde(default = "default_live_mode_interval")]
    pub live_mode_interval: u64, // in seconds, default to 300 (5 minutes)

    /// Align labels in text output to the right (`true` or `false`)
    /// (Note: only affects text output in `full` mode, not `compact` or `one_line` modes)
    #[serde(default)]
    pub align_right: bool,

    /// Use geocoding cache (`true` or `false`)
    /// (if enabled, previously looked up cities will be cached locally to avoid repeated API calls)
    #[serde(default)]
    pub use_geocoding_cache: bool,

    /// Verbosity level of output (0 = errors, 1 = warnings, 2 = info, 3 = debug)
    #[serde(default)]
    pub verbose: u8,

    /// API HTTP client timeout in seconds
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout: u64, // in seconds, default to 10
}

fn default_live_mode_interval() -> u64 {
    300
}
fn default_connect_timeout() -> u64 {
    10
}

impl Default for LegacyConfig {
    fn default() -> Self {
        Self {
            provider: None,
            providers: vec![Provider::default()],
            api_key: None,
            api_key_wwo: String::default(),
            api_key_owm: String::default(),
            api_key_wa: String::default(),
            city: None,
            lat: None,
            lon: None,
            units: Units::default(),
            output_format: OutputFormat::default(),
            language: Language::default(),
            show_city_name: false,
            use_colors: false,
            wind_in_degrees: false,
            text_mode: TextMode::default(),
            compact_mode: None,
            live_mode: false,
            live_mode_interval: default_live_mode_interval(),
            align_right: false,
            use_geocoding_cache: false,
            verbose: 0,
            connect_timeout: default_connect_timeout(),
        }
    }
}
