use crate::config::Config;
use crate::errors::RustormyError;
use crate::models::{Location, Units, Weather, WeatherConditionIcon};
use crate::weather::{GetWeather, LookUpCity};
use reqwest::blocking::Client;

const GEOCODING_API_URL: &str = "https://api.weatherbit.io/v2.0/geocode";
const WEATHER_API_URL: &str = "https://api.weatherbit.io/v2.0/current";

#[derive(Debug, Default)]
pub struct WeatherBit {}

#[derive(Debug, serde::Serialize)]
struct GeocodingApiRequest<'a> {
    city: &'a str,
    key: &'a str,
}

impl<'a> GeocodingApiRequest<'a> {
    pub fn new(config: &'a Config) -> Result<Self, RustormyError> {
        let city = config.city().ok_or(RustormyError::NoLocationProvided)?;
        Ok(Self {
            city,
            key: &config.api_keys().weather_bit,
        })
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum GeocodingApiResponse {
    Ok(GeocodingApiResponseData),
    Err { error: String },
}

#[derive(Debug, serde::Deserialize)]
struct GeocodingApiResponseData {
    name: String,
    lat: f64,
    lon: f64,
    // timezone: String,
}

impl GeocodingApiResponseData {
    pub fn into_location(self) -> Location {
        Location {
            name: self.name,
            latitude: self.lat,
            longitude: self.lon,
        }
    }
}

impl LookUpCity for WeatherBit {
    fn lookup_city(
        &self,
        client: &reqwest::blocking::Client,
        config: &Config,
    ) -> Result<Location, RustormyError> {
        let request = GeocodingApiRequest::new(config)?;
        let response = client.get(GEOCODING_API_URL).query(&request).send()?;
        let data: GeocodingApiResponse = response.json()?;
        match data {
            GeocodingApiResponse::Err { error } => Err(RustormyError::ApiReturnedError(error)),
            GeocodingApiResponse::Ok(data) => Ok(data.into_location()),
        }
    }
}

#[derive(Debug, serde::Serialize)]
struct WeatherAPIRequest<'a> {
    lat: f64,
    lon: f64,
    key: &'a str,
    lang: &'a str,
    units: &'a str,
}

impl<'a> WeatherAPIRequest<'a> {
    pub fn new(location: &Location, config: &'a Config) -> Self {
        let units = match config.units() {
            Units::Metric => "M",
            Units::Imperial => "I",
        };
        Self {
            lat: location.latitude,
            lon: location.longitude,
            key: &config.api_keys().weather_bit,
            lang: config.language().code(),
            units,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum WeatherApiResponse {
    Ok { count: u8, data: Vec<WeatherData> },
    Err { error: String },
}

#[derive(Debug, serde::Deserialize)]
struct WeatherData {
    /// Apparent Temperature
    app_temp: f64,
    // TODO: Air Quality Index
    // aqi: u32,
    city_name: String,
    // clouds: u8,
    // country_code: String,
    // datetime: String,
    dewpt: f64,
    // dhi: f64,
    // dni: f64,
    // elev_angle: f64,
    // ghi: f64,
    // gust: f64,
    // h_angle: f64,
    // lat: f64,
    // lon: f64,
    // ob_time: String,
    // pod: String,
    precip: f64,
    pres: f64,
    rh: u8,
    // slp: f64,
    // snow: f64,
    // solar_rad: f64,
    // sources: Vec<String>,
    // state_code: String,
    // station: String,
    // sunrise: String,
    // sunset: String,
    temp: f64,
    // timezone: String,
    // ts: u64,
    uv: f64,
    // vis: f64,
    weather: WeatherDescription,
    // wind_cdir: String,
    // wind_cdir_full: String,
    wind_dir: u16,
    wind_spd: f64,
}

impl WeatherData {
    fn uv_index(&self) -> u8 {
        self.uv.round() as u8
    }

    fn pressure(&self) -> u32 {
        self.pres.round() as u32
    }

    pub fn into_weather(self) -> Weather {
        Weather {
            temperature: self.temp,
            feels_like: self.app_temp,
            humidity: self.rh,
            dew_point: self.dewpt,
            precipitation: self.precip,
            pressure: self.pressure(),
            wind_speed: self.wind_spd,
            wind_direction: self.wind_dir,
            uv_index: Some(self.uv_index()),
            icon: self.weather.icon(),
            description: self.weather.description,
            location_name: self.city_name,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct WeatherDescription {
    description: String,
    // icon: String,
    code: u32,
}

impl WeatherDescription {
    fn icon(&self) -> WeatherConditionIcon {
        match self.code {
            200..=232 => WeatherConditionIcon::Thunderstorm,
            300..=321 | 500 | 520 => WeatherConditionIcon::LightShowers,
            500..=531 => WeatherConditionIcon::HeavyShowers,
            600 | 612 | 615 | 620 => WeatherConditionIcon::LightSnow,
            601..=622 => WeatherConditionIcon::HeavySnow,
            701..=781 => WeatherConditionIcon::Fog,
            800 => WeatherConditionIcon::Clear,
            801 | 802 => WeatherConditionIcon::PartlyCloudy,
            803 | 804 => WeatherConditionIcon::Cloudy,
            _ => WeatherConditionIcon::Unknown,
        }
    }
}

impl GetWeather for WeatherBit {
    fn get_weather(&self, client: &Client, config: &Config) -> Result<Weather, RustormyError> {
        let location = self.get_location(client, config)?;
        let request = WeatherAPIRequest::new(&location, config);
        let response = client.get(WEATHER_API_URL).query(&request).send()?;
        let data: WeatherApiResponse = response.json()?;
        match data {
            WeatherApiResponse::Err { error } => Err(RustormyError::ApiReturnedError(error)),
            WeatherApiResponse::Ok { count, data } => {
                if count == 0 || data.is_empty() {
                    return Err(RustormyError::CityNotFound(location.name));
                }
                Ok(data.into_iter().next().unwrap().into_weather())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_city_lookup_response() {
        let json_data = r#"
        {
            "id": 2643743,
            "name": "London",
            "lat": 51.5074,
            "lon": -0.1278,
            "timezone": "Europe/London"
        }
        "#;

        let response: GeocodingApiResponseData = serde_json::from_str(json_data).unwrap();
        let location = response.into_location();

        assert_eq!(location.name, "London");
        assert_eq!(location.latitude, 51.5074);
        assert_eq!(location.longitude, -0.1278);
    }

    #[test]
    fn test_parse_weather_api_response() {
        let json_data = r#"
        {
            "count": 1,
            "data": [
                {
                    "app_temp": 15.0,
                    "aqi": 42,
                    "city_name": "London",
                    "dewpt": 10.0,
                    "precip": 0.0,
                    "pres": 1015.0,
                    "rh": 70,
                    "temp": 16.0,
                    "uv": 5.0,
                    "weather": {
                        "description": "Partly cloudy",
                        "icon": "c02d",
                        "code": 802
                    },
                    "wind_dir": 180,
                    "wind_spd": 3.5
                }
            ]
        }
        "#;

        let response: WeatherApiResponse = serde_json::from_str(json_data).unwrap();
        match response {
            WeatherApiResponse::Ok { count, data } => {
                assert_eq!(count, 1);
                let weather = data.into_iter().next().unwrap().into_weather();
                assert_eq!(weather.temperature, 16.0);
                assert_eq!(weather.feels_like, 15.0);
                assert_eq!(weather.humidity, 70);
                assert_eq!(weather.dew_point, 10.0);
                assert_eq!(weather.precipitation, 0.0);
                assert_eq!(weather.pressure, 1015);
                assert_eq!(weather.wind_speed, 3.5);
                assert_eq!(weather.wind_direction, 180);
                assert_eq!(weather.uv_index, Some(5));
                assert_eq!(weather.icon, WeatherConditionIcon::PartlyCloudy);
                assert_eq!(weather.description, "Partly cloudy");
                assert_eq!(weather.location_name, "London");
            }
            WeatherApiResponse::Err { error } => panic!("Unexpected error: {error:?}"),
        }
    }

    #[test]
    fn test_parse_geocoding_api_error_response() {
        let json_data = r#"
        {
            "error": "Invalid API key"
        }
        "#;

        let response: GeocodingApiResponse = serde_json::from_str(json_data).unwrap();
        match response {
            GeocodingApiResponse::Err { error } => {
                assert_eq!(error, "Invalid API key");
            }
            GeocodingApiResponse::Ok(_) => panic!("Expected error response"),
        }
    }

    #[test]
    fn test_parse_weather_api_error_response() {
        let json_data = r#"
        {
            "error": "Invalid API key"
        }
        "#;

        let response: WeatherApiResponse = serde_json::from_str(json_data).unwrap();
        match response {
            WeatherApiResponse::Err { error } => {
                assert_eq!(error, "Invalid API key");
            }
            WeatherApiResponse::Ok { .. } => panic!("Expected error response"),
        }
    }
}
