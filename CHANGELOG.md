# Changelog

All notable changes to this project will be documented in this file.
See [Keep a Changelog](https://keepachangelog.com/) for details.
This project adheres to [Semantic Versioning](https://semver.org/).

## [Upcoming]

### Added

- 🎌 Multilanguage support. Added translations for Russian and Spanish languages. More languages to come, contributions
  welcome!

### Changed

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
