use crate::config::Config;
use crate::errors::RustormyError;
use crate::models::Location;
use chrono::{DateTime, Utc};
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
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct SunTimes {
    /// sunrise (top edge of the sun appears on the horizon)
    sunrise: Option<DateTime<Utc>>,
    /// sunrise ends (bottom edge of the sun touches the horizon)
    sunrise_end: Option<DateTime<Utc>>,
    /// morning golden hour (soft light, best time for photography) ends
    golden_hour_end: Option<DateTime<Utc>>,
    /// solar noon (sun is in the highest position)
    solar_noon: Option<DateTime<Utc>>,
    /// evening golden hour starts
    golden_hour: Option<DateTime<Utc>>,
    /// sunset starts (bottom edge of the sun touches the horizon)
    sunset_start: Option<DateTime<Utc>>,
    /// sunset (top edge of the sun touches the horizon)
    sunset: Option<DateTime<Utc>>,
    /// dusk (evening nautical twilight starts)
    dusk: Option<DateTime<Utc>>,
    /// nautical dusk (evening astronomical twilight starts)
    nautical_dusk: Option<DateTime<Utc>>,
    /// night starts (dark enough for astronomical observations)
    night: Option<DateTime<Utc>>,
    /// nadir (darkest moment of the night, sun is in the lowest position)
    nadir: Option<DateTime<Utc>>,
    /// night ends (morning astronomical twilight starts)
    night_end: Option<DateTime<Utc>>,
    /// nautical dawn (morning nautical twilight starts)
    nautical_dawn: Option<DateTime<Utc>>,
    /// dawn (morning nautical twilight ends, morning civil twilight starts)
    dawn: Option<DateTime<Utc>>,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct SunPosition {
    azimuth: f64,
    altitude: f64,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct SunInfo {
    sun_times: SunTimes,
    sun_position: SunPosition,
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct SafeExposureTime {
    st1: Option<u64>,
    st2: Option<u64>,
    st3: Option<u64>,
    st4: Option<u64>,
    st5: Option<u64>,
    st6: Option<u64>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum UvResponse {
    Ok {
        result: Box<UvResult>,
    },
    Err {
        #[serde(alias = "error")]
        message: String,
    },
}

impl UvResponse {
    fn into_uv_index(self, config: &Config) -> Option<f64> {
        match self {
            Self::Ok { result } => Some((result.uv * 10.).round() / 10.),
            Self::Err { message } => {
                if config.verbose() >= 1 {
                    // TODO: proper logging
                    eprintln!("OpenUV API error: {message}");
                }

                None
            }
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[allow(dead_code)]
struct UvResult {
    uv: f64,
    uv_time: DateTime<Utc>,
    uv_max: f64,
    uv_max_time: DateTime<Utc>,
    ozone: f64,
    ozone_time: DateTime<Utc>,
    safe_exposure_time: SafeExposureTime,
    sun_info: SunInfo,
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

    const TEST_API_RESPONSE: &str = include_str!("../../tests/data/openuv_response.json");

    #[test]
    fn test_openuv_error_response_returns_no_uv_index() {
        let response: UvResponse = serde_json::from_str(r#"{"error":"Daily API quota exceeded."}"#)
            .expect("OpenUV error payload should deserialize");
        let config = Config::default();

        assert_eq!(response.into_uv_index(&config), None);
    }

    #[test]
    fn test_openuv_valid_response_returns_uv_index() {
        let response: UvResponse =
            serde_json::from_str(TEST_API_RESPONSE).expect("Failed to parse JSON");
        let config = Config::default();

        assert_eq!(response.into_uv_index(&config), Some(4.4));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_parse_openuv_response() {
        let data: UvResponse =
            serde_json::from_str(TEST_API_RESPONSE).expect("Failed to parse JSON");
        match data {
            UvResponse::Ok { result } => {
                assert_eq!(result.uv, 4.4176);
                assert_eq!(
                    result
                        .uv_time
                        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                    "2026-03-30T20:03:08.637Z"
                );
                assert_eq!(result.uv_max, 7.1284);
                assert_eq!(
                    result
                        .uv_max_time
                        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                    "2026-03-30T17:42:49.954Z"
                );
                assert_eq!(result.ozone, 326.9);
                assert_eq!(
                    result
                        .ozone_time
                        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                    "2023-04-12T15:04:31.773Z"
                );
                assert_eq!(result.safe_exposure_time.st1, Some(38));
                assert_eq!(result.safe_exposure_time.st2, Some(45));
                assert_eq!(result.safe_exposure_time.st3, Some(60));
                assert_eq!(result.safe_exposure_time.st4, Some(75));
                assert_eq!(result.safe_exposure_time.st5, Some(121));
                assert_eq!(result.safe_exposure_time.st6, Some(226));
                assert_eq!(
                    result
                        .sun_info
                        .sun_times
                        .solar_noon
                        .map(|t| t.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)),
                    Some("2026-03-30T17:42:49.954Z".to_string())
                );
                assert!(
                    (result.sun_info.sun_position.azimuth - 0.936_844_060_974_327_9).abs() < 1e-10
                );
                assert!(
                    (result.sun_info.sun_position.altitude - 0.773_857_699_323_151_7).abs() < 1e-10
                );
            }
            UvResponse::Err { message } => panic!("Unexpected error: {message}"),
        }
    }
}
