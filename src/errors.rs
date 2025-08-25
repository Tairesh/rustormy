use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustormyError {
    #[error("City not found: {0}")]
    CityNotFound(String),
    #[error("API request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    #[error("Invalid coordinates: latitude {lat}, longitude {lon}")]
    InvalidCoordinates { lat: f64, lon: f64 },
    #[error(
        "Either city or both latitude and longitude must be provided. Try rustormy --help for more information."
    )]
    NoLocationProvided,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
