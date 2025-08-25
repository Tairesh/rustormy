use crate::models::{OutputFormat, Units};
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// City name (required if lat/lon not provided)
    #[arg(short = 'c', long)]
    pub city: Option<String>,

    /// Latitude (required if city not provided)
    #[arg(short = 'y', long)]
    pub lat: Option<f64>,

    /// Longitude (required if city not provided)
    #[arg(short = 'x', long)]
    pub lon: Option<f64>,

    /// Units for temperature and wind speed (metric or imperial)
    #[arg(short = 'u', long)]
    pub units: Option<Units>,

    /// Output format (text or json)
    #[arg(short = 'o', long)]
    pub output_format: Option<OutputFormat>,

    /// Show city name in output
    #[arg(long, action = ArgAction::SetTrue)]
    pub show_city_name: bool,

    /// Use colors in output
    #[arg(long, action = ArgAction::SetTrue)]
    pub use_colors: bool,
    // /// Live mode - continuously update weather
    // #[arg(short = 'l', long, action = ArgAction::SetTrue)]
    // pub live_mode: bool,
}
