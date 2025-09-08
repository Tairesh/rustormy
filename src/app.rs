use crate::config::{Cli, Config};
use crate::display::formatter::WeatherFormatter;
use crate::errors::RustormyError;
use crate::models::Provider;
use crate::weather::{GetWeather, GetWeatherProvider};
use reqwest::blocking::Client;
use std::time::Duration;

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H\x1B[?25l");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

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
        let provider = GetWeatherProvider::new(config.provider().unwrap_or_default());
        let formatter = WeatherFormatter::new(&config);
        Ok(Self {
            client,
            config,
            provider,
            formatter,
        })
    }

    pub fn run(&mut self) {
        loop {
            match self.provider.get_weather(&self.client, &self.config) {
                Ok(weather) => {
                    if self.config.live_mode() {
                        clear_screen();
                    }
                    self.formatter.display(weather);
                }
                Err(error) => match error {
                    RustormyError::ApiReturnedError(_) | RustormyError::HttpRequestFailed(_) => {
                        let p: Provider = (&self.provider).into();
                        if self.config.verbose() >= 1 {
                            // TODO: Log this instead of printing to stderr
                            eprintln!("Provider {p:?} failed: {error:?}");
                        }
                        self.provider =
                            GetWeatherProvider::new(self.config.provider().unwrap_or_else(|| {
                                self.formatter.display_error(&error);
                            }));
                        continue;
                    }
                    _ => {
                        self.formatter.display_error(&error);
                    }
                },
            }
            if !self.config.live_mode() {
                break;
            }
            let sleep_duration = Duration::from_secs(self.config.live_mode_interval());
            std::thread::sleep(sleep_duration);
        }
    }
}
