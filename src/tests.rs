#![cfg(test)]
use crate::{
    cli::Cli,
    config::Config,
    display::icons::WeatherCondition,
    errors::RustormyError,
    models::{Units, Weather},
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
            condition: WeatherCondition::Sunny,
            city: Some("Test City".to_string()),
        }
    }
}

#[async_trait::async_trait]
impl GetWeather for TestProvider {
    async fn get_weather(&self, config: &Config) -> Result<Weather, RustormyError> {
        // Validate input parameters just like real providers
        if let Some(city) = config.city() {
            if city.is_empty() {
                return Err(RustormyError::CityNotFound("".to_string()));
            }
            if city == "NonexistentCity" {
                return Err(RustormyError::CityNotFound(city.to_string()));
            }
        } else if config.coordinates().is_none() {
            return Err(RustormyError::Other(anyhow::anyhow!(
                "Neither city nor coordinates provided"
            )));
        }

        Ok(Self::mock_weather(config))
    }
}

#[tokio::test]
async fn test_valid_city_lookup() {
    let config = Config::new(&Cli::parse_from(&["rustormy", "-c", "Test"])).unwrap();
    let provider = TestProvider::new();

    let result = provider.get_weather(&config).await;
    assert!(result.is_ok());

    let weather = result.unwrap();
    assert_eq!(weather.temperature, 20.0);
    assert_eq!(weather.city, Some("Test City".to_string()));
}

#[tokio::test]
async fn test_nonexistent_city() {
    let config = Config::new(&Cli::parse_from(&["rustormy", "-c", "NonexistentCity"])).unwrap();
    let provider = TestProvider::new();

    let result = provider.get_weather(&config).await;
    assert!(matches!(
        result,
        Err(RustormyError::CityNotFound(city)) if city == "NonexistentCity"
    ));
}

#[tokio::test]
async fn test_valid_coordinates() {
    let config = Config::new(&Cli::parse_from(&[
        "rustormy",
        "-y",
        "51.5074",
        "-x=-0.1278",
    ]))
    .unwrap();
    let provider = TestProvider::new();

    let result = provider.get_weather(&config).await;
    assert!(result.is_ok());

    let weather = result.unwrap();
    assert_eq!(weather.temperature, 20.0);
    assert_eq!(weather.humidity, 65);
}

#[tokio::test]
async fn test_invalid_coordinates() {
    let config = Config::new(&Cli::parse_from(&["rustormy", "-y", "91.0", "-x", "0.0"]));
    assert!(matches!(
        config,
        Err(RustormyError::InvalidCoordinates {
            lat: 91.0,
            lon: 0.0
        })
    ));
}

#[tokio::test]
async fn test_no_location_provided() {
    let config = Config::default();
    let provider = TestProvider::new();

    let result = provider.get_weather(&config).await;
    assert!(
        matches!(result, Err(RustormyError::Other(_))),
        "No location provided should result in an error, got {:?}",
        result
    );
}

#[tokio::test]
async fn test_empty_city() {
    let config = Config::new(&Cli::parse_from(&["rustormy", "-c", ""])).unwrap();
    let provider = TestProvider::new();

    let result = provider.get_weather(&config).await;
    assert!(matches!(
        result,
        Err(RustormyError::CityNotFound(city)) if city.is_empty()
    ));
}

#[tokio::test]
async fn test_different_units() {
    let config_metric = Config::new(&Cli::parse_from(&["rustormy", "-c", "Test"])).unwrap();
    let config_imperial = Config::new(&Cli::parse_from(&[
        "rustormy", "-c", "London", "-u", "imperial",
    ]))
    .unwrap();
    let provider = TestProvider::new();

    let weather_metric = provider.get_weather(&config_metric).await.unwrap();
    let weather_imperial = provider.get_weather(&config_imperial).await.unwrap();

    assert_eq!(weather_metric.temperature, 20.0);
    assert_eq!(weather_imperial.temperature, 68.0);
}
