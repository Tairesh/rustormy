use crate::config::Cli;
use crate::config::legacy::LegacyConfig;
use crate::errors::RustormyError;
use crate::models::{ColorTheme, Language, OutputFormat, Provider, TextMode, Units};
#[cfg(not(test))]
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE_HEADER: &str = "# Rustormy Configuration File
# This file is in TOML format. See https://toml.io/ for details
#
# Check the documentation for configuration options: https://github.com/Tairesh/rustormy/tree/main?tab=readme-ov-file#configuration
";

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ApiKeys {
    #[serde(default)]
    pub open_weather_map: String,
    #[serde(default)]
    pub world_weather_online: String,
    #[serde(default)]
    pub weather_api: String,
    #[serde(default)]
    pub weather_bit: String,
    #[serde(default)]
    pub tomorrow_io: String,
    #[serde(default)]
    pub open_uv: String,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct FormatterConfig {
    #[serde(default)]
    pub output_format: OutputFormat,
    #[serde(default)]
    pub text_mode: TextMode,
    #[serde(default)]
    pub use_colors: bool,
    #[serde(default)]
    pub show_city_name: bool,
    #[serde(default)]
    pub align_right: bool,
    #[serde(default)]
    pub wind_in_degrees: bool,
    #[serde(default)]
    pub units: Units,
    #[serde(default)]
    pub language: Language,
    #[serde(default)]
    pub color_theme: ColorTheme,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// List of providers to try in order (if the first fails, try the next, etc.)
    /// Example: `["open_meteo", "open_weather_map", "world_weather_online", "weather_api"]`
    #[serde(default)]
    providers: Vec<Provider>,

    /// API keys for various providers
    api_keys: ApiKeys,

    /// City name (required if lat/lon not provided)
    #[serde(default)]
    city: Option<String>,

    /// Latitude (required if city not provided)
    #[serde(default)]
    lat: Option<f64>,

    /// Longitude (required if city not provided)
    #[serde(default)]
    lon: Option<f64>,

    /// Configuration for output formatting
    #[serde(default)]
    format: FormatterConfig,

    /// Live mode - continuously update weather data every `live_mode_interval` seconds (`true` or `false`)
    #[serde(default)]
    live_mode: bool,

    /// Live mode update interval in seconds (default: 300)
    #[serde(default = "default_live_mode_interval")]
    live_mode_interval: u64, // in seconds, default to 300 (5 minutes)

    /// Use geocoding cache (`true` or `false`)
    /// (if enabled, previously looked up cities will be cached locally to avoid repeated API calls)
    #[serde(default)]
    use_geocoding_cache: bool,

    /// Verbosity level of output (0 = errors, 1 = warnings, 2 = info, 3 = debug)
    #[serde(default)]
    verbose: u8,

    /// API HTTP client timeout in seconds
    #[serde(default = "default_connect_timeout")]
    connect_timeout: u64, // in seconds, default to 10
}

fn default_live_mode_interval() -> u64 {
    300
}
fn default_connect_timeout() -> u64 {
    10
}

impl Default for Config {
    fn default() -> Self {
        Self {
            providers: vec![Provider::default()],
            api_keys: ApiKeys::default(),
            city: None,
            lat: None,
            lon: None,
            format: FormatterConfig::default(),
            live_mode: false,
            live_mode_interval: default_live_mode_interval(),
            use_geocoding_cache: false,
            verbose: 0,
            connect_timeout: default_connect_timeout(),
        }
    }
}

impl Config {
    #[cfg(not(test))]
    pub fn new(cli: Cli) -> Result<Self, RustormyError> {
        // Try to load config from file first
        let file_path = Self::get_config_path()?;
        let mut config = Self::load_from_file(&file_path)?.unwrap_or_default();

        // Merge CLI arguments on top of file config
        config.merge_cli(cli);
        config.validate()?;
        Ok(config)
    }

    #[cfg(test)]
    pub fn new(cli: Cli) -> Result<Self, RustormyError> {
        let mut config = Self::default();
        config.merge_cli(cli);
        config.validate()?;
        Ok(config)
    }

    fn load_from_file(config_path: &PathBuf) -> Result<Option<Self>, RustormyError> {
        if !config_path.exists() {
            let default_config = Self::create_default_config_file(config_path)?;
            return Ok(Some(default_config));
        }

        let config = Self::read_and_parse_config_file(config_path)?;
        Ok(Some(config))
    }

    #[cfg(not(test))]
    fn get_config_path() -> Result<PathBuf, RustormyError> {
        let proj_dirs = ProjectDirs::from("", "", "rustormy")
            .ok_or_else(|| RustormyError::ConfigNotFound("Could not determine config directory"))?;

        let config_dir = proj_dirs.config_dir();
        let config_path = config_dir.join("config.toml");

        Ok(config_path)
    }

    fn create_default_config_file(config_path: &PathBuf) -> Result<Self, RustormyError> {
        let default_config = Self::default();
        default_config.write_to_file(config_path)?;
        Ok(default_config)
    }

    fn write_to_file(&self, config_path: &PathBuf) -> Result<(), RustormyError> {
        // Create parent directories if they don't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Serialize and write config
        let content = format!("{CONFIG_FILE_HEADER}\n{}", toml::to_string_pretty(self)?);
        fs::write(config_path, content)?;

        Ok(())
    }

    fn read_and_parse_config_file(config_path: &PathBuf) -> Result<Self, RustormyError> {
        let content = fs::read_to_string(config_path)?;
        let config: Self = toml::from_str(&content).or_else(|_| {
            let legacy_config: LegacyConfig = toml::from_str(&content)?;
            let config = Config::from(legacy_config);
            config.write_to_file(config_path)?;
            Ok::<Config, RustormyError>(config)
        })?;
        Ok(config)
    }

    fn merge_cli(&mut self, cli: Cli) {
        if let Some(city) = cli.city {
            self.city = Some(city);
        }
        if let Some(lat) = cli.lat {
            self.lat = Some(lat);
        }
        if let Some(lon) = cli.lon {
            self.lon = Some(lon);
        }
        if let Some(provider) = cli.provider {
            self.providers = vec![provider];
        }
        if let Some(units) = cli.units {
            self.format.units = units;
        }
        if let Some(output_format) = cli.output_format {
            self.format.output_format = output_format;
        }
        if let Some(language) = cli.language {
            self.format.language = language;
        }
        if let Some(live_mode_interval) = cli.live_mode_interval {
            self.live_mode_interval = live_mode_interval;
        }

        // Boolean flags are set directly if the flag is present
        if cli.show_city_name {
            self.format.show_city_name = true;
        }
        if cli.use_colors {
            self.format.use_colors = true;
        }
        if cli.use_degrees_for_wind {
            self.format.wind_in_degrees = true;
        }
        if cli.compact_mode {
            self.format.text_mode = TextMode::Compact;
        }
        if cli.one_line_mode {
            self.format.text_mode = TextMode::OneLine;
        }
        if let Some(text_mode) = cli.text_mode {
            self.format.text_mode = text_mode;
        }
        if cli.align_right {
            self.format.align_right = true;
        }
        if cli.live_mode {
            self.live_mode = true;
        }
        if cli.no_cache {
            self.use_geocoding_cache = false;
        }
        if cli.verbose > 0 {
            self.verbose = cli.verbose;
        }
    }

    pub fn validate(&self) -> Result<(), RustormyError> {
        // Check if either city or coordinates are provided
        if self.city.is_none() && (self.lat.is_none() || self.lon.is_none()) {
            return Err(RustormyError::NoLocationProvided);
        }

        // Check if city name is to be shown but no city is provided
        if self.city.is_none() && self.format.show_city_name {
            return Err(RustormyError::InvalidConfiguration(
                "Cannot show city name when no city is provided",
            ));
        }

        // Check if at least one provider is specified
        if self.providers.is_empty() {
            return Err(RustormyError::InvalidConfiguration(
                "At least one provider must be specified",
            ));
        }

        // Check if API key is provided for OpenWeatherMap
        if self.providers.contains(&Provider::OpenWeatherMap)
            && self.api_keys().open_weather_map.is_empty()
        {
            return Err(RustormyError::MissingApiKey(Provider::OpenWeatherMap));
        }

        // Check if API key is provided for World Weather Online
        if self.providers.contains(&Provider::WorldWeatherOnline)
            && self.api_keys().world_weather_online.is_empty()
        {
            return Err(RustormyError::MissingApiKey(Provider::WorldWeatherOnline));
        }

        // Check if API key is provided for WeatherAPI.com
        if self.providers.contains(&Provider::WeatherApi) && self.api_keys().weather_api.is_empty()
        {
            return Err(RustormyError::MissingApiKey(Provider::WeatherApi));
        }

        // Check if API key is provided for WeatherBit
        if self.providers.contains(&Provider::WeatherBit) && self.api_keys().weather_bit.is_empty()
        {
            return Err(RustormyError::MissingApiKey(Provider::WeatherBit));
        }

        // Check if API key is provided for Tomorrow.io
        if self.providers.contains(&Provider::TomorrowIo) && self.api_keys().tomorrow_io.is_empty()
        {
            return Err(RustormyError::MissingApiKey(Provider::TomorrowIo));
        }

        // Validate coordinates if provided
        if let Some((lat, lon)) = self.coordinates()
            && !((-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon))
        {
            return Err(RustormyError::InvalidCoordinates { lat, lon });
        }

        Ok(())
    }

    #[cfg(test)]
    pub fn providers(&self) -> &Vec<Provider> {
        &self.providers
    }

    /// Pop the first provider from the list to try
    pub fn provider(&mut self) -> Option<Provider> {
        if self.providers.is_empty() {
            None
        } else {
            Some(self.providers.remove(0))
        }
    }

    pub fn api_keys(&self) -> &ApiKeys {
        &self.api_keys
    }

    pub fn city(&self) -> Option<&str> {
        self.city.as_deref()
    }

    pub fn coordinates(&self) -> Option<(f64, f64)> {
        match (self.lat, self.lon) {
            (Some(lat), Some(lon)) => Some((lat, lon)),
            _ => None,
        }
    }

    pub fn location_name(&self) -> String {
        self.city.as_ref().map_or_else(
            || format!("{}, {}", self.lat.unwrap(), self.lon.unwrap()),
            String::from,
        )
    }

    pub fn live_mode(&self) -> bool {
        self.live_mode
    }

    pub fn live_mode_interval(&self) -> u64 {
        if self.live_mode_interval == 0 {
            default_live_mode_interval()
        } else {
            self.live_mode_interval
        }
    }
    pub fn format(&self) -> &FormatterConfig {
        &self.format
    }

    #[cfg(test)]
    pub fn set_format(&mut self, format: FormatterConfig) {
        self.format = format;
    }

    pub fn language(&self) -> Language {
        self.format.language
    }

    pub fn units(&self) -> Units {
        self.format.units
    }

    pub fn use_geocoding_cache(&self) -> bool {
        self.use_geocoding_cache
    }

    pub fn verbose(&self) -> u8 {
        self.verbose
    }

    pub fn connect_timeout(&self) -> u64 {
        if self.connect_timeout == 0 {
            default_connect_timeout()
        } else {
            self.connect_timeout
        }
    }
}

impl From<LegacyConfig> for Config {
    fn from(value: LegacyConfig) -> Self {
        let mut providers = value.providers;
        if let Some(provider) = value.provider
            && !providers.contains(&provider)
        {
            providers.insert(0, provider);
        }
        let text_mode = if let Some(compact) = value.compact_mode {
            if compact {
                TextMode::Compact
            } else {
                value.text_mode
            }
        } else {
            value.text_mode
        };
        let api_keys = if let Some(api_keys) = value.api_keys {
            api_keys
        } else {
            ApiKeys {
                open_weather_map: if value.api_key_owm.is_empty()
                    && let Some(api_key) = value.api_key
                {
                    api_key
                } else {
                    value.api_key_owm
                },
                world_weather_online: value.api_key_wwo,
                weather_api: value.api_key_wa,
                weather_bit: String::default(),
                tomorrow_io: String::default(),
                open_uv: String::default(),
            }
        };
        let format = if let Some(format) = value.format {
            format
        } else {
            FormatterConfig {
                text_mode,
                units: value.units,
                output_format: value.output_format,
                language: value.language,
                show_city_name: value.show_city_name,
                use_colors: value.use_colors,
                wind_in_degrees: value.wind_in_degrees,
                align_right: value.align_right,
                color_theme: ColorTheme::default(),
            }
        };

        Self {
            providers,
            api_keys,
            city: value.city,
            lat: value.lat,
            lon: value.lon,
            format,
            live_mode: value.live_mode,
            live_mode_interval: value.live_mode_interval,
            use_geocoding_cache: value.use_geocoding_cache,
            verbose: value.verbose,
            connect_timeout: value.connect_timeout,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_no_location() {
        let config = Config::default();
        let result = config.validate();
        assert!(
            matches!(result, Err(RustormyError::NoLocationProvided)),
            "Expected NoLocationProvided error got {:?}",
            result
        );
    }

    #[test]
    fn test_validate_show_city_name_without_city() {
        let config = Config {
            lat: Some(51.5074),
            lon: Some(-0.1278),
            format: FormatterConfig {
                show_city_name: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let result = config.validate();
        assert!(
            matches!(result, Err(RustormyError::InvalidConfiguration(_))),
            "Expected InvalidConfiguration error got {:?}",
            result
        );
    }

    #[test]
    fn test_validate_missing_api_key_owm() {
        let config = Config {
            providers: vec![Provider::OpenMeteo, Provider::OpenWeatherMap],
            city: Some("TestCity".to_string()),
            ..Default::default()
        };
        let result = config.validate();
        assert!(
            matches!(
                result,
                Err(RustormyError::MissingApiKey(Provider::OpenWeatherMap))
            ),
            "Expected MissingApiKey error got {:?}",
            result
        );
    }

    #[test]
    fn test_validate_missing_api_key_wwo() {
        let config = Config {
            providers: vec![Provider::WorldWeatherOnline],
            city: Some("TestCity".to_string()),
            ..Default::default()
        };
        let result = config.validate();
        assert!(
            matches!(
                result,
                Err(RustormyError::MissingApiKey(Provider::WorldWeatherOnline))
            ),
            "Expected MissingApiKey error got {:?}",
            result
        );
    }

    #[test]
    fn test_validate_missing_api_key_wa() {
        let config = Config {
            providers: vec![Provider::WeatherApi],
            city: Some("TestCity".to_string()),
            ..Default::default()
        };
        let result = config.validate();
        assert!(
            matches!(
                result,
                Err(RustormyError::MissingApiKey(Provider::WeatherApi))
            ),
            "Expected MissingApiKey error got {:?}",
            result
        );
    }

    #[test]
    fn test_validate_missing_api_key_wb() {
        let config = Config {
            providers: vec![Provider::WeatherBit],
            city: Some("TestCity".to_string()),
            ..Default::default()
        };
        let result = config.validate();
        assert!(
            matches!(
                result,
                Err(RustormyError::MissingApiKey(Provider::WeatherBit))
            ),
            "Expected MissingApiKey error got {:?}",
            result
        );
    }

    #[test]
    fn test_validate_invalid_coordinates_lat() {
        let config = Config {
            lat: Some(91.0),
            lon: Some(0.0),
            ..Default::default()
        };
        let result = config.validate();
        assert!(
            matches!(result, Err(RustormyError::InvalidCoordinates { .. })),
            "Expected InvalidCoordinates error got {:?}",
            result
        );
    }

    #[test]
    fn test_validate_invalid_coordinates_lon() {
        let config = Config {
            lat: Some(0.0),
            lon: Some(181.0),
            ..Default::default()
        };
        let result = config.validate();
        assert!(
            matches!(result, Err(RustormyError::InvalidCoordinates { .. })),
            "Expected InvalidCoordinates error got {:?}",
            result
        );
    }

    #[test]
    fn test_validate_valid_config_om() {
        let config = Config {
            city: Some("TestCity".to_string()),
            providers: vec![Provider::OpenMeteo],
            ..Default::default()
        };
        let result = config.validate();
        assert!(
            result.is_ok(),
            "Expected valid config, got error {:?}",
            result
        );
    }

    #[test]
    fn test_validate_valid_config_owm() {
        let config = Config {
            city: Some("TestCity".to_string()),
            providers: vec![Provider::OpenWeatherMap],
            api_keys: ApiKeys {
                open_weather_map: "test_key".to_string(),
                ..ApiKeys::default()
            },
            ..Config::default()
        };
        let result = config.validate();
        assert!(
            result.is_ok(),
            "Expected valid config, got error {:?}",
            result
        );
    }

    #[test]
    fn test_validate_valid_config_wwo() {
        let config = Config {
            city: Some("TestCity".to_string()),
            providers: vec![Provider::WorldWeatherOnline],
            api_keys: ApiKeys {
                world_weather_online: "test_key".to_string(),
                ..ApiKeys::default()
            },
            ..Config::default()
        };
        let result = config.validate();
        assert!(
            result.is_ok(),
            "Expected valid config, got error {:?}",
            result
        );
    }

    #[test]
    fn test_validate_valid_config_wa() {
        let config = Config {
            city: Some("TestCity".to_string()),
            providers: vec![Provider::WeatherApi],
            api_keys: ApiKeys {
                weather_api: "test_key".to_string(),
                ..ApiKeys::default()
            },
            ..Config::default()
        };
        let result = config.validate();
        assert!(
            result.is_ok(),
            "Expected valid config, got error {:?}",
            result
        );
    }

    #[test]
    fn test_validate_valid_config_wb() {
        let config = Config {
            city: Some("TestCity".to_string()),
            providers: vec![Provider::WeatherBit],
            api_keys: ApiKeys {
                weather_bit: "test_key".to_string(),
                ..ApiKeys::default()
            },
            ..Config::default()
        };
        let result = config.validate();
        assert!(
            result.is_ok(),
            "Expected valid config, got error {:?}",
            result
        );
    }

    #[test]
    fn test_validate_valid_config_with_all_providers() {
        let config = Config {
            city: Some("TestCity".to_string()),
            providers: vec![
                Provider::OpenMeteo,
                Provider::OpenWeatherMap,
                Provider::WorldWeatherOnline,
                Provider::WeatherApi,
            ],
            api_keys: ApiKeys {
                open_weather_map: "owm_key".to_string(),
                world_weather_online: "wwo_key".to_string(),
                weather_api: "wa_key".to_string(),
                ..ApiKeys::default()
            },
            ..Config::default()
        };
        let result = config.validate();
        assert!(
            result.is_ok(),
            "Expected valid config, got error {:?}",
            result
        );
    }

    #[test]
    fn test_config_file_header() {
        let default_config = Config::default();
        let config_file_path = std::env::temp_dir().join("test_config_file_header.toml");
        default_config.write_to_file(&config_file_path).unwrap();
        let content = fs::read_to_string(&config_file_path).unwrap();
        assert!(
            content.starts_with(CONFIG_FILE_HEADER),
            "Expected config file to start with header",
        );

        fs::remove_file(config_file_path).unwrap();
    }

    // This test should be in legacy.rs, but uses too much of Config's private API to be there
    #[test]
    fn test_legacy_config_file_migration() {
        let mut legacy_config = LegacyConfig::default();
        legacy_config.api_key = Some("legacy_key".to_string());
        legacy_config.provider = Some(Provider::OpenWeatherMap);
        let config_file_path = std::env::temp_dir().join("test_legacy_config_file_migration.toml");
        fs::write(
            &config_file_path,
            toml::to_string_pretty(&legacy_config).unwrap(),
        )
        .unwrap();

        // Check that loading the config migrates it correctly
        let config = Config::load_from_file(&config_file_path).unwrap().unwrap();
        assert_eq!(config.api_keys.open_weather_map, "legacy_key");
        assert_eq!(
            config.providers,
            vec![Provider::OpenWeatherMap, Provider::default()]
        );

        // Check that the config file has been updated with the new format and header
        let content = fs::read_to_string(&config_file_path).unwrap();
        assert!(content.starts_with(CONFIG_FILE_HEADER));
        let parsed_config: Config = toml::from_str(&content).unwrap();
        assert_eq!(parsed_config.api_keys.open_weather_map, "legacy_key");
        assert_eq!(
            parsed_config.providers,
            vec![Provider::OpenWeatherMap, Provider::default()]
        );

        fs::remove_file(config_file_path).unwrap();
    }

    #[test]
    fn test_load_incorrect_config_file() {
        let config_file_path = std::env::temp_dir().join("test_load_incorrect_config_file.toml");
        fs::write(&config_file_path, "this is not valid toml").unwrap();

        let result = Config::load_from_file(&config_file_path);
        assert!(matches!(result, Err(RustormyError::ConfigParseError(_))));
    }

    #[test]
    fn test_load_nonexistent_config_file() {
        let config_file_path = std::env::temp_dir().join("nonexistent_config_file.toml");
        if config_file_path.exists() {
            fs::remove_file(&config_file_path).unwrap();
        }

        let result = Config::load_from_file(&config_file_path).unwrap();
        assert!(result.is_some(), "Expected default config to be created");
        assert!(
            config_file_path.exists(),
            "Expected config file to be created"
        );

        fs::remove_file(config_file_path).unwrap();
    }

    #[test]
    fn test_load_from_almost_empty_legacy_config_file() {
        let config_file_path =
            std::env::temp_dir().join("test_load_from_almost_empty_legacy_config_file.toml");
        fs::write(
            &config_file_path,
            r#"
                city = "Test City"
                provider = "owm"
                api_key = "legacy_key"
            "#,
        )
        .unwrap();

        let result = Config::load_from_file(&config_file_path).unwrap();
        assert!(result.is_some(), "Expected default config to be created");
        let config = result.unwrap();
        assert_eq!(config.city(), Some("Test City"));
        assert_eq!(config.api_keys().open_weather_map, "legacy_key");
        assert_eq!(config.providers, vec![Provider::OpenWeatherMap]);

        let content = fs::read_to_string(&config_file_path).unwrap();
        assert!(content.starts_with(CONFIG_FILE_HEADER));

        fs::remove_file(config_file_path).unwrap();
    }

    #[test]
    fn test_merge_cli_overrides() {
        let mut config = Config::default();
        config.city = Some("File City".to_string());
        config.lat = Some(10.0);
        config.lon = Some(20.0);
        config.providers = vec![Provider::OpenMeteo];
        config.live_mode = false;
        config.live_mode_interval = 300;
        config.use_geocoding_cache = true;
        config.verbose = 1;
        config.format = FormatterConfig {
            output_format: OutputFormat::Text,
            text_mode: TextMode::Full,
            use_colors: false,
            show_city_name: false,
            align_right: false,
            wind_in_degrees: false,
            units: Units::Metric,
            language: Language::English,
            color_theme: ColorTheme::default(),
        };

        let cli = Cli {
            city: Some("CLI City".to_string()),
            lat: Some(30.0),
            lon: Some(40.0),
            provider: Some(Provider::OpenWeatherMap),
            units: Some(Units::Imperial),
            output_format: Some(OutputFormat::Json),
            language: Some(Language::Spanish),
            show_city_name: true,
            use_colors: true,
            use_degrees_for_wind: true,
            compact_mode: true,
            one_line_mode: false,
            text_mode: None,
            align_right: true,
            live_mode: true,
            live_mode_interval: Some(600),
            no_cache: true,
            verbose: 3,
            clear_cache: false,
        };
        config.merge_cli(cli);
        assert_eq!(config.city(), Some("CLI City"));
        assert_eq!(config.coordinates(), Some((30.0, 40.0)));
        assert_eq!(config.providers, vec![Provider::OpenWeatherMap]);
        assert_eq!(config.format.units, Units::Imperial);
        assert_eq!(config.format.output_format, OutputFormat::Json);
        assert_eq!(config.format.language, Language::Spanish);
        assert!(config.format.show_city_name);
        assert!(config.format.use_colors);
        assert!(config.format.wind_in_degrees);
        assert_eq!(config.format.text_mode, TextMode::Compact);
        assert!(config.live_mode);
        assert_eq!(config.live_mode_interval, 600);
        assert!(config.format.align_right);
        assert!(!config.use_geocoding_cache);
        assert_eq!(config.verbose, 3);
    }
}
