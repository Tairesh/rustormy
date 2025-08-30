use crate::cli::Cli;
use crate::errors::RustormyError;
use crate::models::{Language, OutputFormat, Provider, TextMode, Units};
#[cfg(not(test))]
use directories::ProjectDirs;
use serde_derive::{Deserialize, Serialize};
#[cfg(not(test))]
use std::fs;
#[cfg(not(test))]
use std::path::PathBuf;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Weather data provider (`open_meteo`, `open_weather_map`, or `world_weather_online`)
    #[serde(default)]
    provider: Provider,

    /// Deprecated, kept for migration purposes. Use `api_key_owm` or `api_key_wwo` instead.
    #[serde(default, skip_serializing)]
    api_key: Option<String>, // Deprecated, kept for migration purposes

    /// API key for Open Weather Map
    #[serde(default)]
    api_key_owm: Option<String>,

    /// API key for World Weather Online
    #[serde(default)]
    api_key_wwo: Option<String>,

    /// City name (required if lat/lon not provided)
    #[serde(default)]
    city: Option<String>,

    /// Latitude (required if city not provided)
    #[serde(default)]
    lat: Option<f64>,

    /// Longitude (required if city not provided)
    #[serde(default)]
    lon: Option<f64>,

    /// Units for temperature and wind speed (`metric` or `imperial`)
    #[serde(default)]
    units: Units,

    /// Output format (`text` or `json`)
    #[serde(default)]
    output_format: OutputFormat,

    /// Language code for weather output (e.g., `en`, `es`, `ru`, etc.)
    #[serde(default)]
    language: Language,

    /// Show city name in output (`true` or `false`)
    #[serde(default)]
    show_city_name: bool,

    /// Use colors in output (`true` or `false`)
    #[serde(default)]
    use_colors: bool,

    /// Use degrees for wind direction in output instead of arrows (`true` or `false`)
    #[serde(default, alias = "use_degrees_for_wind")]
    wind_in_degrees: bool,

    /// Text mode for text output (`full`, `compact`, or `one_line`)
    #[serde(default)]
    text_mode: TextMode,

    /// Deprecated, kept for migration purposes. Use `text_mode` instead.
    #[serde(default, skip_serializing)]
    compact_mode: Option<bool>, // Deprecated, kept for migration purposes

    /// Live mode - continuously update weather data every `live_mode_interval` seconds (`true` or `false`)
    #[serde(default)]
    live_mode: bool,

    /// Live mode update interval in seconds (default: 300)
    #[serde(default = "default_live_mode_interval")]
    live_mode_interval: u64, // in seconds, default to 300 (5 minutes)

    /// Align output to the right (`true` or `false`)
    #[serde(default)]
    align_right: bool, // Actually aligns only labels to the right, not the whole output
}

fn default_live_mode_interval() -> u64 {
    300
}

impl Default for Config {
    fn default() -> Self {
        Self {
            provider: Provider::default(),
            api_key: None,
            api_key_wwo: None,
            api_key_owm: None,
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
        }
    }
}

impl Config {
    #[cfg(not(test))]
    pub fn new(cli: &Cli) -> Result<Self, RustormyError> {
        // Try to load config from file first
        let mut config = Self::load_from_file()?.unwrap_or_default();

        // Merge CLI arguments on top of file config
        config.merge_cli(cli);
        config.validate()?;
        Ok(config)
    }

    #[cfg(test)]
    pub fn new(cli: &Cli) -> Result<Self, RustormyError> {
        let mut config = Self::default();
        config.merge_cli(cli);
        config.validate()?;
        Ok(config)
    }

    #[cfg(not(test))]
    fn load_from_file() -> Result<Option<Self>, RustormyError> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            let default_config = Self::create_default_config_file(&config_path)?;
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

    #[cfg(not(test))]
    fn create_default_config_file(config_path: &PathBuf) -> Result<Self, RustormyError> {
        let default_config = Self::default();

        // Create parent directories if they don't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Serialize and write default config
        let default_content = toml::to_string_pretty(&default_config)?;
        fs::write(config_path, default_content)?;

        Ok(default_config)
    }

