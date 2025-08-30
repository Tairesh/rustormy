use crate::cache::clear_cache;
use crate::models::{Language, OutputFormat, Provider, TextMode, Units};
use clap::{ArgAction, Parser};

#[allow(clippy::struct_excessive_bools)]
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// City name (required if lat/lon not provided)
    #[arg(short = 'c', long)]
    pub city: Option<String>,

    /// Latitude (required if city not provided)
    #[arg(short = 'y', long, allow_negative_numbers = true)]
    pub lat: Option<f64>,

    /// Longitude (required if city not provided)
    #[arg(short = 'x', long, allow_negative_numbers = true)]
    pub lon: Option<f64>,

    /// Weather data provider
    #[arg(short = 'p', long, value_enum)]
    pub provider: Option<Provider>,

    /// Units for temperature and wind speed
    #[arg(short = 'u', long)]
    pub units: Option<Units>,

    /// Output format
    #[arg(short = 'o', long = "format", value_enum, alias = "output-format")]
    pub output_format: Option<OutputFormat>,

    /// Language for weather output
    #[arg(short = 'g', long = "lang", value_enum, alias = "language")]
    pub language: Option<Language>,

    /// Show city name in output
    #[arg(long="name", action = ArgAction::SetTrue, alias="show-city-name")]
    pub show_city_name: bool,

    /// Use colors in output
    #[arg(long="colors", action = ArgAction::SetTrue, alias="use-colors")]
    pub use_colors: bool,

    /// Use degrees for wind direction in output
    #[arg(long="degrees", action = ArgAction::SetTrue)]
    pub use_degrees_for_wind: bool,

    /// Compact mode for text output (short for `--text-mode compact`)
    #[arg(long="compact", action = ArgAction::SetTrue)]
    pub compact_mode: bool,

    /// One-line mode for text output (short for `--text-mode one-line`)
    #[arg(long="one-line", action = ArgAction::SetTrue)]
    pub one_line_mode: bool,

    /// Text mode for text output
    #[arg(short = 'm', long = "text-mode", value_enum)]
    pub text_mode: Option<TextMode>,

    /// Live mode - continuously update weather data every 5 minutes (or specified interval)
    #[arg(short = 'l', long = "live", action = ArgAction::SetTrue, alias="live-mode")]
    pub live_mode: bool,

    /// Live mode update interval in seconds (default: 300)
    #[arg(
        short = 'i',
        long = "interval",
        requires = "live_mode",
        alias = "live-mode-interval"
    )]
    pub live_mode_interval: Option<u64>, // in seconds, default to 300 (5 minutes)

    /// Disable caching of geocoding results
    #[arg(long, action = ArgAction::SetTrue)]
    pub no_cache: bool,

    /// Clear cached geocoding results and exit
    #[arg(long, action = ArgAction::SetTrue)]
    pub clear_cache: bool,
}

impl Cli {
    pub fn new() -> Self {
        let cli = Cli::parse();

        if cli.clear_cache {
            clear_cache().expect("Failed to clear cache");
            println!("Cache cleared successfully.");
            std::process::exit(0);
        }

        cli
    }
}
