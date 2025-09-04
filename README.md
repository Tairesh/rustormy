# rustormy

Minimal and neofetch-like weather CLI with multiple data providers support, ASCI-icons, ANSI colors, localization and
various output modes.

[![Tests](https://github.com/Tairesh/rustormy/actions/workflows/tests.yml/badge.svg)](https://github.com/Tairesh/rustormy/actions/workflows/tests.yml)
[![Crates.io](https://img.shields.io/crates/v/rustormy.svg)](https://crates.io/crates/rustormy)
[![Commit activity](https://img.shields.io/github/commit-activity/m/tairesh/rustormy)](https://github.com/Tairesh/rustormy/commits/main)
[![Lines of code](https://tokei.rs/b1/github/Tairesh/rustormy?category=code)](https://github.com/Tairesh/rustormy/tree/main)

![Screenshot](.github/assets/live.png)

---

## Current features

- Fetch weather data from multiple providers: [OpenMeteo](https://open-meteo.com/) (default, no API key required),
  [OpenWeatherMap](https://openweathermap.org/), amd [World Weather Online](https://www.worldweatheronline.com/)
- If first provider is not available (e.g. API limit exceeded), automatically fallback to the next one
- Display current temperature, weather condition, wind speed and direction, humidity. precipitation, atmospheric
  pressure.
- Display ASCII art icons for weather conditions
- Supports ANSI colors in terminal output
- Supports geocoding by city name or latitude/longitude input
- Supports caching geocoding results to reduce API calls
- Supports multiple languages (English, Russian, Spanish; more to come)
- Supports imperial and metric units
- Supports different text output modes (full, compact, one-line) and JSON output
- Supports live mode (periodically fetch and display updated weather data)
- Cross-platform (Linux, macOS, Windows)

## Planned features

- More weather-related information (e.g. sunrise/sunset times, moon phase, etc.)
- Support for more weather APIs and data providers
- More customization options (e.g. colors, layout, etc.)
- More languages
- Improved test coverage
- Homebrew, nix, rpm, deb packages
- Docker image (just for fun)

## Installation

You can install `rustormy` using `cargo`:

```sh
cargo install rustormy
```

Or download a precompiled binary from the [releases page](https://github.com/Tairesh/rustormy/releases).

## Configuration

`rustormy` uses XDG Base Directory Specification for configuration files and will create a configuration file at
`~/.config/rustormy/config.toml` to set default options at first run.

On macOS, the configuration file will be located at
`$HOME/Library/Application Support/rustormy/config.toml`.

On Windows, the configuration file will be located at
`%APPDATA%\Roaming\rustormy\config.toml`.

### Configuration options

```toml
# Rustormy Configuration File
# This file is in TOML format. See https://toml.io/ for details
# For more details, see the documentation at https://github.com/Tairesh/rustormy/tree/main?tab=readme-ov-file#configuration
#
# Possible providers: `open_meteo`, `open_weather_map`, `world_weather_online`
# Note that `open_weather_map` and `world_weather_online` require an API key
# (`api_key_owm` for Open Weather Map, `api_key_wwo` for World Weather Online)
# You can specify multiple providers in the `providers` array to try them in order
# Example: `providers = ["world_weather_online", "open_weather_map", "open_meteo"]`

providers = ["open_meteo"]
api_key_owm = ""
api_key_wwo = ""

# You can specify location either by `city` name or by `lat` and `lon` coordinates
# If both are provided, coordinates will be used

# city = "London"
# lat = 51.5074
# lon = -0.1278

# Units can be `metric` (Celsius, m/s) or `imperial` (Fahrenheit, mph)

units = "metric"

# Output format can be `text` or `json`

output_format = "text"

# Language codes: `en` (English), `es` (Spanish), `ru` (Russian)
# (more languages will be added in future)

language = "en"

# Text mode can be `full`, `compact`, or `one_line`
# `compact` mode shows same info as `full` but without labels and trailing empty lines
# `one_line` mode shows only temperature and weather condition in a single line

text_mode = "full"

# Show city name can be enabled with `show_city_name = true` to include the city name in the output
# (only works if `city` is provided, not coordinates)

show_city_name = false

# Use colors can be enabled with `use_colors = true` to colorize the text output with ANSI colors

use_colors = false

# Wind in degrees can be enabled with `wind_in_degrees = true` to show wind direction in degrees

wind_in_degrees = false

# Live mode can be enabled with `live_mode = true` to update weather data every
# `live_mode_interval` seconds (default is 300 seconds, i.e., 5 minutes)

live_mode = false
live_mode_interval = 300

# Align right can be enabled with `align_right = true` to align labels to the right

align_right = false

# Use geocoding cache can be enabled with `use_geocoding_cache = true` to cache
# previously looked up cities locally to avoid repeated API calls

use_geocoding_cache = false

# Verbosity level can be set with `verbose` (0 = errors, 1 = warnings, 2 = info, 3 = debug)

verbose = 0
```

## Usage

```
rustormy [OPTIONS]

Options:
  -c, --city <CITY>
          City name (required if lat/lon not provided)
  -y, --lat <LAT>
          Latitude (required if city not provided)
  -x, --lon <LON>
          Longitude (required if city not provided)
  -p, --provider <PROVIDER>
          Weather data provider (OpenMeteo, OpenWeatherMap, WorldWeatherOnline) [possible values: om, owm, wwo]
  -u, --units <UNITS>
          Units for temperature and wind speed [possible values: metric, imperial]
  -o, --format <OUTPUT_FORMAT>
          Output format [possible values: text, json]
  -g, --lang <LANGUAGE>
          Language for weather output [possible values: en, ru, es]
      --name
          Show city name in output
      --colors
          Use colors in output
      --degrees
          Use degrees for wind direction in output
      --compact
          Compact mode for text output (short for `--text-mode compact`)
      --one-line
          One-line mode for text output (short for `--text-mode one_line`)
  -m, --text-mode <TEXT_MODE>
          Text output mode [possible values: full, compact, one_line]
      --align-right
          Align labels to the right in text output
  -l, --live
          Live mode - continuously update weather data every 5 minutes (or specified interval)
  -i, --interval <LIVE_MODE_INTERVAL>
          Live mode update interval in seconds (default: 300)
      --no-cache
          Disable caching of geocoding results
      --clear-cache
          Clear cached geocoding results and exit
  -v, --verbose...
          Increase verbosity level (can be used multiple times)
  -h, --help
          Print help
  -V, --version
          Print version
```

## Examples

![Basic usage: `rustormy -c London`](.github/assets/basic.png)
![Colors: `rustormy -c Milan --colors`](.github/assets/colors.png)
![Imperial units: `rustormy -c "New York" --colors -u imperial`](.github/assets/imperial.png)
![Spanish translation: `rustormy -c Valparaiso -g es --name --colors`](.github/assets/spanish.png)
![Compact and one-line modes](.github/assets/compact.png)
![JSON output: `rustormy -c Ajax -o json`](.github/assets/json.png)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

This project uses `just` for basic scripting. You can install it from cargo:

```sh
cargo install just
```

Use the following commands to run tests and lint the code before committing:

```sh
just check     # Run tests and clippy
just before-commit  # Run commands before committing (lint and check)
```

To run the application in development mode with specific options, use:

```sh
cargo run -- -c London --colors
```

## Acknowledgements

This project heavily inpired by [stormy](https://github.com/ashish0kumar/stormy). Actually, it started as a clone
of `stormy`, just in Rust. Then I added more features, but kept the name similar to honor the original project.

Also, [wttr.in](https://wttr.in) was a big inspiration for this project.
