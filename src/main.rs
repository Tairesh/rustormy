mod cli;
mod config;
mod display;
mod errors;
mod models;
#[cfg(test)]
mod tests;
mod weather;

use clap::Parser;
use cli::Cli;
use config::Config;
use display::formatter::WeatherFormatter;
use std::time::Duration;
use weather::{GetWeather, GetWeatherProvider};

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H\x1B[?25l");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

#[tokio::main]
async fn main() -> Result<(), errors::RustormyError> {
    let config = Config::new(&Cli::parse())?;
    let provider = GetWeatherProvider::new(config.provider());
    let formatter = WeatherFormatter::new(config.clone());

    loop {
        match provider.get_weather(&config).await {
            Ok(weather) => {
                if config.live_mode() {
                    clear_screen();
                }
                formatter.display(weather);
            }
            Err(error) => formatter.display_error(&error),
        }
        if !config.live_mode() {
            break;
        }
        tokio::time::sleep(Duration::from_secs(config.live_mode_interval())).await;
    }

    Ok(())
}
