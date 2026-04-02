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
    Err { error: String },
}

impl UvResponse {
    fn into_uv_index(self, config: &Config) -> Option<f64> {
        match self {
            Self::Ok { result } => Some((result.uv * 10.).round() / 10.),
            Self::Err { error } => {
                if config.verbose() >= 1 {
                    // TODO: proper logging
                    eprintln!("OpenUV API error: {error}");
                }

                None
            }
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
    Ok(response.json::<UvResponse>()?.into_uv_index(config))
}

#[cfg(test)]
mod tests {
    use super::UvResponse;
    use crate::config::Config;

    #[test]
    fn test_openuv_error_response_returns_no_uv_index() {
        let response: UvResponse = serde_json::from_str(r#"{"error":"Daily API quota exceeded."}"#)
            .expect("OpenUV error payload should deserialize");
        let config = Config::default();

        assert_eq!(response.into_uv_index(&config), None);
    }

    #[test]
    fn test_openuv_valid_response_returns_uv_index() {
        let response: UvResponse = serde_json::from_str(
            r#"{
  "result": {
    "uv": 5.7346,
    "uv_time": "2026-04-02T14:08:34.531Z",
    "uv_max": 12.6597,
    "uv_max_time": "2026-04-02T04:52:16.247Z",
    "ozone": 281.2,
    "ozone_time": "2023-04-12T15:04:31.773Z",
    "safe_exposure_time": {
      "st1": null,
      "st2": null,
      "st3": null,
      "st4": null,
      "st5": null,
      "st6": null
    },
    "sun_info": {
      "sun_times": {
        "solarNoon": "2026-04-02T04:52:16.247Z",
        "nadir": "2026-04-01T16:52:16.247Z",
        "sunrise": "2026-04-01T22:43:17.825Z",
        "sunset": "2026-04-02T11:01:14.669Z",
        "sunriseEnd": "2026-04-01T22:45:31.485Z",
        "sunsetStart": "2026-04-02T10:59:01.009Z",
        "dawn": "2026-04-01T22:21:40.228Z",
        "dusk": "2026-04-02T11:22:52.266Z",
        "nauticalDawn": "2026-04-01T21:56:28.084Z",
        "nauticalDusk": "2026-04-02T11:48:04.409Z",
        "nightEnd": "2026-04-01T21:31:08.119Z",
        "night": "2026-04-02T12:13:24.374Z",
        "goldenHourEnd": "2026-04-01T23:11:49.210Z",
        "goldenHour": "2026-04-02T10:32:43.284Z"
      },
      "sun_position": {
        "azimuth": 1.9922084535953004,
        "altitude": -0.7782688029847042
      }
    }
  }
}"#,
        )
        .expect("OpenUV valid payload should deserialize");
        let config = Config::default();

        assert_eq!(response.into_uv_index(&config), Some(5.7));
    }
}
