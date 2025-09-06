use crate::config::Config;
use crate::errors::RustormyError;
use crate::models::{Language, Units, Weather, WeatherConditionIcon};
use crate::weather::GetWeather;
use reqwest::blocking::Client;

const WWO_API_URL: &str = "https://api.worldweatheronline.com/premium/v1/weather.ashx";

#[derive(Debug, serde::Serialize)]
struct WwoRequestParams<'a> {
    q: String,
    key: &'a str,
    format: &'a str,
    lang: &'a str,
    fx: &'a str,
    mca: &'a str,
}

impl<'a> WwoRequestParams<'a> {
    pub fn new(config: &'a Config) -> Result<Self, RustormyError> {
        let q = match (config.coordinates(), config.city()) {
            (Some((lat, lon)), _) => format!("{lat},{lon}"),
            (None, Some(city)) => city.to_string(),
            (None, None) => return Err(RustormyError::NoLocationProvided),
        };
        let key = config.api_key_wwo().ok_or(RustormyError::MissingApiKey)?;

        Ok(Self {
            q,
            key,
            format: "json",
            lang: config.language().code(),
            fx: "no",
            mca: "no",
        })
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
    request: Vec<WwoRequestInfo>,
    current_condition: Vec<WwoCurrentCondition>,
}

impl WwoWeatherData {
    fn into_weather(self, config: &Config) -> Result<Weather, RustormyError> {
        let location_name = self.location_name()?.to_string();
        let condition = self.current_condition.into_iter().next().ok_or_else(|| {
            RustormyError::ApiReturnedError("No current condition data".to_string())
        })?;

        Ok(Weather {
            temperature: condition.temperature(config.units())?,
            feels_like: condition.feels_like(config.units())?,
            humidity: condition.humidity()?,
            precipitation: condition.precipitation(config.units())?,
            pressure: condition.pressure()?,
            wind_speed: condition.wind_speed(config.units())?,
            wind_direction: condition.wind_direction()?,
            uv_index: condition.uv_index()?,
            description: condition.desc(config.language())?.to_string(),
            icon: condition.icon()?,
            location_name,
        })
    }

    fn location_name(&self) -> Result<&str, RustormyError> {
        self.request
            .first()
            .map(|r| r.query.as_str())
            .ok_or_else(|| {
                RustormyError::ApiReturnedError("No location name available".to_string())
            })
    }
}

#[derive(Debug, serde::Deserialize)]
struct WwoRequestInfo {
    #[allow(dead_code)]
    #[serde(rename = "type")]
    req_type: String,
    query: String,
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
        }
        .ok_or_else(|| {
            RustormyError::ApiReturnedError(
                "No description available in the requested language".to_string(),
            )
        })?;

        Ok(&desc.value)
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

        // Convert km/h to m/s for Metric
        Ok(if units == Units::Metric {
            value / 3.6
        } else {
            value
        })
    }

    fn humidity(&self) -> Result<u8, RustormyError> {
        self.humidity
            .parse::<u8>()
            .map_err(|e| RustormyError::ApiReturnedError(format!("Invalid humidity value: {e:?}")))
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

    fn uv_index(&self) -> Result<Option<u8>, RustormyError> {
        let uv_index = self.uv_index.parse::<u8>().map_err(|e| {
            RustormyError::ApiReturnedError(format!("Invalid UV index value: {e:?}"))
        })?;
        Ok(Some(uv_index))
    }

    fn icon(&self) -> Result<WeatherConditionIcon, RustormyError> {
        let code = self.weather_code.parse::<u16>().map_err(|e| {
            RustormyError::ApiReturnedError(format!("Invalid weather code value: {e:?}"))
        })?;
        let icon = match code {
            113 => WeatherConditionIcon::Sunny,
            116 => WeatherConditionIcon::PartlyCloudy,
            119 | 122 => WeatherConditionIcon::Cloudy,
            143 | 248 | 260 => WeatherConditionIcon::Fog,
            263 | 266 | 281 | 284 => WeatherConditionIcon::LightShowers,
            176 | 293 | 296 | 299 | 302 | 308 | 305 | 353 | 356 | 359 | 311 | 314 | 317 | 320
            | 362 | 365 | 374 | 377 => WeatherConditionIcon::HeavyShowers,
            179 | 323 | 326 | 329 | 332 | 335 | 338 | 368 | 371 | 227 | 230 => {
                WeatherConditionIcon::HeavySnow
            }
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
        let params = WwoRequestParams::new(config)?;
        let response = client.get(WWO_API_URL).query(&params).send()?;
        let response: WwoResponse = response.json()?;

        match response {
            WwoResponse::Ok { data } => data.into_weather(config),
            WwoResponse::Err { data } => Err(RustormyError::ApiReturnedError(data.get_message())),
        }
    }
}
