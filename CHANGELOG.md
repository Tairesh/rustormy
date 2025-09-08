# Changelog

All notable changes to this project will be documented in this file.
See [Keep a Changelog](https://keepachangelog.com/) for details.
This project adheres to [Semantic Versioning](https://semver.org/).

## [Upcoming]

### Changed

- Changed config file structure to use sections for better organization.
  API keys and text formatting are now grouped under their respective sections.
  Old config file structure is still supported for backward compatibility.
  If you encounter any migration issues (from config file v0.3.4 or older),
  please [report](https://github.com/Tairesh/rustormy/issues/new?template=bug_report.md) them.

### Fixed

- Refactored config file handling to support backward compatibility and easier future changes.

## [0.3.4] - 2025-09-08

### Added

- Added new weather data provider: [WeatherAPI.com](https://weatherapi.com/) (`weather_api` or `wa` in config/CLI).
- Added `api_key_wa` option to config file for WeatherAPI.com API key.

### Fixed

- Fixed incorrect icon detection for weather conditions provided by World Weather Online provider.
- Improved code quality and maintainability.

## [0.3.3] - 2025-09-07

### Added

- Added option `connect_timeout` to config file to set providers' APIs connection timeout in seconds (default is 10
  seconds).

### Changed

- Slightly changed default colors for better readability.
- Improved code quality and maintainability.

## [0.3.2] - 2025-09-04

### Added

- Added wind info in one-line mode.

### Fixed

- Fixed minor bugs and improved code quality.

## [0.3.1] - 2025-09-01

### Added

- Added UV index to weather information display (only supported by World Weather Online provider for now).
- Added dew point to weather information (calculated from temperature and humidity by Magnus formula).
- Added `--align-right` CLI option to align labels to the right in text output (as `align_right` config option).

### Changed

- Changed default layout, now precipitation is shown in separate line, humidity and dew point are shown together.

### Fixed

- Fixed bug with World Weather Online not working in different languages.
- Minor code improvements and optimizations.

## [0.3.0] - 2025-08-31

### Added

- Now `rustormy` will try to get data from other providers if the first one fails.
- Added `providers` option to config file to specify a list of providers in order of preference.
- Added `-v/--verbose` option to show error details when a provider fails.
- Implemented simple file cache for getting coordinates from city names to reduce API calls.
- Added `use_geocoding_cache` option to config file to enable/disable caching (disabled for default).
- Added `--no-cache` CLI option to disable caching for current run (overrides config file).
- Added `--clear-cache` CLI option to clear cache directory and exit.

### Changed

- Deprecated config file option `provider` in favor of `providers` which accepts a list of providers in order of
  preference. Old name is still supported for backward compatibility. Also `--provider` CLI option is still supported
  and will override config file providers list with a single provider.

### Fixed

- Improved code quality and maintainability.

## [0.2.2] - 2025-08-30

### Added

- Add World Weather Online as a new weather data provider option (`--provider wwo` or `provider = "wwo"` in config).
- Support for multiple API keys via config file (`api_key_owm` for OpenWeatherMap and `api_key_wwo` for World Weather
  Online).
- More unit tests for better coverage.

### Changed

- Config file option `use_degrees_for_wind` renamed to `wind_in_degrees` for clarity. Old name is still supported for
  backward compatibility.
- Removed some dependencies to reduce binary size.
- Improved error handling and messages.

### Fixed

- Wind speed now displays one decimal place.

### Removed

- Config file option `api_key` removed due to introduction of multiple providers. Use `api_key_owm` for OpenWeatherMap
  and `api_key_wwo` for World Weather Online instead. Old name is still supported for backward compatibility. Each
  provider will check for its own API key first, then fallback to `api_key` if not found.

## [0.2.1] - 2025-08-29

### Added

- Add option `align_right` to config file for aligning labels to the right in text output. By default,
  labels are left-aligned.

### Changed

- Improved code quality and maintainability by using `enum_dispatch` for weather providers.

### Fixed

- Fix negative coordinates being parsed as unexpected arguments.

## [0.2.0] - 2025-08-28

### Added

- Add `--one-line` option to display weather information in a single line.
- Add `--text-mode` option to CLI and `text_mode` option to config file to choose between `full` (default), `compact`
  and `one_line` modes.
- Implement automatic config file migration for `compact_mode` to `text_mode`.

### Changed

- Temperature now shows only one decimal place for better readability.

### Removed

- :boom: Config file option `compact_mode` removed due to introduction of multiple text modes. Use
  `text_mode = "compact"`
  instead. `--compact` CLI option is still supported though.

## [0.1.5] - 2025-08-27

### Added

- Compact mode (`--compact` in cli or `compact_mode = true` in config) to use only 5 lines of output and less width
  without labels.
- Option to show wind direction in degrees instead of arrows (`--degrees` in cli or `use_degrees_for_wind = true` in
  config).
- Unit tests for verifying correct formatting in different modes and configurations.

### Changed

- The following CLI options have been renamed. Old names are still supported for backward compatibility:
    - `--show-city-name` to `--name`
    - `--use-colors` to `--colors`
    - `--live-mode` to `--live`
    - `--live-mode-interval` to `--interval`
    - `--output-format` to `--format`
    - `--language` to `--lang`

## [0.1.4] - 2025-08-27

### Added

- 🎌 Multilanguage support. Added translations for Russian and Spanish languages. More languages to come, contributions
  welcome!

### Changed

- Use arrow icons for wind direction instead of degrees.
- Improved code quality.

## [0.1.3] - 2025-08-26

### Added

- Live mode (periodically fetch and display updated weather data).

### Fixed

- Minor refinements and optimizations.

## [0.1.2] - 2025-08-26

### Added

- Added this changelog file.
- Added automatic cargo publish on new git tag via GitHub Actions.

### Fixed

- Fixed build issue on Windows.
- Refactored codebase for better maintainability and readability.
- Improved error handling.
- Updated dependencies to their latest versions.

## [0.1.1] - 2025-08-25

### Added

- Support for [OpenWeatherMap](https://openweathermap.org/) as an alternative weather data provider.
- Ability to choose weather data provider via configuration file or command-line argument.

## [0.1.0] - 2025-08-25

Initial release.

### Added

- Fetch and display weather data from [OpenMeteo](https://open-meteo.com/)
- Basic configuration options via XDG config file and command-line arguments.
- Support for city name or latitude/longitude input.
- ANSI color support.
- JSON output support.
- Metric and imperial units support.
