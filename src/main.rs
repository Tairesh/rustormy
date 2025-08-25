mod cli;
mod config;
mod display;
mod errors;
mod models;
#[cfg(test)]
mod tests;
mod weather;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use config::Config;
use display::formatter::WeatherFormatter;
use weather::{GetWeather, WeatherProvider};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new(&Cli::parse())?;
    config.validate()?;

    let provider = WeatherProvider::new(config.provider());
    let formatter = WeatherFormatter::new(config.clone());

    match provider.get_weather(&config).await {
        Ok(weather) => formatter.display(weather),
        Err(error) => formatter.display_error(&error),
    }

    Ok(())
}
