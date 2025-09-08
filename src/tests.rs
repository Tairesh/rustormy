#![cfg(test)]
use crate::models::Location;
use crate::weather::LookUpCity;
use crate::{
    cli::Cli,
    config::Config,
    errors::RustormyError,
    models::{Units, Weather, WeatherConditionIcon},
    tools,
    weather::GetWeather,
};
use clap::Parser;
use reqwest::blocking::Client;

struct TestProvider;

impl TestProvider {
    pub fn new() -> Self {
        Self
    }

    fn mock_weather(config: &Config, location: Location) -> Weather {
        let temperature = if config.units() == Units::Metric {
            20.0
        } else {
            68.0
        };
        let humidity = 65;
        Weather {
            temperature,
            feels_like: 19.5,
            humidity,
            dew_point: tools::dew_point(temperature, humidity as f64, config.units()),
            precipitation: 0.0,
            pressure: 1013,
            wind_speed: 5.0,
            wind_direction: 180,
            uv_index: None,
            description: "Clear sky".to_string(),
            icon: WeatherConditionIcon::Clear,
            location_name: location.name,
        }
    }
}

impl LookUpCity for TestProvider {
    fn lookup_city(&self, _client: &Client, config: &Config) -> Result<Location, RustormyError> {
        let city = config.city().ok_or(RustormyError::NoLocationProvided)?;
        if city == "NonexistentCity" {
            return Err(RustormyError::CityNotFound(city.to_string()));
        }
        if city == "" {
            return Err(RustormyError::CityNotFound(city.to_string()));
        }
        Ok(Location {
            name: city.to_string(),
            latitude: 51.5074,
            longitude: -0.1278,
        })
    }
}

impl GetWeather for TestProvider {
    fn get_weather(&self, client: &Client, config: &Config) -> Result<Weather, RustormyError> {
        // Validate input parameters just like real providers
        let location = self.get_location(client, config)?;

        Ok(Self::mock_weather(config, location))
    }
}

#[test]
fn test_valid_city_lookup() {
    let client = Client::new();
    let config = Config::new(Cli::parse_from(&["rustormy", "-c", "Test City"])).unwrap();
    let provider = TestProvider::new();

    let result = provider.get_weather(&client, &config);
    assert!(result.is_ok());

    let weather = result.unwrap();
    assert_eq!(weather.temperature, 20.0);
    assert_eq!(weather.location_name, "Test City".to_string());
}

#[test]
fn test_nonexistent_city() {
    let client = Client::new();
    let config = Config::new(Cli::parse_from(&["rustormy", "-c", "NonexistentCity"])).unwrap();
    let provider = TestProvider::new();

    let result = provider.get_weather(&client, &config);
    assert!(matches!(
        result,
        Err(RustormyError::CityNotFound(city)) if city == "NonexistentCity"
    ));
}

#[test]
fn test_valid_coordinates() {
    let client = Client::new();
    let config = Config::new(Cli::parse_from(&[
        "rustormy",
        "-y",
        "51.5074",
        "-x=-0.1278",
    ]))
    .unwrap();
    let provider = TestProvider::new();

    let result = provider.get_weather(&client, &config);
    assert!(result.is_ok());

    let weather = result.unwrap();
    assert_eq!(weather.temperature, 20.0);
    assert_eq!(weather.humidity, 65);
}

#[test]
fn test_invalid_coordinates() {
    let config = Config::new(Cli::parse_from(&["rustormy", "-y", "91.0", "-x", "0.0"]));
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
    let client = Client::new();
    let config = Config::default();
    let provider = TestProvider::new();

    let result = provider.get_weather(&client, &config);
    assert!(
        matches!(result, Err(RustormyError::NoLocationProvided)),
        "No location provided should result in an error, got {:?}",
        result
    );
}

#[test]
fn test_empty_city() {
    let client = Client::new();
    let config = Config::new(Cli::parse_from(&["rustormy", "-c", ""])).unwrap();
    let provider = TestProvider::new();

    let result = provider.get_weather(&client, &config);
    assert!(matches!(result, Err(RustormyError::NoLocationProvided)));
}

#[test]
fn test_different_units() {
    let client = Client::new();
    let config_metric = Config::new(Cli::parse_from(&["rustormy", "-c", "Test"])).unwrap();
    let config_imperial = Config::new(Cli::parse_from(&[
        "rustormy", "-c", "London", "-u", "imperial",
    ]))
    .unwrap();
    let provider = TestProvider::new();

    let weather_metric = provider.get_weather(&client, &config_metric).unwrap();
    let weather_imperial = provider.get_weather(&client, &config_imperial).unwrap();

    assert_eq!(weather_metric.temperature, 20.0);
    assert_eq!(weather_imperial.temperature, 68.0);
}