    #[cfg(not(test))]
    fn read_and_parse_config_file(config_path: PathBuf) -> Result<Self, RustormyError> {
        let content = fs::read_to_string(config_path)?;
        let mut config: Self = toml::from_str(&content)?;
        config = config.migrate()?;
        Ok(config)
    }

    fn migrate(mut self) -> Result<Self, RustormyError> {
        #[cfg(not(test))]
        let mut migrated = false;

        if let Some(compact_mode) = self.compact_mode {
            if compact_mode {
                self.text_mode = TextMode::Compact;
            }

            #[cfg(not(test))]
            {
                migrated = true;
            }
        }

        if let Some(api_key) = self.api_key.as_deref() {
            if self.api_key_owm.is_none() {
                self.api_key_owm = Some(api_key.to_string());
            }

            #[cfg(not(test))]
            {
                migrated = true;
            }
        }

        if self.api_key_owm.is_none() {
            self.api_key_owm = Some(String::default());
            #[cfg(not(test))]
            {
                migrated = true;
            }
        }
        if self.api_key_wwo.is_none() {
            self.api_key_wwo = Some(String::default());
            #[cfg(not(test))]
            {
                migrated = true;
            }
        }

        #[cfg(not(test))]
        if migrated {
            // Save config back to file after migration
            let config_path = Self::get_config_path()?;
            let content = toml::to_string_pretty(&self)?;
            fs::write(config_path, content)?;
        }

        Ok(self)
    }

    fn merge_cli(&mut self, cli: &Cli) {
        if let Some(city) = &cli.city {
            self.city = Some(city.clone());
        }
        if let Some(lat) = cli.lat {
            self.lat = Some(lat);
        }
        if let Some(lon) = cli.lon {
            self.lon = Some(lon);
        }
        if let Some(provider) = cli.provider {
            self.provider = provider;
        }
        if let Some(units) = cli.units {
            self.units = units;
        }
        if let Some(output_format) = cli.output_format {
            self.output_format = output_format;
        }
        if let Some(language) = cli.language {
            self.language = language;
        }
        if let Some(live_mode_interval) = cli.live_mode_interval {
            self.live_mode_interval = live_mode_interval;
        }

        // Boolean flags are set directly if the flag is present
        if cli.show_city_name {
            self.show_city_name = true;
        }
        if cli.use_colors {
            self.use_colors = true;
        }
        if cli.use_degrees_for_wind {
            self.wind_in_degrees = true;
        }
        if cli.compact_mode {
            self.text_mode = TextMode::Compact;
        }
        if cli.one_line_mode {
            self.text_mode = TextMode::OneLine;
        }
        if let Some(text_mode) = cli.text_mode {
            self.text_mode = text_mode;
        }
        if cli.live_mode {
            self.live_mode = true;
        }
    }

    pub fn validate(&self) -> Result<(), RustormyError> {
        // Check if either city or coordinates are provided
        if self.city.is_none() && (self.lat.is_none() || self.lon.is_none()) {
            return Err(RustormyError::NoLocationProvided);
        }

        // Check if city name is to be shown but no city is provided
        if self.city.is_none() && self.show_city_name {
            return Err(RustormyError::InvalidConfiguration(
                "Cannot show city name when no city is provided",
            ));
        }

        // Check if API key is provided for OpenWeatherMap
        if matches!(self.provider, Provider::OpenWeatherMap)
            && self.api_key_owm().is_none_or(str::is_empty)
        {
            return Err(RustormyError::MissingApiKey);
        }

        // Check if API key is provided for World Weather Online
        if matches!(self.provider, Provider::WorldWeatherOnline)
            && self.api_key_wwo().is_none_or(str::is_empty)
        {
            return Err(RustormyError::MissingApiKey);
        }

        // Validate coordinates if provided
        if let Some((lat, lon)) = self.coordinates()
            && !((-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon))
        {
            return Err(RustormyError::InvalidCoordinates { lat, lon });
        }

        Ok(())
    }

    pub fn provider(&self) -> Provider {
        self.provider
    }

