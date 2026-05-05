use crate::config::Config;
use crate::errors::RustormyError;
use crate::models::Weather;
use crate::weather::openuv::get_uv_index;
use crate::weather::sun;
use chrono::Utc;
use reqwest::blocking::Client;

pub fn enrich(
    weather: &mut Weather,
    client: &Client,
    config: &Config,
) -> Result<(), RustormyError> {
    if weather.is_day.is_none() {
        weather.is_day = Some(sun::is_daytime(&weather.location, Utc::now()));
    }
    if weather.uv_index.is_none() && !config.api_keys().open_uv.is_empty() {
        weather.uv_index = get_uv_index(client, config, &weather.location)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Location, WeatherConditionIcon};

    fn make_weather() -> Weather {
        Weather {
            temperature: 0.0,
            feels_like: 0.0,
            humidity: 0,
            dew_point: 0.0,
            precipitation: 0.0,
            pressure: 0,
            wind_speed: 0.0,
            wind_direction: 0,
            uv_index: None,
            is_day: None,
            description: String::new(),
            icon: WeatherConditionIcon::Clear,
            location: Location {
                name: String::new(),
                latitude: 0.0,
                longitude: 0.0,
            },
        }
    }

    #[test]
    fn fills_is_day_when_none() {
        let mut weather = make_weather();
        assert!(weather.is_day.is_none());
        let client = Client::new();
        let config = Config::default();
        enrich(&mut weather, &client, &config).expect("enrich should succeed");
        assert!(weather.is_day.is_some(), "is_day should be populated");
    }

    #[test]
    fn does_not_overwrite_false_is_day_when_set() {
        let mut weather = make_weather();
        weather.is_day = Some(false);
        let client = Client::new();
        let config = Config::default();
        enrich(&mut weather, &client, &config).expect("enrich should succeed");
        assert_eq!(
            weather.is_day,
            Some(false),
            "is_day should not be overwritten"
        );
    }

    #[test]
    fn does_not_overwrite_true_is_day_when_set() {
        let mut weather = make_weather();
        weather.is_day = Some(true);
        let client = Client::new();
        let config = Config::default();
        enrich(&mut weather, &client, &config).expect("enrich should succeed");
        assert_eq!(
            weather.is_day,
            Some(true),
            "is_day should not be overwritten"
        );
    }

    #[test]
    fn skips_uv_when_openuv_key_empty() {
        let mut weather = make_weather();
        let client = Client::new();
        let config = Config::default();
        enrich(&mut weather, &client, &config).expect("enrich should succeed");
        assert_eq!(weather.uv_index, None, "no UV when key is empty");
    }

    #[test]
    fn does_not_overwrite_uv_when_set() {
        let mut weather = make_weather();
        weather.uv_index = Some(4.2);
        let client = Client::new();
        let mut config = Config::default();
        // Set a non-empty OpenUV key so the empty-key short-circuit does
        // not fire. The is_none() guard on uv_index should prevent the
        // OpenUV HTTP call from happening — if it did fire, the fake key
        // would produce an error and this test would fail.
        config.api_keys_mut().open_uv = "fake-key".to_string();
        enrich(&mut weather, &client, &config).expect("enrich should succeed");
        assert_eq!(weather.uv_index, Some(4.2));
    }
}
