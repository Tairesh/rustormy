use crate::config::Config;
use crate::errors::RustormyError;
use crate::logging::{Level, level_passes, state};
use crate::models::{Location, Provider};
use reqwest::blocking::RequestBuilder;
use serde::de::DeserializeOwned;
use std::fmt;
use std::time::Instant;

const BODY_LOG_LIMIT: usize = 1024;
const TRUNCATED_MARKER: &str = "…[truncated]";
/// Query parameters that carry provider API keys.
const SECRET_QUERY_PARAMS: &[&str] = &["appid", "key", "apikey"];

/// Describes the HTTP call for logging purposes.
///
/// Built once per call via the `geocode` / `weather_at` / `weather_for` / `uv_at`
/// constructors and passed to [`get_json`]. `Display` is only invoked when an
/// info-level log actually fires, so providers don't allocate label strings at
/// default verbosity.
#[derive(Debug, Clone, Copy)]
pub enum Op<'a> {
    Geocode {
        provider: Provider,
        city: &'a str,
    },
    WeatherAtCity {
        provider: Provider,
        city: &'a str,
    },
    WeatherAtCoords {
        provider: Provider,
        lat: f64,
        lon: f64,
    },
    Uv {
        lat: f64,
        lon: f64,
    },
}

impl<'a> Op<'a> {
    pub const fn geocode(provider: Provider, city: &'a str) -> Self {
        Self::Geocode { provider, city }
    }

    pub fn weather_for(provider: Provider, config: &'a Config) -> Self {
        debug_assert!(
            config.coordinates().is_some() || config.city().is_some(),
            "Op::weather_for called with neither coordinates nor city in config",
        );
        if let Some((lat, lon)) = config.coordinates() {
            Self::WeatherAtCoords { provider, lat, lon }
        } else {
            Self::WeatherAtCity {
                provider,
                city: config.city().unwrap_or(""),
            }
        }
    }
}

impl Op<'static> {
    pub fn weather_at(provider: Provider, location: &Location) -> Self {
        Self::WeatherAtCoords {
            provider,
            lat: location.latitude,
            lon: location.longitude,
        }
    }

    pub fn uv_at(location: &Location) -> Self {
        Self::Uv {
            lat: location.latitude,
            lon: location.longitude,
        }
    }
}

impl fmt::Display for Op<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Geocode { provider, city } => {
                write!(f, "provider:{provider:?} geocode \"{city}\"")
            }
            Self::WeatherAtCity { provider, city } => {
                write!(f, "provider:{provider:?} weather \"{city}\"")
            }
            Self::WeatherAtCoords { provider, lat, lon } => {
                write!(f, "provider:{provider:?} weather {lat:.2},{lon:.2}")
            }
            Self::Uv { lat, lon } => {
                write!(f, "provider:OpenUV uv {lat:.2},{lon:.2}")
            }
        }
    }
}

