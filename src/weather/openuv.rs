use crate::config::Config;
use crate::errors::RustormyError;
use crate::models::Location;
use reqwest::blocking::Client;

const OPEN_UV_API_URL: &str = "https://api.openuv.io/api/v1/uv";

#[derive(Debug, serde::Serialize)]
struct UvRequestParams {
    lat: f64,
    lng: f64,
}

impl UvRequestParams {
    pub fn new(location: &Location) -> Self {
        Self {
            lat: location.latitude,
            lng: location.longitude,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum UvResponse {
    Ok { result: UvResult },
    Err { message: String },
}

#[derive(Debug, serde::Deserialize)]
struct UvResult {
    uv: f64,
}

pub fn get_uv_index(
    client: &Client,
    config: &Config,
    location: &Location,
) -> Result<Option<u8>, RustormyError> {
    if config.api_keys().open_uv.is_empty() {
        return Ok(None);
    }
    let params = UvRequestParams::new(location);
    let response = client
        .get(OPEN_UV_API_URL)
        .query(&params)
        .header("x-access-token", &config.api_keys().open_uv)
        .send()?;
    let data: UvResponse = response.json()?;
    match data {
        UvResponse::Ok { result } => Ok(Some(result.uv.round() as u8)),
        UvResponse::Err { message } => Err(RustormyError::ApiReturnedError(message)),
    }
}
