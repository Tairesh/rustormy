use crate::config::{Cli, Config};
use crate::display::formatter::WeatherFormatter;
use crate::errors::RustormyError;
use crate::live::run as run_live;
use crate::models::Weather;
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
        let mut last_error = RustormyError::InvalidConfiguration("No weather providers configured");
        for &p in self.config.providers() {
            match GetWeatherProvider::new(p).get_weather(&self.client, &self.config) {
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
