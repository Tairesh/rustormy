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
    Ok {
        result: UvResult,
    },
    Err {
        _message: Option<String>,
        _error: Option<String>,
    },
}

impl UvResponse {
    fn into_uv_index(self) -> Option<f64> {
        match self {
            Self::Ok { result } => Some((result.uv * 10.).round() / 10.),
            Self::Err { .. } => None,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct UvResult {
    uv: f64,
}

pub fn get_uv_index(
    client: &Client,
    config: &Config,
    location: &Location,
) -> Result<Option<f64>, RustormyError> {
    if config.api_keys().open_uv.is_empty() {
        return Ok(None);
    }
    let params = UvRequestParams::new(location);
    let response = client
        .get(OPEN_UV_API_URL)
        .query(&params)
        .header("x-access-token", &config.api_keys().open_uv)
        .send()?;
    Ok(response.json::<UvResponse>()?.into_uv_index())
}

#[cfg(test)]
mod tests {
    use super::UvResponse;

    #[test]
    fn test_openuv_error_response_returns_no_uv_index() {
        let response: UvResponse = serde_json::from_str(r#"{"error":"Daily API quota exceeded."}"#)
            .expect("OpenUV error payload should deserialize");

        assert_eq!(response.into_uv_index(), None);
    }
}
