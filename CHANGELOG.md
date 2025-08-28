# Changelog

All notable changes to this project will be documented in this file.
See [Keep a Changelog](https://keepachangelog.com/) for details.
This project adheres to [Semantic Versioning](https://semver.org/).

## [Upcoming]

### Changed

- Improved code quality and maintainability by using `enum_dispatch` for weather providers.

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
