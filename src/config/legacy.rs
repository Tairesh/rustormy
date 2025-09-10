use crate::config::{ApiKeys, FormatterConfig};
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

    /// API keys for different providers
    #[serde(default)]
    pub api_keys: Option<ApiKeys>,

    /// Deprecated, kept for migration purposes. Will fall back to `api_key_owm` if set.
    #[serde(default)]
    pub api_key: Option<String>,

    /// Deprecated API key for Open Weather Map
    #[serde(default)]
    pub api_key_owm: String,

    /// Deprecated API key for World Weather Online
    #[serde(default)]
    pub api_key_wwo: String,

    /// Deprecated API key for WeatherAPI.com
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

    /// Configuration for output formatting
    #[serde(default)]
    pub format: Option<FormatterConfig>,

    /// Units for temperature and wind speed (`metric` or `imperial`)
    /// Deprecated, kept for migration purposes. Will fall back to `format.units` if set.
    #[serde(default)]
    pub units: Units,

    /// Output format (`text` or `json`)
    /// Deprecated, kept for migration purposes. Will fall back to `format.output_format` if set.
    #[serde(default)]
    pub output_format: OutputFormat,

    /// Language code for weather output (e.g., `en`, `es`, `ru`, etc.)
    /// Deprecated, kept for migration purposes. Will fall back to `format.language` if set.
    #[serde(default)]
    pub language: Language,

    /// Show city name in output (`true` or `false`)
    /// Deprecated, kept for migration purposes. Will fall back to `format.show_city_name` if set.
    #[serde(default)]
    pub show_city_name: bool,

    /// Use colors in output (`true` or `false`)
    /// Deprecated, kept for migration purposes. Will fall back to `format.use_colors` if set.
    #[serde(default)]
    pub use_colors: bool,

    /// Use degrees for wind direction in output instead of arrows (`true` or `false`)
    /// Deprecated, kept for migration purposes. Will fall back to `format.wind_in_degrees` if set.
    #[serde(default, alias = "use_degrees_for_wind")]
    pub wind_in_degrees: bool,

    /// Text mode for text output (`full`, `compact`, or `one_line`)
    /// Deprecated, kept for migration purposes. Will fall back to `format.text_mode` if set.
    #[serde(default)]
    pub text_mode: TextMode,

    /// Deprecated, kept for migration purposes. Will fall back to `text_mode = "compact"`.
    /// If `true`, sets `text_mode` to `compact`. If `false` or `null` does not change `text_mode`.
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
    /// Deprecated, kept for migration purposes. Will fall back to `format.align_right` if set.
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
            api_keys: None,
            api_key: None,
            api_key_wwo: String::default(),
            api_key_owm: String::default(),
            api_key_wa: String::default(),
            city: None,
            lat: None,
            lon: None,
            format: None,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Cli, Config};
    use clap::Parser;

    #[test]
    fn test_migrate_compact_mode_true() {
        let config = Config::from(LegacyConfig {
            compact_mode: Some(true),
            ..Default::default()
        });
        assert_eq!(config.format().text_mode, TextMode::Compact);
    }

    #[test]
    fn test_migrate_compact_mode_false() {
        let config = Config::from(LegacyConfig {
            compact_mode: Some(false),
            ..Default::default()
        });
        assert_eq!(config.format().text_mode, TextMode::Full);
    }

    #[test]
    fn test_migrate_api_key() {
        let config = Config::from(LegacyConfig {
            api_key: Some("test_key".to_string()),
            ..Default::default()
        });
        assert_eq!(config.api_keys().open_weather_map, "test_key");
        assert_eq!(config.api_keys().world_weather_online, "");
    }

    #[test]
    fn test_validate_valid_config_with_old_api_key() {
        let config = Config::from(LegacyConfig {
            city: Some("TestCity".to_string()),
            providers: vec![Provider::OpenWeatherMap],
            api_key: Some("test_key".to_string()),
            api_key_owm: "".to_string(),
            ..Default::default()
        });
        let result = config.validate();
        assert!(
            result.is_ok(),
            "Expected valid config, got error {:?}",
            result
        );
    }

    #[test]
    fn test_migrate_provider_to_providers() {
        let config = Config::from(LegacyConfig {
            provider: Some(Provider::OpenWeatherMap),
            ..Default::default()
        });
        assert_eq!(
            config.providers(),
            &vec![Provider::OpenWeatherMap, Provider::default()]
        );
    }

    #[test]
    fn test_legacy_config_api_keys_migration() {
        let legacy_config = LegacyConfig {
            city: Some("Legacy City".to_string()),
            providers: vec![
                Provider::OpenWeatherMap,
                Provider::WorldWeatherOnline,
                Provider::WeatherApi,
            ],
            api_key_owm: "owm_key".to_string(),
            api_key_wwo: "wwo_key".to_string(),
            api_key_wa: "wa_key".to_string(),
            ..Default::default()
        };
        let config = Config::from(legacy_config);
        assert_eq!(config.api_keys().open_weather_map, "owm_key");
        assert_eq!(config.api_keys().world_weather_online, "wwo_key");
        assert_eq!(config.api_keys().weather_api, "wa_key");
        let valid = config.validate();
        assert!(
            valid.is_ok(),
            "Expected valid config after migration, got {:?}",
            valid
        );
    }

    #[test]
    fn test_legacy_config_format_options() {
        let legacy_config = LegacyConfig {
            output_format: OutputFormat::Json,
            text_mode: TextMode::OneLine,
            show_city_name: true,
            use_colors: true,
            wind_in_degrees: true,
            align_right: true,
            ..Default::default()
        };
        let config = Config::from(legacy_config);
        assert_eq!(config.format().output_format, OutputFormat::Json);
        assert_eq!(config.format().text_mode, TextMode::OneLine);
        assert!(config.format().show_city_name);
        assert!(config.format().use_colors);
        assert!(config.format().wind_in_degrees);
        assert!(config.format().align_right);
    }

    #[test]
    fn test_legacy_config_keeps_api_keys() {
        let legacy_config = LegacyConfig {
            api_keys: Some(ApiKeys {
                open_weather_map: "existing_owm_key".to_string(),
                world_weather_online: "existing_wwo_key".to_string(),
                weather_api: "existing_wa_key".to_string(),
                ..Default::default()
            }),
            api_key_owm: "owm_key".to_string(),
            api_key_wwo: "wwo_key".to_string(),
            api_key_wa: "wa_key".to_string(),
            ..Default::default()
        };
        let config = Config::from(legacy_config);
        assert_eq!(config.api_keys().open_weather_map, "existing_owm_key");
        assert_eq!(config.api_keys().world_weather_online, "existing_wwo_key");
        assert_eq!(config.api_keys().weather_api, "existing_wa_key");
    }

    #[test]
    fn test_parse_config_from_v010() {
        const EXAMPLE: &str = r#"
            provider = "open_meteo"
            units = "metric"
            output_format = "text"
            show_city_name = true
            use_colors = true
        "#;
        let legacy_config: LegacyConfig = toml::from_str(EXAMPLE).unwrap();
        let config = Config::from(legacy_config)
            .merge_cli_test(Cli::parse_from(&["rustormy", "-c", "TestCity"]));
        assert_eq!(config.city(), Some("TestCity"));
        assert_eq!(config.providers(), &vec![Provider::OpenMeteo]);
        assert_eq!(config.format().units, Units::Metric);
        assert_eq!(config.format().output_format, OutputFormat::Text);
        assert!(config.format().show_city_name);
        assert!(config.format().use_colors);
        let valid = config.validate();
        assert!(valid.is_ok(), "Expected valid config, got {:?}", valid);
    }

    #[test]
    fn test_parse_config_from_v015() {
        const EXAMPLE: &str = r#"
            provider = "open_weather_map"
            api_key = "test_key"
            units = "metric"
            output_format = "text"
            language = "Spanish"
            show_city_name = true
            use_colors = true
            use_degrees_for_wind = false
            compact_mode = true
            live_mode = false
            live_mode_interval = 0
        "#;
        let legacy_config: LegacyConfig = toml::from_str(EXAMPLE).unwrap();
        let config = Config::from(legacy_config)
            .merge_cli_test(Cli::parse_from(&["rustormy", "-c", "TestCity"]));
        assert_eq!(config.city(), Some("TestCity"));
        assert_eq!(config.providers(), &vec![Provider::OpenWeatherMap]);
        assert_eq!(config.api_keys().open_weather_map, "test_key");
        assert_eq!(config.format().units, Units::Metric);
        assert_eq!(config.format().output_format, OutputFormat::Text);
        assert_eq!(config.format().language, Language::Spanish);
        assert!(config.format().show_city_name);
        assert!(config.format().use_colors);
        assert!(!config.format().wind_in_degrees);
        assert_eq!(config.format().text_mode, TextMode::Compact);
        assert!(!config.live_mode());
        assert_eq!(config.live_mode_interval(), 300); // should default to 300 if 0 is provided
        let valid = config.validate();
        assert!(valid.is_ok(), "Expected valid config, got {:?}", valid);
    }

    #[test]
    fn test_parse_config_from_v020() {
        const EXAMPLE: &str = r#"
            provider = "open_weather_map"
            api_key = "test_key"
            units = "metric"
            output_format = "text"
            language = "Spanish"
            show_city_name = true
            use_colors = true
            use_degrees_for_wind = false
            text_mode = "full"
            live_mode = false
            live_mode_interval = 301
        "#;
        let legacy_config: LegacyConfig = toml::from_str(EXAMPLE).unwrap();
        let config = Config::from(legacy_config)
            .merge_cli_test(Cli::parse_from(&["rustormy", "-c", "TestCity"]));
        assert_eq!(config.city(), Some("TestCity"));
        assert_eq!(config.providers(), &vec![Provider::OpenWeatherMap]);
        assert_eq!(config.api_keys().open_weather_map, "test_key");
        assert_eq!(config.format().units, Units::Metric);
        assert_eq!(config.format().output_format, OutputFormat::Text);
        assert_eq!(config.format().language, Language::Spanish);
        assert!(config.format().show_city_name);
        assert!(config.format().use_colors);
        assert!(!config.format().wind_in_degrees);
        assert_eq!(config.format().text_mode, TextMode::Full);
        assert!(!config.live_mode());
        assert_eq!(config.live_mode_interval(), 301);
        let valid = config.validate();
        assert!(valid.is_ok(), "Expected valid config, got {:?}", valid);
    }

    #[test]
    fn test_parse_config_from_v030() {
        const EXAMPLE: &str = r#"
            # Rustormy Configuration File
            # This file is in TOML format. See https://toml.io/ for details
            # For more details, see the documentation at https://github.com/Tairesh/rustormy/tree/main?tab=readme-ov-file#configuration
            #
            # Possible providers: `open_meteo`, `open_weather_map`, `world_weather_online`
            # Note that `open_weather_map` and `world_weather_online` require an API key
            # (`api_key_owm` for Open Weather Map, `api_key_wwo` for World Weather Online)
            # You can specify multiple providers in the `providers` array to try them in order
            # Example: `providers = ["world_weather_online", "open_weather_map", "open_meteo"]`

            providers = ["world_weather_online", "open_weather_map", "open_meteo"]
            api_key_owm = "test_key"
            api_key_wwo = "test_key_wwo"

            # You can specify location either by `city` name or by `lat` and `lon` coordinates
            # If both are provided, coordinates will be used

            # city = "London"
            # lat = 51.5074
            # lon = -0.1278

            # Units can be `metric` (Celsius, m/s) or `imperial` (Fahrenheit, mph)

            units = "metric"

            # Output format can be `text` or `json`

            output_format = "text"

            # Language codes: `en` (English), `es` (Spanish), `ru` (Russian)
            # (more languages will be added in future)

            language = "es"

            # Text mode can be `full`, `compact`, or `one_line`
            # `compact` mode shows same info as `full` but without labels and trailing empty lines
            # `one_line` mode shows only temperature and weather condition in a single line

            text_mode = "full"

            # Show city name can be enabled with `show_city_name = true` to include the city name in the output
            # (only works if `city` is provided, not coordinates)

            show_city_name = true

            # Use colors can be enabled with `use_colors = true` to colorize the text output with ANSI colors

            use_colors = true

            # Wind in degrees can be enabled with `wind_in_degrees = true` to show wind direction in degrees

            wind_in_degrees = false

            # Live mode can be enabled with `live_mode = true` to update weather data every
            # `live_mode_interval` seconds (default is 300 seconds, i.e., 5 minutes)

            live_mode = false
            live_mode_interval = 302

            # Align right can be enabled with `align_right = true` to align labels to the right

            align_right = true

            # Use geocoding cache can be enabled with `use_geocoding_cache = true` to cache
            # previously looked up cities locally to avoid repeated API calls

            use_geocoding_cache = true

            # Verbosity level can be set with `verbose` (0 = errors, 1 = warnings, 2 = info, 3 = debug)

            verbose = 1
        "#;
        let legacy_config: LegacyConfig = toml::from_str(EXAMPLE).unwrap();
        let config = Config::from(legacy_config)
            .merge_cli_test(Cli::parse_from(&["rustormy", "-c", "TestCity"]));
        assert_eq!(config.city(), Some("TestCity"));
        assert_eq!(
            config.providers(),
            &vec![
                Provider::WorldWeatherOnline,
                Provider::OpenWeatherMap,
                Provider::OpenMeteo
            ]
        );
        assert_eq!(config.api_keys().open_weather_map, "test_key");
        assert_eq!(config.api_keys().world_weather_online, "test_key_wwo");
        assert_eq!(config.format().units, Units::Metric);
        assert_eq!(config.format().output_format, OutputFormat::Text);
        assert_eq!(config.format().language, Language::Spanish);
        assert!(config.format().show_city_name);
        assert!(config.format().use_colors);
        assert!(!config.format().wind_in_degrees);
        assert_eq!(config.format().text_mode, TextMode::Full);
        assert!(!config.live_mode());
        assert_eq!(config.live_mode_interval(), 302);
        assert!(config.format().align_right);
        assert!(config.use_geocoding_cache());
        assert_eq!(config.verbose(), 1);
        let valid = config.validate();
        assert!(valid.is_ok(), "Expected valid config, got {:?}", valid);
    }

    #[test]
    fn test_parse_config_from_v034() {
        const EXAMPLE: &str = r#"
            # Rustormy Configuration File
            # This file is in TOML format. See https://toml.io/ for details
            # For more details, see the documentation at https://github.com/Tairesh/rustormy/tree/main?tab=readme-ov-file#configuration

            # Possible providers: `open_meteo`, `open_weather_map`, `world_weather_online`, `weather_api`
            # Note that all providers except `open_meteo` require an API key
            # You can specify multiple providers in the `providers` array to try them in order
            # Example: `providers = ["world_weather_online", "open_weather_map", "open_meteo"]`

            providers = ["weather_api", "world_weather_online", "open_weather_map", "open_meteo"]

            # API key for Open Weather Map (required if using `open_weather_map` provider)
            # Get your free API key from https://home.openweathermap.org/users/sign_up

            api_key_owm = "test_key_owm"

            # API key for World Weather Online (required if using `world_weather_online` provider)
            # Get your free API key from https://www.worldweatheronline.com/developer/

            api_key_wwo = "test_key_wwo"

            # API key for WeatherAPI.com (required if using `weather_api` provider)
            # Get your free API key from https://www.weatherapi.com/signup.aspx

            api_key_wa = "test_key_wa"

            # You can specify location either by `city` name or by `lat` and `lon` coordinates
            # If both are provided, coordinates will be used

            # city = "London"
            # lat = 51.5074
            # lon = -0.1278

            # Units can be `metric` (Celsius, m/s) or `imperial` (Fahrenheit, mph)

            units = "metric"

            # Output format can be `text` or `json`

            output_format = "text"

            # Language codes: `en` (English), `es` (Spanish), `ru` (Russian)
            # (more languages will be added in future)

            language = "es"

            # Text mode can be `full`, `compact`, or `one_line`
            # `compact` mode shows same info as `full` but without labels and trailing empty lines
            # `one_line` mode shows only temperature and weather condition in a single line

            text_mode = "full"

            # Show city name can be enabled with `show_city_name = true` to include the city name in the output
            # (only works if `city` is provided, not coordinates)

            show_city_name = true

            # Use colors can be enabled with `use_colors = true` to colorize the text output with ANSI colors

            use_colors = true

            # Wind in degrees can be enabled with `wind_in_degrees = true` to show wind direction in degrees

            wind_in_degrees = false

            # Live mode can be enabled with `live_mode = true` to update weather data every
            # `live_mode_interval` seconds (default is 300 seconds, i.e., 5 minutes)

            live_mode = false
            live_mode_interval = 300

            # Align right can be enabled with `align_right = true` to align labels to the right

            align_right = false

            # Use geocoding cache can be enabled with `use_geocoding_cache = true` to cache
            # previously looked up cities locally to avoid repeated API calls

            use_geocoding_cache = false

            # Verbosity level can be set with `verbose` (0 = errors, 1 = warnings, 2 = info, 3 = debug)

            verbose = 1

            # API HTTP client timeout in seconds (default is 10 seconds)

            connect_timeout = 11
        "#;
        let legacy_config: LegacyConfig = toml::from_str(EXAMPLE).unwrap();
        let config = Config::from(legacy_config)
            .merge_cli_test(Cli::parse_from(&["rustormy", "-c", "TestCity"]));
        assert_eq!(config.city(), Some("TestCity"));
        assert_eq!(
            config.providers(),
            &vec![
                Provider::WeatherApi,
                Provider::WorldWeatherOnline,
                Provider::OpenWeatherMap,
                Provider::OpenMeteo
            ]
        );
        assert_eq!(config.api_keys().open_weather_map, "test_key_owm");
        assert_eq!(config.api_keys().world_weather_online, "test_key_wwo");
        assert_eq!(config.api_keys().weather_api, "test_key_wa");
        assert_eq!(config.format().units, Units::Metric);
        assert_eq!(config.format().output_format, OutputFormat::Text);
        assert_eq!(config.format().language, Language::Spanish);
        assert!(config.format().show_city_name);
        assert!(config.format().use_colors);
        assert!(!config.format().wind_in_degrees);
        assert_eq!(config.format().text_mode, TextMode::Full);
        assert!(!config.live_mode());
        assert_eq!(config.live_mode_interval(), 300);
        assert!(!config.format().align_right);
        assert!(!config.use_geocoding_cache());
        assert_eq!(config.verbose(), 1);
        assert_eq!(config.connect_timeout(), 11);
        let valid = config.validate();
        assert!(valid.is_ok(), "Expected valid config, got {:?}", valid);
    }
}
