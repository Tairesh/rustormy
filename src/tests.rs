#![cfg(test)]
use crate::models::Location;
use crate::{
    cli::Cli,
    config::Config,
    errors::RustormyError,
    models::{Units, Weather, WeatherConditionIcon},
    weather::GetWeather,
};
use clap::Parser;

struct TestProvider;

impl TestProvider {
    pub fn new() -> Self {
        Self
    }

    fn mock_weather(config: &Config) -> Weather {
        Weather {
            temperature: if config.units() == Units::Metric {
                20.0
            } else {
                68.0
            },
            feels_like: 19.5,
            humidity: 65,
            precipitation: 0.0,
            pressure: 1013,
            wind_speed: 5.0,
            wind_direction: 180,
            description: "Clear sky".to_string(),
            icon: WeatherConditionIcon::Sunny,
            location_name: "Test City".to_string(),
        }
    }
}

impl GetWeather for TestProvider {
    fn get_weather(&self, config: &Config) -> Result<Weather, RustormyError> {
        // Validate input parameters just like real providers
        if let Some(city) = config.city() {
            self.lookup_city(city, config)?;
        } else if config.coordinates().is_none() {
            return Err(RustormyError::NoLocationProvided);
        }

        Ok(Self::mock_weather(config))
    }

    fn lookup_city(&self, city: &str, _config: &Config) -> Result<Location, RustormyError> {
        if city.is_empty() {
            return Err(RustormyError::CityNotFound("".to_string()));
        }
        if city == "NonexistentCity" {
            return Err(RustormyError::CityNotFound(city.to_string()));
        }

        Ok(Location {
            name: "Test City".to_string(),
            latitude: 51.5074,
            longitude: -0.1278,
        })
    }
}

#[test]
fn test_valid_city_lookup() {
    let config = Config::new(&Cli::parse_from(&["rustormy", "-c", "Test"])).unwrap();
    let provider = TestProvider::new();

    let result = provider.get_weather(&config);
    assert!(result.is_ok());

    let weather = result.unwrap();
    assert_eq!(weather.temperature, 20.0);
    assert_eq!(weather.location_name, "Test City".to_string());
}

#[test]
fn test_nonexistent_city() {
    let config = Config::new(&Cli::parse_from(&["rustormy", "-c", "NonexistentCity"])).unwrap();
    let provider = TestProvider::new();

    let result = provider.get_weather(&config);
    assert!(matches!(
        result,
        Err(RustormyError::CityNotFound(city)) if city == "NonexistentCity"
    ));
}

#[test]
fn test_valid_coordinates() {
    let config = Config::new(&Cli::parse_from(&[
        "rustormy",
        "-y",
        "51.5074",
        "-x=-0.1278",
    ]))
    .unwrap();
    let provider = TestProvider::new();

    let result = provider.get_weather(&config);
    assert!(result.is_ok());

    let weather = result.unwrap();
    assert_eq!(weather.temperature, 20.0);
    assert_eq!(weather.humidity, 65);
}

#[test]
fn test_invalid_coordinates() {
    let config = Config::new(&Cli::parse_from(&["rustormy", "-y", "91.0", "-x", "0.0"]));
    assert!(matches!(
        config,
        Err(RustormyError::InvalidCoordinates {
            lat: 91.0,
            lon: 0.0
        })
    ));
}

#[test]
fn test_no_location_provided() {
    let config = Config::default();
    let provider = TestProvider::new();

    let result = provider.get_weather(&config);
    assert!(
        matches!(result, Err(RustormyError::NoLocationProvided)),
        "No location provided should result in an error, got {:?}",
        result
    );
}

#[test]
fn test_empty_city() {
    let config = Config::new(&Cli::parse_from(&["rustormy", "-c", ""])).unwrap();
    let provider = TestProvider::new();

    let result = provider.get_weather(&config);
    assert!(matches!(
        result,
        Err(RustormyError::CityNotFound(city)) if city.is_empty()
    ));
}

#[test]
fn test_different_units() {
    let config_metric = Config::new(&Cli::parse_from(&["rustormy", "-c", "Test"])).unwrap();
    let config_imperial = Config::new(&Cli::parse_from(&[
        "rustormy", "-c", "London", "-u", "imperial",
    ]))
    .unwrap();
    let provider = TestProvider::new();

    let weather_metric = provider.get_weather(&config_metric).unwrap();
    let weather_imperial = provider.get_weather(&config_imperial).unwrap();

    assert_eq!(weather_metric.temperature, 20.0);
    assert_eq!(weather_imperial.temperature, 68.0);
}