pub fn get_json<T>(request: RequestBuilder, op: Op<'_>) -> Result<T, RustormyError>
where
    T: DeserializeOwned,
{
    let level = state().map_or(0, |s| s.level);

    if level_passes(level, Level::Debug)
        && let Some(url) = extract_url_for_log(&request)
    {
        crate::debug!("GET {url}");
    }

    let start = level_passes(level, Level::Info).then(Instant::now);
    let response = request.send()?;
    let status = response.status();

    let parsed: T = if level_passes(level, Level::Debug) {
        let text = response.text()?;
        crate::debug!("response body: {}", truncate_for_log(&text, BODY_LOG_LIMIT));
        serde_json::from_str(&text)?
    } else {
        response.json()?
    };

    if let Some(s) = start {
        let ms = s.elapsed().as_millis();
        crate::info!("{op} → {} in {}ms", status.as_u16(), ms);
    }

    Ok(parsed)
}

fn extract_url_for_log(request: &RequestBuilder) -> Option<String> {
    request
        .try_clone()
        .and_then(|c| c.build().ok())
        .map(|mut r| {
            redact_url(r.url_mut());
            r.url().to_string()
        })
}

/// Replaces the values of [`SECRET_QUERY_PARAMS`] in `url` with `***`.
pub(crate) fn redact_url(url: &mut reqwest::Url) {
    let is_secret = |name: &str| SECRET_QUERY_PARAMS.contains(&name);
    if !url.query_pairs().any(|(name, _)| is_secret(&name)) {
        return;
    }
    let pairs: Vec<(String, String)> = url
        .query_pairs()
        .map(|(name, value)| {
            let value = if is_secret(&name) {
                "***".to_string()
            } else {
                value.into_owned()
            };
            (name.into_owned(), value)
        })
        .collect();
    url.query_pairs_mut().clear().extend_pairs(pairs);
}

fn truncate_for_log(s: &str, byte_limit: usize) -> String {
    if s.len() <= byte_limit {
        return s.to_string();
    }
    let mut cut = byte_limit;
    while cut > 0 && !s.is_char_boundary(cut) {
        cut -= 1;
    }
    let mut out = String::with_capacity(cut + TRUNCATED_MARKER.len());
    out.push_str(&s[..cut]);
    out.push_str(TRUNCATED_MARKER);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_short_string_is_unchanged() {
        assert_eq!(truncate_for_log("hello", 1024), "hello");
    }

    #[test]
    fn truncate_long_string_marks_truncation() {
        let s = "x".repeat(2048);
        let out = truncate_for_log(&s, 1024);
        assert!(out.starts_with(&"x".repeat(1024)));
        assert!(out.ends_with("…[truncated]"));
        assert_eq!(out.len(), 1024 + "…[truncated]".len());
    }

    #[test]
    fn truncate_handles_utf8_boundaries() {
        let s = "абв";
        let out = truncate_for_log(s, 3);
        assert!(out.starts_with('а'));
        assert!(out.ends_with("…[truncated]"));
    }

    #[test]
    fn op_display_geocode() {
        let op = Op::geocode(Provider::OpenMeteo, "Lisbon");
        assert_eq!(op.to_string(), "provider:OpenMeteo geocode \"Lisbon\"");
    }

    #[test]
    fn op_display_weather_at() {
        let location = Location {
            name: "Lisbon".to_string(),
            latitude: 38.7223,
            longitude: -9.1393,
        };
        let op = Op::weather_at(Provider::OpenMeteo, &location);
        assert_eq!(op.to_string(), "provider:OpenMeteo weather 38.72,-9.14");
    }

    #[test]
    fn op_display_uv_at() {
        let location = Location {
            name: "Lisbon".to_string(),
            latitude: 38.7223,
            longitude: -9.1393,
        };
        let op = Op::uv_at(&location);
        assert_eq!(op.to_string(), "provider:OpenUV uv 38.72,-9.14");
    }

    fn redacted(url: &str) -> String {
        let mut url = reqwest::Url::parse(url).unwrap();
        redact_url(&mut url);
        url.to_string()
    }

    #[test]
    fn redact_url_masks_each_secret_param() {
        for param in ["appid", "key", "apikey"] {
            let out = redacted(&format!(
                "https://api.example.com/v1?q=London&{param}=SECRET"
            ));
            assert_eq!(
                out,
                format!("https://api.example.com/v1?q=London&{param}=***")
            );
        }
    }

    #[test]
    fn redact_url_leaves_url_without_secrets_unchanged() {
        let url = "https://api.example.com/v1?q=New%20York&units=metric";
        assert_eq!(redacted(url), url);
    }

    #[test]
    fn redact_url_leaves_url_without_query_unchanged() {
        let url = "https://api.example.com/v1/current";
        assert_eq!(redacted(url), url);
    }

    #[test]
    fn extract_url_for_log_redacts_key() {
        let request = reqwest::blocking::Client::new()
            .get("https://api.example.com/v1")
            .query(&[("q", "London"), ("key", "SECRET")]);
        let url = extract_url_for_log(&request).unwrap();
        assert!(url.contains("q=London"), "{url}");
        assert!(url.contains("key=***"), "{url}");
        assert!(!url.contains("SECRET"), "{url}");
    }

    #[test]
    fn http_error_does_not_leak_key() {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(2))
            .build()
            .unwrap();
        let request = client.get("http://127.0.0.1:1/?q=London&key=SECRET");
        let err =
            get_json::<serde_json::Value>(request, Op::geocode(Provider::OpenMeteo, "London"))
                .unwrap_err();
        assert!(
            matches!(err, RustormyError::HttpRequestFailed(_)),
            "{err:?}"
        );
        let message = err.to_string();
        assert!(message.contains("key=***"), "{message}");
        assert!(!message.contains("SECRET"), "{message}");
    }
}
