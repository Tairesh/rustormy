use crate::config::Config;
use crate::errors::RustormyError;
use crate::models::{Language, Location, Units, Weather, WeatherConditionIcon};
use crate::weather::{GetWeather, tools};
use reqwest::blocking::Client;

const WWO_API_URL: &str = "https://api.worldweatheronline.com/premium/v1/weather.ashx";

#[derive(Debug, serde::Serialize)]
struct WwoRequestParams<'a> {
    q: String,
    key: &'a str,
    lang: &'a str,
    format: &'a str,
    fx: &'a str,
    mca: &'a str,
    includelocation: &'a str,
}

impl<'a> WwoRequestParams<'a> {
    pub fn new(config: &'a Config) -> Self {
        let q = config.location_name();
        let lang = config.language().code();

        Self {
            q,
            key: &config.api_keys().world_weather_online,
            lang,
            format: "json",
            fx: "no",
            mca: "no",
            includelocation: "yes",
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum WwoResponse {
    Ok { data: WwoWeatherData },
    Err { data: WwoError },
}

#[derive(Debug, serde::Deserialize)]
struct WwoWeatherData {
    current_condition: Vec<WwoCurrentCondition>,
    #[serde(default)]
    nearest_area: Vec<WwoNearestArea>,
}

impl WwoWeatherData {
    fn into_weather(self, config: &Config) -> Result<Weather, RustormyError> {
        let location = self.location()?;
        let condition = self.current_condition.into_iter().next().ok_or_else(|| {
            RustormyError::ApiReturnedError("No current condition data".to_string())
        })?;

        Ok(Weather {
            temperature: condition.temperature(config.units())?,
            feels_like: condition.feels_like(config.units())?,
            humidity: condition.humidity()?,
            dew_point: condition.dew_point(config.units())?,
            precipitation: condition.precipitation(config.units())?,
            pressure: condition.pressure()?,
            wind_speed: condition.wind_speed(config.units())?,
            wind_direction: condition.wind_direction()?,
            uv_index: condition.uv_index()?,
            is_day: None,
            description: condition.desc(config.language())?.to_string(),
            icon: condition.icon()?,
            location,
        })
    }

    fn location(&self) -> Result<Location, RustormyError> {
        self.nearest_area
            .first()
            .and_then(|a| {
                let lat = a.latitude.parse::<f64>().ok()?;
                let lon = a.longitude.parse::<f64>().ok()?;
                let city = a.area_name.first()?.value.as_str();
                let country = a.country.first().map_or("", |c| c.value.as_str());
                let name = if country.is_empty() {
                    city.to_string()
                } else {
                    tools::shorten_location_name(format!("{city}, {country}"))
                };
                Some(Location::new(name, lat, lon))
            })
            .ok_or_else(|| RustormyError::ApiReturnedError("No location data".to_string()))
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct WwoNearestArea {
    area_name: Vec<WwoWeatherDesc>,
    country: Vec<WwoWeatherDesc>,
    latitude: String,
    longitude: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct WwoCurrentCondition {
    #[serde(rename = "temp_C")]
    temp_c: String,
    #[serde(rename = "temp_F")]
    temp_f: String,
    weather_code: String,
    weather_desc: Vec<WwoWeatherDesc>,
    #[serde(default, rename = "lang_ru")]
    lang_ru: Vec<WwoWeatherDesc>,
    #[serde(default, rename = "lang_es")]
    lang_es: Vec<WwoWeatherDesc>,
    #[serde(default, rename = "lang_ko")]
    lang_ko: Vec<WwoWeatherDesc>,
    windspeed_miles: String,
    windspeed_kmph: String,
    winddir_degree: String,
    #[serde(rename = "precipMM")]
    precip_mm: String,
    #[serde(rename = "precipInches")]
    precip_inches: String,
    humidity: String,
    pressure: String,
    #[serde(rename = "FeelsLikeC")]
    feels_like_c: String,
    #[serde(rename = "FeelsLikeF")]
    feels_like_f: String,
    uv_index: String,
}

impl WwoCurrentCondition {
    fn desc(&self, language: Language) -> Result<&str, RustormyError> {
        let desc = match language {
            Language::English => self.weather_desc.first(),
            Language::Russian => self.lang_ru.first(),
            Language::Spanish => self.lang_es.first(),
            Language::Korean => self.lang_ko.first(),
        }
        .ok_or_else(|| {
            RustormyError::ApiReturnedError(
                "No description available in the requested language".to_string(),
            )
        })?;

        Ok(desc.value.trim())
    }

    fn temperature(&self, units: Units) -> Result<f64, RustormyError> {
        let value = match units {
            Units::Metric => &self.temp_c,
            Units::Imperial => &self.temp_f,
        };

        value.parse::<f64>().map_err(|e| {
            RustormyError::ApiReturnedError(format!("Invalid temperature value: {e:?}"))
        })
    }

    fn feels_like(&self, units: Units) -> Result<f64, RustormyError> {
        let value = match units {
            Units::Metric => &self.feels_like_c,
            Units::Imperial => &self.feels_like_f,
        };

        value.parse::<f64>().map_err(|e| {
            RustormyError::ApiReturnedError(format!("Invalid feels like value: {e:?}"))
        })
    }

    fn wind_speed(&self, units: Units) -> Result<f64, RustormyError> {
        let value = match units {
            Units::Metric => &self.windspeed_kmph,
            Units::Imperial => &self.windspeed_miles,
        };

        let value = value.parse::<f64>().map_err(|e| {
            RustormyError::ApiReturnedError(format!("Invalid wind speed value: {e:?}"))
        })?;

        Ok(if units == Units::Metric {
            tools::kph_to_ms(value)
        } else {
            (value * 10.0).round() / 10.0
        })
    }

    fn humidity(&self) -> Result<u8, RustormyError> {
        self.humidity
            .parse::<u8>()
            .map_err(|e| RustormyError::ApiReturnedError(format!("Invalid humidity value: {e:?}")))
    }

    fn dew_point(&self, units: Units) -> Result<f64, RustormyError> {
        let t = self.temperature(units)?;
        let h = self.humidity()?.into();
        Ok(tools::dew_point(t, h, units))
    }

    fn precipitation(&self, units: Units) -> Result<f64, RustormyError> {
        let value = if units == Units::Metric {
            &self.precip_mm
        } else {
            &self.precip_inches
        };

        value.parse::<f64>().map_err(|e| {
            RustormyError::ApiReturnedError(format!("Invalid precipitation value: {e:?}"))
        })
    }

    fn pressure(&self) -> Result<u32, RustormyError> {
        self.pressure
            .parse::<u32>()
            .map_err(|e| RustormyError::ApiReturnedError(format!("Invalid pressure value: {e:?}")))
    }

    fn wind_direction(&self) -> Result<u16, RustormyError> {
        self.winddir_degree.parse::<u16>().map_err(|e| {
            RustormyError::ApiReturnedError(format!("Invalid wind direction value: {e:?}"))
        })
    }

    fn uv_index(&self) -> Result<Option<f64>, RustormyError> {
        let uv_index = self.uv_index.parse::<f64>().map_err(|e| {
            RustormyError::ApiReturnedError(format!("Invalid UV index value: {e:?}"))
        })?;
        Ok(Some(uv_index))
    }

    fn icon(&self) -> Result<WeatherConditionIcon, RustormyError> {
        let code = self.weather_code.parse::<u16>().map_err(|e| {
            RustormyError::ApiReturnedError(format!("Invalid weather code value: {e:?}"))
        })?;
        let icon = match code {
            // Clear/Sunny
            113 => WeatherConditionIcon::Clear,
            // Partly Cloudy
            116 => WeatherConditionIcon::PartlyCloudy,
            // Cloudy/Overcast
            119 | 122 => WeatherConditionIcon::Cloudy,
            // Mist/Fog
            143 | 248 | 260 => WeatherConditionIcon::Fog,
            // Light Rain Showers
            176 | 263 | 266 | 281 | 284 => WeatherConditionIcon::LightShowers,
            // Heavy Rain Showers
            293 | 296 | 299 | 302 | 308 | 305 | 353 | 356 | 359 | 311 | 314 | 317 | 320 | 362
            | 365 | 374 | 377 => WeatherConditionIcon::HeavyShowers,
            // Light Snow/Blowing Snow
            179 | 227 | 326 | 368 | 323 => WeatherConditionIcon::LightSnow,
            // Heavy Snow/Snow Showers
            230 | 329 | 332 | 338 | 335 | 371 => WeatherConditionIcon::HeavySnow,
            // Thunderstorm
            200 | 386 | 389 | 392 | 395 => WeatherConditionIcon::Thunderstorm,
            _ => WeatherConditionIcon::Unknown,
        };
        Ok(icon)
    }
}

#[derive(Debug, serde::Deserialize)]
struct WwoWeatherDesc {
    value: String,
}

#[derive(Debug, serde::Deserialize)]
struct WwoError {
    error: Vec<WwoErrorMessage>,
}

impl WwoError {
    fn get_message(&self) -> String {
        self.error
            .iter()
            .map(|e| e.msg.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[derive(Debug, serde::Deserialize)]
struct WwoErrorMessage {
    msg: String,
}

#[derive(Debug, Default)]
pub struct WorldWeatherOnline {}

impl GetWeather for WorldWeatherOnline {
    fn get_weather(&self, client: &Client, config: &Config) -> Result<Weather, RustormyError> {
        let params = WwoRequestParams::new(config);
        let response = client.get(WWO_API_URL).query(&params).send()?;
        let response: WwoResponse = response.json()?;

        match response {
            WwoResponse::Ok { data } => data.into_weather(config),
            WwoResponse::Err { data } => Err(RustormyError::ApiReturnedError(data.get_message())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    const TEST_API_RESPONSE: &str = include_str!("../../tests/data/wwo_response.json");

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_parse_wwo_response() {
        let response: WwoResponse =
            serde_json::from_str(TEST_API_RESPONSE).expect("Failed to parse JSON");
        let WwoResponse::Ok { data } = response else {
            panic!("Expected Ok variant");
        };

        let weather = data
            .into_weather(&Config::default())
            .expect("into_weather should succeed");

        assert_eq!(weather.location.name, "London, United Kingdom");
        assert_eq!(weather.location.latitude, 51.517);
        assert_eq!(weather.location.longitude, -0.106);
        assert_eq!(weather.is_day, None);
        assert_eq!(weather.temperature, 14.0);
        assert_eq!(weather.humidity, 63);
        assert_eq!(weather.icon, WeatherConditionIcon::Cloudy);
        assert_eq!(weather.description, "Cloudy");
    }
}
