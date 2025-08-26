use crate::cli::Cli;
use crate::errors::RustormyError;
use crate::models::{OutputFormat, Provider, Units};
#[cfg(not(test))]
use anyhow::Context;
#[cfg(not(test))]
use directories::ProjectDirs;
use serde_derive::{Deserialize, Serialize};
#[cfg(not(test))]
use std::fs;
#[cfg(not(test))]
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    provider: Provider,

    #[serde(default)]
    api_key: Option<String>,

    #[serde(default)]
    city: Option<String>,

    #[serde(default)]
    lat: Option<f64>,

    #[serde(default)]
    lon: Option<f64>,

    #[serde(default)]
    units: Units,

    #[serde(default)]
    output_format: OutputFormat,

    #[serde(default)]
    show_city_name: bool,

    #[serde(default)]
    use_colors: bool,

    #[serde(default)]
    live_mode: bool,

    #[serde(default = "default_live_mode_interval")]
    live_mode_interval: u64, // in seconds, default to 300 (5 minutes)
}

fn default_live_mode_interval() -> u64 {
    300
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
        let proj_dirs = ProjectDirs::from("", "", "rustormy").ok_or_else(|| {
            RustormyError::Other(anyhow::anyhow!("Could not determine config directory"))
        })?;

        let config_dir = proj_dirs.config_dir();
        let config_path = config_dir.join("config.toml");

        if !config_path.exists() {
            let default_config = Self::create_default_config_file(&config_path)?;
            return Ok(Some(default_config));
        }

        let config = Self::read_and_parse_config_file(config_path)?;
        Ok(Some(config))
    }

    #[cfg(not(test))]
    fn create_default_config_file(config_path: &PathBuf) -> Result<Self, RustormyError> {
        let default_config = Self::default();

        // Create parent directories if they don't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        // Serialize and write default config
        let default_content = toml::to_string_pretty(&default_config)
            .context("Failed to serialize default config")?;
        fs::write(config_path, default_content).context("Failed to write default config file")?;

        Ok(default_config)
    }

    #[cfg(not(test))]
    fn read_and_parse_config_file(config_path: PathBuf) -> Result<Self, RustormyError> {
        let content = fs::read_to_string(config_path).context("Failed to read config file")?;
        let config: Self = toml::from_str(&content).context("Failed to parse config file")?;
        Ok(config)
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
            return Err(RustormyError::Other(anyhow::anyhow!(
                "Cannot show city name when no city is provided"
            )));
        }

        // Check if API key is provided for OpenWeatherMap
        if matches!(self.provider, Provider::OpenWeatherMap) && self.api_key.is_none() {
            return Err(RustormyError::Other(anyhow::anyhow!(
                "API key is required for OpenWeatherMap provider"
            )));
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

    #[allow(dead_code)]
    pub fn api_key(&self) -> Option<&str> {
        self.api_key.as_deref()
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

    pub fn output_format(&self) -> OutputFormat {
        self.output_format
    }

    pub fn show_city_name(&self) -> bool {
        self.show_city_name
    }

    pub fn use_colors(&self) -> bool {
        self.use_colors
    }

    pub fn live_mode(&self) -> bool {
        self.live_mode
    }

    pub fn live_mode_interval(&self) -> u64 {
        self.live_mode_interval
    }
}
