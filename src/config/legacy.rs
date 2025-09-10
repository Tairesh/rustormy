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
    use crate::config::Config;

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
}
