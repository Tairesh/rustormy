use crate::config::{Cli, Config};
use crate::display::formatter::WeatherFormatter;
use crate::errors::RustormyError;
use crate::live::run as run_live;
use crate::models::{Provider, Weather};
use crate::weather::{GetWeather, GetWeatherProvider};
use reqwest::blocking::Client;
use std::time::Duration;

pub struct App {
    client: Client,
    config: Config,
    provider: GetWeatherProvider,
    formatter: WeatherFormatter,
}

impl App {
    pub fn new() -> Result<App, RustormyError> {
        let mut config = Config::new(Cli::new())?;
        let client = Client::builder()
            .user_agent(concat!("rustormy/", env!("CARGO_PKG_VERSION")))
            .timeout(Duration::from_secs(config.connect_timeout()))
            .build()?;
        let provider = GetWeatherProvider::new(config.take_next_provider().unwrap_or_default());
        let formatter = WeatherFormatter::new(&config);
        Ok(Self {
            client,
            config,
            provider,
            formatter,
        })
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn formatter(&self) -> &WeatherFormatter {
        &self.formatter
    }

    pub fn fetch_with_fallback(&mut self) -> Result<Weather, RustormyError> {
        loop {
            match self.provider.get_weather(&self.client, &self.config) {
                Ok(weather) => return Ok(weather),
                Err(error) => match error {
                    RustormyError::ApiReturnedError(_) | RustormyError::HttpRequestFailed(_) => {
                        let p: Provider = (&self.provider).into();
                        if self.config.verbose() >= 1 {
                            eprintln!("Provider {p:?} failed: {error:?}");
                        }
                        let Some(next) = self.config.take_next_provider() else {
                            return Err(error);
                        };
                        self.provider = GetWeatherProvider::new(next);
                    }
                    _ => return Err(error),
                },
            }
        }
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
