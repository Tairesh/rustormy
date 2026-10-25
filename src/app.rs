use crate::config::{Cli, Config};
use crate::display::formatter::WeatherFormatter;
use crate::errors::RustormyError;
use crate::live::run as run_live;
use crate::models::{Provider, Weather};
use crate::weather::{GetWeather, GetWeatherProvider, enrich};
use reqwest::blocking::Client;
use std::time::Duration;

pub struct App {
    client: Client,
    config: Config,
    formatter: WeatherFormatter,
}

impl App {
    pub fn new() -> Result<App, RustormyError> {
        let config = Config::new(Cli::new())?;
        if !config.live_mode() {
            crate::logging::init(config.verbose(), config.format().use_colors);
        }
        let client = Client::builder()
            .user_agent(concat!("rustormy/", env!("CARGO_PKG_VERSION")))
            .timeout(Duration::from_secs(config.connect_timeout()))
            .build()?;
        let formatter = WeatherFormatter::new(&config);
        Ok(Self {
            client,
            config,
            formatter,
        })
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn formatter(&self) -> &WeatherFormatter {
        &self.formatter
    }

    pub fn fetch_with_fallback(&self) -> Result<Weather, RustormyError> {
        self.fetch_with(|p| GetWeatherProvider::new(p).get_weather(&self.client, &self.config))
    }

    // Keep provider selection independent of transport so retries can be tested offline.
    fn fetch_with(
        &self,
        mut fetch: impl FnMut(Provider) -> Result<Weather, RustormyError>,
    ) -> Result<Weather, RustormyError> {
        let mut last_error = RustormyError::InvalidConfiguration("No weather providers configured");
        for &p in self.config.providers() {
            match fetch(p) {
                Ok(mut weather) => {
                    enrich(&mut weather, &self.client, &self.config);
                    return Ok(weather);
                }
                Err(
                    error @ (RustormyError::ApiReturnedError(_)
                    | RustormyError::HttpRequestFailed(_)),
                ) => {
                    crate::warn!("Provider {p:?} failed: {error}");
                    last_error = error;
                }
                Err(error) => return Err(error),
            }
        }
        Err(last_error)
    }

    pub fn run(&mut self) {
        if self.config.live_mode() {
            if let Err(error) = run_live(self) {
                self.formatter.display_error(&error);
            }
            return;
        }

        match self.fetch_with_fallback() {
            Ok(weather) => self.formatter.display(&weather),
            Err(error) => self.formatter.display_error(&error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::sample_weather;
    use std::collections::VecDeque;

    fn app(providers: &[Provider]) -> App {
        let mut config = Config::default();
        config.set_providers(providers.to_vec());
        App {
            client: Client::new(),
            formatter: WeatherFormatter::new(&config),
            config,
        }
    }

    fn reading(description: &str) -> Weather {
        let mut weather = sample_weather();
        weather.description = description.to_string();
        weather
    }

    #[test]
    fn retries_primary_after_fallback_and_total_outage() {
        use Provider::{OpenMeteo, Yr};

        let app = app(&[OpenMeteo, Yr]);
        let mut responses = VecDeque::from([
            (OpenMeteo, Ok(reading("primary"))),
            (
                OpenMeteo,
                Err(RustormyError::ApiReturnedError("primary down".to_string())),
            ),
            (Yr, Ok(reading("fallback"))),
            (OpenMeteo, Ok(reading("primary recovered"))),
            (
                OpenMeteo,
                Err(RustormyError::ApiReturnedError("primary down".to_string())),
            ),
            (
                Yr,
                Err(RustormyError::ApiReturnedError("fallback down".to_string())),
            ),
            (OpenMeteo, Ok(reading("primary recovered again"))),
        ]);
        let mut fetch = |provider| {
            let (expected_provider, response) = responses.pop_front().expect("unexpected attempt");
            assert_eq!(provider, expected_provider);
            response
        };

        for expected in ["primary", "fallback", "primary recovered"] {
            assert_eq!(app.fetch_with(&mut fetch).unwrap().description, expected);
        }
        assert!(matches!(
            app.fetch_with(&mut fetch),
            Err(RustormyError::ApiReturnedError(message)) if message == "fallback down"
        ));
        assert_eq!(
            app.fetch_with(&mut fetch).unwrap().description,
            "primary recovered again"
        );
        assert!(responses.is_empty());
        assert_eq!(app.config.providers(), &[OpenMeteo, Yr]);
    }

    #[test]
    fn http_error_falls_back_in_configured_order_and_stops_on_success() {
        let providers = [Provider::Yr, Provider::OpenMeteo, Provider::WeatherApi];
        let app = app(&providers);
        let mut attempts = Vec::new();
        let weather = app
            .fetch_with(|provider| {
                attempts.push(provider);
                if provider == Provider::Yr {
                    // Building an invalid request produces an HTTP error without sending it.
                    Err(Client::new().get("://invalid").build().unwrap_err().into())
                } else {
                    let mut weather = reading("fallback");
                    weather.is_day = None;
                    Ok(weather)
                }
            })
            .unwrap();
        assert_eq!(attempts, providers[..2]);
        assert_eq!(weather.description, "fallback");
        assert!(
            weather.is_day.is_some(),
            "enrich must run on the fallback reading"
        );
    }

    #[test]
    fn non_retryable_error_does_not_try_fallback() {
        let app = app(&[Provider::OpenMeteo, Provider::Yr]);
        let mut attempts = Vec::new();
        let result = app.fetch_with(|provider| {
            attempts.push(provider);
            Err(RustormyError::MissingApiKey(provider))
        });
        assert!(matches!(
            result,
            Err(RustormyError::MissingApiKey(Provider::OpenMeteo))
        ));
        assert_eq!(attempts, [Provider::OpenMeteo]);
    }

    #[test]
    fn empty_provider_list_returns_configuration_error() {
        let app = app(&[]);
        let result = app.fetch_with(|_| panic!("no provider should be called"));
        assert!(matches!(
            result,
            Err(RustormyError::InvalidConfiguration(
                "No weather providers configured"
            ))
        ));
    }
}
