use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustormyError {
    #[error("Failed to parse command line arguments: {0}")]
    CliError(#[from] clap::Error),
    #[cfg(not(test))]
    #[error("Failed to find config file: {0}")]
    ConfigNotFound(&'static str),
    #[error("Failed to read config file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse config file: {0}")]
    ConfigParseError(#[from] toml::de::Error),
    #[error("Failed to save config file: {0}")]
    ConfigSaveError(#[from] toml::ser::Error),
    #[error("Invalid coordinates: latitude {lat}, longitude {lon}")]
    InvalidCoordinates { lat: f64, lon: f64 },
    #[error("No location provided. Please specify a city or coordinates.")]
    NoLocationProvided,
    #[error("Missing API key for selected weather provider")]
    MissingApiKey,
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(&'static str),
    #[error("HTTP request failed: {0}")]
    HttpRequestFailed(#[from] reqwest::Error),
    #[error("City not found: {0}")]
    CityNotFound(String),
    #[error("API returned an error: {0}")]
    ApiReturnedError(String),
    #[error("Failed to encode JSON output: {0}")]
    JsonSerializeError(#[from] serde_json::Error),
}
