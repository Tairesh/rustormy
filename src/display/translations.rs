use crate::models::Language;
use std::collections::HashMap;
use std::sync::LazyLock;

macro_rules! translations {
    ($($key:expr => {
        $([$lang:expr] => $text:expr),* $(,)?
    }),* $(,)?) => {{
        LazyLock::new(|| {
            let mut result = HashMap::new();
            $(
                $(
                    result.entry($lang)
                        .or_insert_with(HashMap::new)
                        .insert($key, $text);
                )*
            )*
            result
        })
    }};
}

static TRANSLATIONS: LazyLock<HashMap<&'static str, HashMap<&'static str, &'static str>>> = translations! {
    "Location" => {
        ["en"] => "Location",
        ["ru"] => "Локация",
        ["es"] => "Ubicación",
    },
    "Temperature" => {
        ["en"] => "Temperature",
        ["ru"] => "Температура",
        ["es"] => "Temperatura",
    },
    "Condition" => {
        ["en"] => "Condition",
        ["ru"] => "Погода",
        ["es"] => "Condición",
    },
    "Wind" => {
        ["en"] => "Wind",
        ["ru"] => "Ветер",
        ["es"] => "Viento",
    },
    "Humidity" => {
        ["en"] => "Humidity",
        ["ru"] => "Влажность",
        ["es"] => "Humedad",
    },
    "Precipitation" => {
        ["en"] => "Precip",
        ["ru"] => "Осадки",
        ["es"] => "Lluvias",
    },
    "Pressure" => {
        ["en"] => "Pressure",
        ["ru"] => "Давление",
        ["es"] => "Presión",
    },
    "UV index" => {
        ["en"] => "UV index",
        ["ru"] => "УФ индекс",
        ["es"] => "Índice UV",
    },
    "dew point" => {
        ["en"] => "dew point",
        ["ru"] => "точка росы",
        ["es"] => "punto de rocío",
    },
    // Weather conditions
    "Clear sky" => {
        ["en"] => "Clear sky",
        ["ru"] => "Ясное небо",
        ["es"] => "Cielo despejado",
    },
    "Mainly clear" => {
        ["en"] => "Mainly clear",
        ["ru"] => "Преимущественно ясно",
        ["es"] => "Mayormente despejado",
    },
    "Partly cloudy" => {
        ["en"] => "Partly cloudy",
        ["ru"] => "Переменная облачность",
        ["es"] => "Parcialmente nublado",
    },
    "Overcast" => {
        ["en"] => "Overcast",
        ["ru"] => "Пасмурно",
        ["es"] => "Nublado",
    },
    "Fog" => {
        ["en"] => "Fog",
        ["ru"] => "Туман",
        ["es"] => "Niebla",
    },
    "Depositing rime fog" => {
        ["en"] => "Depositing rime fog",
        ["ru"] => "Изморозь",
        ["es"] => "Niebla con escarcha",
    },
    "Light drizzle" => {
        ["en"] => "Light drizzle",
        ["ru"] => "Легкая морось",
        ["es"] => "Llovizna ligera",
    },
    "Moderate drizzle" => {
        ["en"] => "Moderate drizzle",
        ["ru"] => "Умеренная морось",
        ["es"] => "Llovizna moderada",
    },
    "Dense drizzle" => {
        ["en"] => "Dense drizzle",
        ["ru"] => "Сильная морось",
        ["es"] => "Llovizna intensa",
    },
    "Light freezing drizzle" => {
        ["en"] => "Light freezing drizzle",
        ["ru"] => "Слабая ледяная морось",
        ["es"] => "Llovizna helada ligera",
    },
    "Dense freezing drizzle" => {
        ["en"] => "Dense freezing drizzle",
        ["ru"] => "Сильная ледяная морось",
        ["es"] => "Llovizna helada intensa",
    },
    "Slight rain" => {
        ["en"] => "Slight rain",
        ["ru"] => "Небольшой дождь",
        ["es"] => "Lluvia ligera",
    },
    "Moderate rain" => {
        ["en"] => "Moderate rain",
        ["ru"] => "Умеренный дождь",
        ["es"] => "Lluvia moderada",
    },
    "Heavy rain" => {
        ["en"] => "Heavy rain",
        ["ru"] => "Сильный дождь",
        ["es"] => "Lluvia intensa",
    },
    "Light freezing rain" => {
        ["en"] => "Light freezing rain",
        ["ru"] => "Слабый ледяной дождь",
        ["es"] => "Lluvia helada ligera",
    },
    "Heavy freezing rain" => {
        ["en"] => "Heavy freezing rain",
        ["ru"] => "Сильный ледяной дождь",
        ["es"] => "Lluvia helada intensa",
    },
    "Slight snow fall" => {
        ["en"] => "Slight snow fall",
        ["ru"] => "Небольшой снег",
        ["es"] => "Nevada ligera",
    },
    "Moderate snow fall" => {
        ["en"] => "Moderate snow fall",
        ["ru"] => "Умеренный снег",
        ["es"] => "Nevada moderada",
    },
    "Heavy snow fall" => {
        ["en"] => "Heavy snow fall",
        ["ru"] => "Сильный снегопад",
        ["es"] => "Nevada intensa",
    },
    "Snow grains" => {
        ["en"] => "Snow grains",
        ["ru"] => "Снежная крупа",
        ["es"] => "Granos de nieve",
    },
    "Slight rain showers" => {
        ["en"] => "Slight rain showers",
        ["ru"] => "Небольшой ливень",
        ["es"] => "Chubascos ligeros",
    },
    "Moderate rain showers" => {
        ["en"] => "Moderate rain showers",
        ["ru"] => "Умеренный ливень",
        ["es"] => "Chubascos moderados",
    },
    "Violent rain showers" => {
        ["en"] => "Violent rain showers",
        ["ru"] => "Сильный ливень",
        ["es"] => "Chubascos intensos",
    },
    "Slight snow showers" => {
        ["en"] => "Slight snow showers",
        ["ru"] => "Небольшой снежный ливень",
        ["es"] => "Chubascos de nieve ligeros",
    },
    "Heavy snow showers" => {
        ["en"] => "Heavy snow showers",
        ["ru"] => "Сильный снежный ливень",
        ["es"] => "Chubascos de nieve intensos",
    },
    "Thunderstorm" => {
        ["en"] => "Thunderstorm",
        ["ru"] => "Гроза",
        ["es"] => "Tormenta",
    },
    "Thunderstorm with slight hail" => {
        ["en"] => "Thunderstorm with slight hail",
        ["ru"] => "Гроза с небольшим градом",
        ["es"] => "Tormenta con granizo ligero",
    },
    "Thunderstorm with heavy hail" => {
        ["en"] => "Thunderstorm with heavy hail",
        ["ru"] => "Гроза с сильным градом",
        ["es"] => "Tormenta con granizo intenso",
    },
    "Unknown" => {
        ["en"] => "Unknown",
        ["ru"] => "Неизвестно",
        ["es"] => "Desconocido",
    },
    // Units
    "feels like" => {
        ["en"] => "feels like",
        ["ru"] => "ощущается как",
        ["es"] => "se siente como",
    },
    "mph" => {
        ["en"] => "mph",
        ["ru"] => "миль/ч",
        ["es"] => "mph",
    },
    "m/s" => {
        ["en"] => "m/s",
        ["ru"] => "м/с",
        ["es"] => "m/s",
    },
    "mm" => {
        ["en"] => "mm",
        ["ru"] => "мм",
        ["es"] => "mm",
    },
    "inch" => {
        ["en"] => "inch",
        ["ru"] => "дюйм",
        ["es"] => "pulgada",
    },
    "hPa" => {
        ["en"] => "hPa",
        ["ru"] => "гПа",
        ["es"] => "hPa",
    },
};

pub fn ll(lang: Language, key: &'static str) -> &'static str {
    TRANSLATIONS
        .get(lang.code())
        .and_then(|translations| translations.get(key))
        // TODO: Add logging for missing translations
        .unwrap_or(&key)
}
