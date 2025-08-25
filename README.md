# rustormy

Minimal and neofetch-like weather CLI inspired by
[stormy](https://github.com/ashish0kumar/stormy) and
[rainy](https://github.com/liveslol/rainy), written in 🦀 Rust

[![Tests](https://github.com/Tairesh/rustormy/actions/workflows/tests.yml/badge.svg)](https://github.com/Tairesh/rustormy/actions/workflows/tests.yml)
[![Crates.io](https://img.shields.io/crates/v/rustormy.svg)](https://crates.io/crates/rustormy)
[![Commit activity](https://img.shields.io/github/commit-activity/m/tairesh/rustormy)](https://github.com/Tairesh/rustormy/commits/main)
[![Lines of code](https://tokei.rs/b1/github/Tairesh/rustormy?category=code)](https://github.com/Tairesh/rustormy/tree/main)

---

## Current features

- Fetch weather data from [OpenMeteo](https://open-meteo.com/) (no API key required)
- Supports ANSI colors
- Supports JSON output
- Supports imperial and metric units

## Planned features

- Support for more weather APIs (e.g. OpenWeatherMap, WeatherAPI
- More detailed weather information (e.g. dew point, UV index, etc.)
- More customization options (e.g. colors, layout, etc.)
- Live mode (e.g. update every X minutes)

## Installation

You can install `rustormy` using `cargo`:

```sh
cargo install rustormy
```

Or download a precompiled binary from the [releases page](https://github.com/Tairesh/rustormy/releases).

## Configuration

`rustormy` uses XDG Base Directory Specification for configuration files and will create a configuration file at
`~/.config/rustormy/config.toml` to set default options at first run.
On Windows, the configuration file will be located at
`%APPDATA%\rustormy\config.toml`.

### Configuration options

```toml
# Default city name (if not provided via CLI)
city = "New York"
# Default latitude (if not provided via CLI)
lat = 40.7128
# Default longitude (if not provided via CLI)
lon = -74.0060
# Weather data provider (currently only "open_meteo" is supported)
provider = "open_meteo"
# Default units for temperature and wind speed (metric or imperial)
units = "metric"
# Default output format (text or json)
output_format = "text"
# Show city name in output
show_city_name = true
# Use colors in output
use_colors = true
```

## Usage

```
rustormy [OPTIONS]

Options:
  -c, --city <CITY>                    City name (required if lat/lon not provided)
  -y, --lat <LAT>                      Latitude (required if city not provided)
  -x, --lon <LON>                      Longitude (required if city not provided)
  -u, --units <UNITS>                  Units for temperature and wind speed [possible values: metric, imperial]
  -o, --output-format <OUTPUT_FORMAT>  Output format (text or json) [possible values: text, json]
      --show-city-name                 Show city name in output
      --use-colors                     Use colors in output
  -h, --help                           Print help
  -V, --version                        Print version
```

## Examples

![Basic usage: `rustormy -c London`](.github/assets/basic.png)
![Colors: `rustormy -c Batumi --use-colors`](.github/assets/colors.png)
![Imperial units: `rustormy -c "New York" --use-colors -u imperial`](.github/assets/imperial.png)
![JSON output: `rustormy -c Ajax -o json`](.github/assets/json.png)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.
