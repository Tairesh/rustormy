use crate::cli::Cli;
use crate::errors::RustormyError;
use crate::models::{Language, Location, OutputFormat, Provider, TextMode, Units};
use crate::weather::LookUpCity;
#[cfg(not(test))]
use directories::ProjectDirs;
use serde_derive::{Deserialize, Serialize};
#[cfg(not(test))]
use std::fs;
#[cfg(not(test))]
use std::path::PathBuf;

#[cfg(not(test))]
const CONFIG_FILE_EXAMPLE: &str = r#"# Rustormy Configuration File
# This file is in TOML format. See https://toml.io/ for details
# For more details, see the documentation at https://github.com/Tairesh/rustormy/tree/main?tab=readme-ov-file#configuration
#
# Possible providers: `open_meteo`, `open_weather_map`, `world_weather_online`
# Note that `open_weather_map` and `world_weather_online` require an API key
# (`api_key_owm` for Open Weather Map, `api_key_wwo` for World Weather Online)
# You can specify multiple providers in the `providers` array to try them in order
# Example: `providers = ["world_weather_online", "open_weather_map", "open_meteo"]`

providers = ["open_meteo"]
api_key_owm = ""
api_key_wwo = ""

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

language = "en"

# Text mode can be `full`, `compact`, or `one_line`
# `compact` mode shows same info as `full` but without labels and trailing empty lines
# `one_line` mode shows only temperature and weather condition in a single line

text_mode = "full"

# Show city name can be enabled with `show_city_name = true` to include the city name in the output
# (only works if `city` is provided, not coordinates)

show_city_name = false

# Use colors can be enabled with `use_colors = true` to colorize the text output with ANSI colors

use_colors = false

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

verbose = 0
"#;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Weather data provider (`open_meteo`, `open_weather_map`, or `world_weather_online`)
    #[serde(default, skip_serializing)]
    provider: Option<Provider>, // Deprecated, kept for migration purposes. Use `providers` instead.

    /// List of providers to try in order (if the first fails, try the next, etc.)
    /// Example: `["open_meteo", "open_weather_map", "world_weather_online"]`
    #[serde(default)]
    providers: Vec<Provider>,

    /// Deprecated, kept for migration purposes. Use `api_key_owm` or `api_key_wwo` instead.
    #[serde(default, skip_serializing)]
    api_key: Option<String>, // Deprecated, kept for migration purposes. Use `api_key_owm` or `api_key_wwo` instead.

    /// API key for Open Weather Map
    #[serde(default)]
    api_key_owm: String,

    /// API key for World Weather Online
    #[serde(default)]
    api_key_wwo: String,

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
    compact_mode: Option<bool>, // Deprecated, kept for migration purposes. Use `text_mode = "compact"` instead.

    /// Live mode - continuously update weather data every `live_mode_interval` seconds (`true` or `false`)
    #[serde(default)]
    live_mode: bool,

    /// Live mode update interval in seconds (default: 300)
    #[serde(default = "default_live_mode_interval")]
    live_mode_interval: u64, // in seconds, default to 300 (5 minutes)

    /// Align output to the right (`true` or `false`)
    #[serde(default)]
    align_right: bool, // Actually aligns only labels to the right, not the whole output

    /// Use geocoding cache (`true` or `false`)
    /// (if enabled, previously looked up cities will be cached locally to avoid repeated API calls)
    #[serde(default)]
    use_geocoding_cache: bool,

    /// Verbosity level of output (0 = errors, 1 = warnings, 2 = info, 3 = debug)
    #[serde(default)]
    verbose: u8,
}

fn default_live_mode_interval() -> u64 {
    300
}

impl Default for Config {
    fn default() -> Self {
        Self {
            provider: None,
            providers: vec![Provider::default()],
            api_key: None,
            api_key_wwo: String::default(),
            api_key_owm: String::default(),
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
        fs::write(config_path, CONFIG_FILE_EXAMPLE)?;

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
            if self.api_key_owm.is_empty() {
                self.api_key_owm = api_key.to_string();
            }

            #[cfg(not(test))]
            {
                migrated = true;
            }
        }

        if let Some(provider) = self.provider
            && self.providers.is_empty()
        {
            self.providers = vec![provider];
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
            self.providers = vec![provider];
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
        if self.city.is_none() && self.show_city_name {
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

        // Check if API key is provided for Open Weather Map
        if self.providers.contains(&Provider::OpenWeatherMap)
            && self.api_key_owm().is_none_or(str::is_empty)
        {
            return Err(RustormyError::MissingApiKey);
        }

        // Check if API key is provided for World Weather Online
        if self.providers.contains(&Provider::WorldWeatherOnline)
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

    /// Pop the first provider from the list to try
    pub fn provider(&mut self) -> Option<Provider> {
        if self.providers.is_empty() {
            None
        } else {
            Some(self.providers.remove(0))
        }
    }

    pub fn api_key_wwo(&self) -> Option<&str> {
        if self.api_key_wwo.is_empty() {
            return self.api_key.as_deref();
        }

        Some(self.api_key_wwo.as_str())
    }

    pub fn api_key_owm(&self) -> Option<&str> {
        if self.api_key_owm.is_empty() {
            return self.api_key.as_deref();
        }

        Some(self.api_key_owm.as_str())
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

    pub fn get_location(&self, city_provider: &impl LookUpCity) -> Result<Location, RustormyError> {
        match (self.coordinates(), self.city()) {
            (Some((lat, lon)), _) => Ok(Location {
                name: self.location_name(),
                latitude: lat,
                longitude: lon,
            }),
            (None, Some(city)) if !city.is_empty() => city_provider.lookup_city(self),
            _ => Err(RustormyError::NoLocationProvided),
        }
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

    pub fn use_geocoding_cache(&self) -> bool {
        self.use_geocoding_cache
    }

    pub fn verbose(&self) -> u8 {
        self.verbose
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
            ..Default::default()
        };
        let migrated_config = config.migrate().unwrap();
        assert_eq!(migrated_config.api_key_owm, "test_key");
        assert_eq!(migrated_config.api_key_wwo, "");
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
            providers: vec![Provider::OpenMeteo, Provider::OpenWeatherMap],
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
            providers: vec![Provider::WorldWeatherOnline],
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
            api_key_owm: "test_key".to_string(),
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
            providers: vec![Provider::WorldWeatherOnline],
            api_key_wwo: "test_key".to_string(),
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
            providers: vec![Provider::OpenWeatherMap],
            api_key: Some("test_key".to_string()),
            api_key_owm: "".to_string(),
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
    fn test_validate_valid_config_with_all_providers() {
        let config = Config {
            city: Some("TestCity".to_string()),
            providers: vec![
                Provider::OpenMeteo,
                Provider::OpenWeatherMap,
                Provider::WorldWeatherOnline,
            ],
            api_key_owm: "test_key_owm".to_string(),
            api_key_wwo: "test_key_wwo".to_string(),
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
    fn test_migrate_provider_to_providers() {
        let config = Config {
            provider: Some(Provider::OpenWeatherMap),
            providers: vec![],
            ..Default::default()
        };
        let migrated_config = config.migrate().expect("Failed to migrate");
        assert_eq!(migrated_config.providers, vec![Provider::OpenWeatherMap]);
    }
}