    pub fn api_key_wwo(&self) -> Option<&str> {
        self.api_key_wwo.as_deref().or(self.api_key.as_deref())
    }

    pub fn api_key_owm(&self) -> Option<&str> {
        self.api_key_owm.as_deref().or(self.api_key.as_deref())
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

    pub fn units(&self) -> Units {
        self.units
    }

    #[cfg(test)]
    pub fn set_units(&mut self, units: Units) {
        self.units = units;
    }

    pub fn language(&self) -> Language {
        self.language
    }

    #[cfg(test)]
    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }

    pub fn output_format(&self) -> OutputFormat {
        self.output_format
    }

    pub fn show_city_name(&self) -> bool {
        self.show_city_name
    }

    #[cfg(test)]
    pub fn set_show_city_name(&mut self, show: bool) {
        self.show_city_name = show;
    }

    pub fn use_colors(&self) -> bool {
        self.use_colors
    }

    #[cfg(test)]
    pub fn set_use_colors(&mut self, use_colors: bool) {
        self.use_colors = use_colors;
    }

    pub fn use_wind_in_degrees(&self) -> bool {
        self.wind_in_degrees
    }

    #[cfg(test)]
    pub fn set_wind_in_degrees(&mut self, use_degrees: bool) {
        self.wind_in_degrees = use_degrees;
    }

    pub fn text_mode(&self) -> TextMode {
        self.text_mode
    }

    #[cfg(test)]
    pub fn set_text_mode(&mut self, text_mode: TextMode) {
        self.text_mode = text_mode;
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

    pub fn align_right(&self) -> bool {
        self.align_right
    }

    #[cfg(test)]
    pub fn set_align_right(&mut self, align_right: bool) {
        self.align_right = align_right;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrate_compact_mode_true() {
        let config = Config {
            compact_mode: Some(true),
            ..Default::default()
        }
        .migrate()
        .unwrap();
        assert_eq!(config.text_mode, TextMode::Compact);
    }

    #[test]
    fn test_migrate_compact_mode_false() {
        let config = Config {
            compact_mode: Some(false),
            ..Default::default()
        }
        .migrate()
        .unwrap();
        assert_eq!(config.text_mode, TextMode::Full);
    }

    #[test]
    fn test_migrate_api_key() {
        let config = Config {
            api_key: Some("test_key".to_string()),
            api_key_owm: None,
            api_key_wwo: None,
            ..Default::default()
        };
        let migrated_config = config.migrate().unwrap();
        assert_eq!(migrated_config.api_key_owm, Some("test_key".to_string()));
        assert_eq!(migrated_config.api_key_wwo, Some(String::default()));
    }

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
            show_city_name: true,
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
            provider: Provider::OpenWeatherMap,
            api_key_owm: None,
            city: Some("TestCity".to_string()),
            ..Default::default()
        };
        let result = config.validate();
        assert!(
            matches!(result, Err(RustormyError::MissingApiKey)),
            "Expected MissingApiKey error got {:?}",
            result
        );
    }

    #[test]
    fn test_validate_missing_api_key_wwo() {
        let config = Config {
            provider: Provider::WorldWeatherOnline,
            api_key_wwo: None,
            city: Some("TestCity".to_string()),
            ..Default::default()
        };
        let result = config.validate();
        assert!(
            matches!(result, Err(RustormyError::MissingApiKey)),
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
            provider: Provider::OpenMeteo,
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
            provider: Provider::OpenWeatherMap,
            api_key_owm: Some("test_key".to_string()),
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
    fn test_validate_valid_config_wwo() {
        let config = Config {
            city: Some("TestCity".to_string()),
            provider: Provider::WorldWeatherOnline,
            api_key_wwo: Some("test_key".to_string()),
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
    fn test_validate_valid_config_with_old_api_key() {
        let config = Config {
            city: Some("TestCity".to_string()),
            provider: Provider::OpenWeatherMap,
            api_key: Some("test_key".to_string()),
            api_key_owm: None,
            ..Default::default()
        };
        let result = config.validate();
        assert!(
            result.is_ok(),
            "Expected valid config, got error {:?}",
            result
        );
    }
}
