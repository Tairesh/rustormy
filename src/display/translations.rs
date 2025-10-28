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
        ["ko"] => "위치",
    },
    "Temperature" => {
        ["en"] => "Temperature",
        ["ru"] => "Температура",
        ["es"] => "Temperatura",
        ["ko"] => "온도",
    },
    "Condition" => {
        ["en"] => "Condition",
        ["ru"] => "Погода",
        ["es"] => "Condición",
        ["ko"] => "상태",
    },
    "Wind" => {
        ["en"] => "Wind",
        ["ru"] => "Ветер",
        ["es"] => "Viento",
        ["ko"] => "바람",
    },
    "Humidity" => {
        ["en"] => "Humidity",
        ["ru"] => "Влажность",
        ["es"] => "Humedad",
        ["ko"] => "습도",
    },
    "Precipitation" => {
        ["en"] => "Precip",
        ["ru"] => "Осадки",
        ["es"] => "Lluvias",
        ["ko"] => "강수량",
    },
    "Pressure" => {
        ["en"] => "Pressure",
        ["ru"] => "Давление",
        ["es"] => "Presión",
        ["ko"] => "기압",
    },
    "UV index" => {
        ["en"] => "UV index",
        ["ru"] => "УФ индекс",
        ["es"] => "Índice UV",
        ["ko"] => "자외선 지수",
    },
    "dew point" => {
        ["en"] => "dew point",
        ["ru"] => "точка росы",
        ["es"] => "punto de rocío",
        ["ko"] => "이슬점",
    },
    // Weather conditions
    "Clear" => {
        ["en"] => "Clear",
        ["ru"] => "Ясно",
        ["es"] => "Despejado",
        ["ko"] => "맑음",
    },
    "Mostly clear" => {
        ["en"] => "Mostly clear",
        ["ru"] => "Легкая облачность",
        ["es"] => "Mayormente despejado",
        ["ko"] => "대체로 맑음",
    },
    "Partly cloudy" => {
        ["en"] => "Partly cloudy",
        ["ru"] => "Переменная облачность",
        ["es"] => "Parcialmente nublado",
        ["ko"] => "부분적으로 흐림",
    },
    "Mostly cloudy" => {
        ["en"] => "Mostly cloudy",
        ["ru"] => "Сильная облачность",
        ["es"] => "Mayormente nublado",
        ["ko"] => "대체로 흐림",
    },
    "Overcast" => {
        ["en"] => "Overcast",
        ["ru"] => "Пасмурно",
        ["es"] => "Nublado",
        ["ko"] => "흐림",
    },
    "Cloudy" => {
        ["en"] => "Cloudy",
        ["ru"] => "Облачно",
        ["es"] => "Nublado",
        ["ko"] => "흐림",
    },
    "Fog" => {
        ["en"] => "Fog",
        ["ru"] => "Туман",
        ["es"] => "Niebla",
        ["ko"] => "안개",
    },
    "Light fog" => {
        ["en"] => "Light fog",
        ["ru"] => "Легкий туман",
        ["es"] => "Niebla ligera",
        ["ko"] => "옅은 안개",
    },
    "Depositing rime fog" => {
        ["en"] => "Depositing rime fog",
        ["ru"] => "Изморозь",
        ["es"] => "Niebla con escarcha",
        ["ko"] => "서리 안개",
    },
    "Drizzle" => {
        ["en"] => "Drizzle",
        ["ru"] => "Морось",
        ["es"] => "Llovizna",
        ["ko"] => "이슬비",
    },
    "Light drizzle" => {
        ["en"] => "Light drizzle",
        ["ru"] => "Легкая морось",
        ["es"] => "Llovizna ligera",
        ["ko"] => "약한 이슬비",
    },
    "Moderate drizzle" => {
        ["en"] => "Moderate drizzle",
        ["ru"] => "Умеренная морось",
        ["es"] => "Llovizna moderada",
        ["ko"] => "보통 이슬비",
    },
    "Dense drizzle" => {
        ["en"] => "Dense drizzle",
        ["ru"] => "Сильная морось",
        ["es"] => "Llovizna intensa",
        ["ko"] => "짙은 이슬비",
    },
    "Freezing drizzle" => {
        ["en"] => "Freezing drizzle",
        ["ru"] => "Ледяная морось",
        ["es"] => "Llovizna helada",
        ["ko"] => "얼음 이슬비",
    },
    "Light freezing drizzle" => {
        ["en"] => "Light freezing drizzle",
        ["ru"] => "Слабая ледяная морось",
        ["es"] => "Llovizna helada ligera",
        ["ko"] => "약한 얼음 이슬비",
    },
    "Dense freezing drizzle" => {
        ["en"] => "Dense freezing drizzle",
        ["ru"] => "Сильная ледяная морось",
        ["es"] => "Llovizna helada intensa",
        ["ko"] => "짙은 얼음 이슬비",
    },
    "Rain" => {
        ["en"] => "Rain",
        ["ru"] => "Дождь",
        ["es"] => "Lluvia",
        ["ko"] => "비",
    },
    "Light rain" => {
        ["en"] => "Light rain",
        ["ru"] => "Небольшой дождь",
        ["es"] => "Lluvia ligera",
        ["ko"] => "약한 비",
    },
    "Moderate rain" => {
        ["en"] => "Moderate rain",
        ["ru"] => "Умеренный дождь",
        ["es"] => "Lluvia moderada",
        ["ko"] => "보통 비",
    },
    "Heavy rain" => {
        ["en"] => "Heavy rain",
        ["ru"] => "Сильный дождь",
        ["es"] => "Lluvia intensa",
        ["ko"] => "강한 비",
    },
    "Freezing rain" => {
        ["en"] => "Freezing rain",
        ["ru"] => "Ледяной дождь",
        ["es"] => "Lluvia helada",
        ["ko"] => "얼음 비",
    },
    "Light freezing rain" => {
        ["en"] => "Light freezing rain",
        ["ru"] => "Слабый ледяной дождь",
        ["es"] => "Lluvia helada ligera",
        ["ko"] => "약한 얼음 비",
    },
    "Heavy freezing rain" => {
        ["en"] => "Heavy freezing rain",
        ["ru"] => "Сильный ледяной дождь",
        ["es"] => "Lluvia helada intensa",
        ["ko"] => "강한 얼음 비",
    },
    "Snow" => {
        ["en"] => "Snow",
        ["ru"] => "Снег",
        ["es"] => "Nevada",
        ["ko"] => "눈",
    },
    "Slight snow fall" => {
        ["en"] => "Slight snow fall",
        ["ru"] => "Небольшой снег",
        ["es"] => "Nevada ligera",
        ["ko"] => "약한 눈",
    },
    "Moderate snow fall" => {
        ["en"] => "Moderate snow fall",
        ["ru"] => "Умеренный снег",
        ["es"] => "Nevada moderada",
        ["ko"] => "보통 눈",
    },
    "Heavy snow fall" => {
        ["en"] => "Heavy snow fall",
        ["ru"] => "Сильный снегопад",
        ["es"] => "Nevada intensa",
        ["ko"] => "강한 눈",
    },
    "Flurries" => {
        ["en"] => "Flurries",
        ["ru"] => "Поземок",
        ["es"] => "Chubascos de nieve",
        ["ko"] => "눈보라",
    },
    "Snow grains" => {
        ["en"] => "Snow grains",
        ["ru"] => "Снежная крупа",
        ["es"] => "Granos de nieve",
        ["ko"] => "눈 알갱이",
    },
    "Ice pellets" => {
        ["en"] => "Ice pellets",
        ["ru"] => "Град",
        ["es"] => "Granizo",
        ["ko"] => "우박",
    },
    "Light ice pellets" => {
        ["en"] => "Light ice pellets",
        ["ru"] => "Небольшой град",
        ["es"] => "Granizo ligero",
        ["ko"] => "약한 우박",
    },
    "Heavy ice pellets" => {
        ["en"] => "Heavy ice pellets",
        ["ru"] => "Сильный град",
        ["es"] => "Granizo intenso",
        ["ko"] => "강한 우박",
    },
    "Slight rain showers" => {
        ["en"] => "Slight rain showers",
        ["ru"] => "Небольшой ливень",
        ["es"] => "Chubascos ligeros",
        ["ko"] => "약한 소나기",
    },
    "Moderate rain showers" => {
        ["en"] => "Moderate rain showers",
        ["ru"] => "Умеренный ливень",
        ["es"] => "Chubascos moderados",
        ["ko"] => "보통 소나기",
    },
    "Violent rain showers" => {
        ["en"] => "Violent rain showers",
        ["ru"] => "Сильный ливень",
        ["es"] => "Chubascos intensos",
        ["ko"] => "강한 소나기",
    },
    "Light snow" => {
        ["en"] => "Light snow",
        ["ru"] => "Небольшой снег",
        ["es"] => "Nieve ligera",
        ["ko"] => "약한 눈",
    },
    "Heavy snow" => {
        ["en"] => "Heavy snow",
        ["ru"] => "Сильный снег",
        ["es"] => "Nieve intensa",
        ["ko"] => "강한 눈",
    },
    "Slight snow showers" => {
        ["en"] => "Slight snow showers",
        ["ru"] => "Небольшой снежный ливень",
        ["es"] => "Chubascos de nieve ligeros",
        ["ko"] => "약한 눈 소나기",
    },
    "Heavy snow showers" => {
        ["en"] => "Heavy snow showers",
        ["ru"] => "Сильный снежный ливень",
        ["es"] => "Chubascos de nieve intensos",
        ["ko"] => "강한 눈 소나기",
    },
    "Thunderstorm" => {
        ["en"] => "Thunderstorm",
        ["ru"] => "Гроза",
        ["es"] => "Tormenta",
        ["ko"] => "뇌우",
    },
    "Thunderstorm with slight hail" => {
        ["en"] => "Thunderstorm with slight hail",
        ["ru"] => "Гроза с небольшим градом",
        ["es"] => "Tormenta con granizo ligero",
        ["ko"] => "약한 우박을 동반한 뇌우",
    },
    "Thunderstorm with heavy hail" => {
        ["en"] => "Thunderstorm with heavy hail",
        ["ru"] => "Гроза с сильным градом",
        ["es"] => "Tormenta con granizo intenso",
        ["ko"] => "강한 우박을 동반한 뇌우",
    },
    "Unknown" => {
        ["en"] => "Unknown",
        ["ru"] => "Неизвестно",
        ["es"] => "Desconocido",
        ["ko"] => "알 수 없음",
    },
    // Units
    "feels like" => {
        ["en"] => "feels like",
        ["ru"] => "ощущается как",
        ["es"] => "se siente como",
        ["ko"] => "체감 온도",
    },
    "mph" => {
        ["en"] => "mph",
        ["ru"] => "миль/ч",
        ["es"] => "mph",
        ["ko"] => "mph",
    },
    "m/s" => {
        ["en"] => "m/s",
        ["ru"] => "м/с",
        ["es"] => "m/s",
        ["ko"] => "m/s",
    },
    "mm" => {
        ["en"] => "mm",
        ["ru"] => "мм",
        ["es"] => "mm",
        ["ko"] => "mm",
    },
    "inch" => {
        ["en"] => "inch",
        ["ru"] => "дюйм",
        ["es"] => "pulgada",
        ["ko"] => "인치",
    },
    "hPa" => {
        ["en"] => "hPa",
        ["ru"] => "гПа",
        ["es"] => "hPa",
        ["ko"] => "hPa",
    },
};

pub fn ll(lang: Language, key: &'static str) -> &'static str {
    TRANSLATIONS
        .get(lang.code())
        .and_then(|translations| translations.get(key))
        // TODO: Add logging for missing translations
        .unwrap_or(&key)
}
