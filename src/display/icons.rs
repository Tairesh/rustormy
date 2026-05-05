use crate::models::WeatherConditionIcon;

pub type Icon = [&'static str; 7];

impl WeatherConditionIcon {
    #[allow(clippy::too_many_lines)]
    pub fn icon(self, is_day: bool) -> Icon {
        match self {
            Self::Unknown => [
                "             ",
                "    .-.      ",
                "     __)     ",
                "    (        ",
                "     `-’     ",
                "      •      ",
                "             ",
            ],
            Self::Clear if is_day => [
                "             ",
                "    \\   /    ",
                "     .-.     ",
                "  ― (   ) ―  ",
                "     `-’     ",
                "    /   \\    ",
                "             ",
            ],
            Self::Clear => [
                "     .  *    ",
                "       .-.   ",
                "   . (   )   ",
                "      `-’    ",
                "   *   .     ",
                "             ",
                "             ",
            ],
            Self::PartlyCloudy if is_day => [
                "             ",
                "   \\  /      ",
                " _ /\"\".-.    ",
                "   \\_(   ).  ",
                "   /(___(__) ",
                "             ",
                "             ",
            ],
            Self::PartlyCloudy => [
                "             ",
                "     . *     ",
                "   _ .-.     ",
                "    (   ).   ",
                "   *(___(__) ",
                "             ",
                "             ",
            ],
            Self::Cloudy => [
                "             ",
                "             ",
                "     .--.    ",
                "  .-(    ).  ",
                " (___.__)__) ",
                "             ",
                "             ",
            ],
            Self::LightShowers => [
                "             ",
                " _`/\"\".-.    ",
                "  ,\\_(   ).  ",
                "   /(___(__) ",
                "     ' ' ' ' ",
                "    ' ' ' '  ",
                "             ",
            ],
            Self::HeavyShowers => [
                "             ",
                " _`/\"\".-.    ",
                "  ,\\_(   ).  ",
                "   /(___(__) ",
                "   ‚'‚'‚'‚'  ",
                "   ‚'‚'‚'‚'  ",
                "             ",
            ],
            Self::LightSnow => [
                "             ",
                "     .-.     ",
                "    (   ).   ",
                "   (___(__)  ",
                "    *  *  *  ",
                "   *  *  *   ",
                "             ",
            ],
            Self::HeavySnow => [
                "             ",
                "     .-.     ",
                "    (   ).   ",
                "   (___(__)  ",
                "   * * * *   ",
                "  * * * *    ",
                "             ",
            ],
            Self::Thunderstorm => [
                "             ",
                "     .-.     ",
                "    (   ).   ",
                "   (___(__)  ",
                "   ⚡\"\"⚡\"\"  ",
                "  ‚'‚'‚'‚'   ",
                "             ",
            ],
            Self::Fog => [
                "             ",
                "             ",
                " _ - _ - _ - ",
                "  _ - _ - _  ",
                " _ - _ - _ - ",
                "             ",
                "             ",
            ],
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn colored_icon(self, is_day: bool) -> Icon {
        match self {
            Self::Unknown => [
                "             ",
                "    .-.      ",
                "     __)     ",
                "    (        ",
                "     `-’     ",
                "      •      ",
                "             ",
            ],
            Self::Clear if is_day => [
                "             ",
                "\x1b[38;5;226m    \\   /    \x1b[0m",
                "\x1b[38;5;226m     .-.     \x1b[0m",
                "\x1b[38;5;226m  ― (   ) ―  \x1b[0m",
                "\x1b[38;5;226m     `-’     \x1b[0m",
                "\x1b[38;5;226m    /   \\    \x1b[0m",
                "             ",
            ],
            Self::Clear => [
                "\x1b[38;5;111;1m     .  *    \x1b[0m",
                "\x1b[38;5;230;1m      .-.    \x1b[0m",
                "\x1b[38;5;111;1m   . \x1b[38;5;230;1m(   )   \x1b[0m",
                "\x1b[38;5;230;1m      `-’    \x1b[0m",
                "\x1b[38;5;111;1m   *   .     \x1b[0m",
                "             ",
                "             ",
            ],
            Self::PartlyCloudy if is_day => [
                "             ",
                "\x1b[38;5;226m   \\  /\x1b[0m      ",
                "\x1b[38;5;226m _ /\"\"\x1b[38;5;250m.-.    \x1b[0m",
                "\x1b[38;5;226m   \\_\x1b[38;5;250m(   ).  \x1b[0m",
                "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m",
                "             ",
                "             ",
            ],
            Self::PartlyCloudy => [
                "             ",
                "\x1b[38;5;111;1m     . *     \x1b[0m",
                "\x1b[38;5;230;1m     \x1b[38;5;250m.-.     \x1b[0m",
                "\x1b[38;5;250m    (   ).   \x1b[0m",
                "\x1b[38;5;111;1m   *\x1b[38;5;250m(___(__) \x1b[0m",
                "             ",
                "             ",
            ],
            Self::Cloudy => [
                "             ",
                "             ",
                "\x1b[38;5;250m     .--.    \x1b[0m",
                "\x1b[38;5;250m  .-(    ).  \x1b[0m",
                "\x1b[38;5;250m (___.__)__) \x1b[0m",
                "             ",
                "             ",
            ],
            Self::LightShowers => [
                "             ",
                "\x1b[38;5;226m _`/\"\"\x1b[38;5;250m.-.    \x1b[0m",
                "\x1b[38;5;226m  ,\\_\x1b[38;5;250m(   ).  \x1b[0m",
                "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m",
                "\x1b[38;5;111m     ' ' ' ' \x1b[0m",
                "\x1b[38;5;111m    ' ' ' '  \x1b[0m",
                "             ",
            ],
            Self::HeavyShowers => [
                "             ",
                "\x1b[38;5;226m _`/\"\"\x1b[38;5;240;1m.-.    \x1b[0m",
                "\x1b[38;5;226m  ,\\_\x1b[38;5;240;1m(   ).  \x1b[0m",
                "\x1b[38;5;226m   /\x1b[38;5;240;1m(___(__) \x1b[0m",
                "\x1b[38;5;21;1m   ‚'‚'‚'‚'  \x1b[0m",
                "\x1b[38;5;21;1m   ‚'‚'‚'‚'  \x1b[0m",
                "             ",
            ],
            Self::LightSnow => [
                "             ",
                "\x1b[38;5;250m     .-.     \x1b[0m",
                "\x1b[38;5;250m    (   ).   \x1b[0m",
                "\x1b[38;5;250m   (___(__)  \x1b[0m",
                "\x1b[38;5;255m    *  *  *  \x1b[0m",
                "\x1b[38;5;255m   *  *  *   \x1b[0m",
                "             ",
            ],
            Self::HeavySnow => [
                "             ",
                "\x1b[38;5;240;1m     .-.     \x1b[0m",
                "\x1b[38;5;240;1m    (   ).   \x1b[0m",
                "\x1b[38;5;240;1m   (___(__)  \x1b[0m",
                "\x1b[38;5;255;1m   * * * *   \x1b[0m",
                "\x1b[38;5;255;1m  * * * *    \x1b[0m",
                "             ",
            ],
            Self::Thunderstorm => [
                "             ",
                "\x1b[38;5;240;1m     .-.     \x1b[0m",
                "\x1b[38;5;240;1m    (   ).   \x1b[0m",
                "\x1b[38;5;240;1m   (___(__)  \x1b[0m",
                "\x1b[38;5;228;5m   ⚡\x1b[38;5;111;25m\"\"\x1b[38;5;228;5m⚡\x1b[38;5;111;25m\"\"  \x1b[0m",
                "\x1b[38;5;21;1m  ‚'‚'‚'‚'   \x1b[0m",
                "             ",
            ],
            Self::Fog => [
                "             ",
                "             ",
                "\x1b[38;5;251m _ - _ - _ - \x1b[0m",
                "\x1b[38;5;251m  _ - _ - _  \x1b[0m",
                "\x1b[38;5;251m _ - _ - _ - \x1b[0m",
                "             ",
                "             ",
            ],
        }
    }

    pub fn emoji(self, is_day: bool) -> &'static str {
        match self {
            WeatherConditionIcon::Unknown => "❓",
            WeatherConditionIcon::Clear if is_day => "☀️ ",
            WeatherConditionIcon::Clear => "🌙",
            WeatherConditionIcon::PartlyCloudy if is_day => "⛅️",
            WeatherConditionIcon::PartlyCloudy | WeatherConditionIcon::Cloudy => "☁️ ",
            WeatherConditionIcon::LightShowers => "🌦️ ",
            WeatherConditionIcon::HeavyShowers => "🌧️ ",
            WeatherConditionIcon::LightSnow => "🌨️ ",
            WeatherConditionIcon::HeavySnow => "❄️ ",
            WeatherConditionIcon::Thunderstorm => "⛈️ ",
            WeatherConditionIcon::Fog => "🌫 ",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_day_and_night_differ() {
        assert_ne!(
            WeatherConditionIcon::Clear.icon(true),
            WeatherConditionIcon::Clear.icon(false)
        );
        assert_ne!(
            WeatherConditionIcon::Clear.colored_icon(true),
            WeatherConditionIcon::Clear.colored_icon(false)
        );
        assert_ne!(
            WeatherConditionIcon::Clear.emoji(true),
            WeatherConditionIcon::Clear.emoji(false)
        );
    }

    #[test]
    fn partly_cloudy_day_and_night_differ() {
        assert_ne!(
            WeatherConditionIcon::PartlyCloudy.icon(true),
            WeatherConditionIcon::PartlyCloudy.icon(false)
        );
        assert_ne!(
            WeatherConditionIcon::PartlyCloudy.emoji(true),
            WeatherConditionIcon::PartlyCloudy.emoji(false)
        );
    }

    #[test]
    fn cloudy_ignores_is_day() {
        assert_eq!(
            WeatherConditionIcon::Cloudy.icon(true),
            WeatherConditionIcon::Cloudy.icon(false)
        );
        assert_eq!(
            WeatherConditionIcon::Cloudy.colored_icon(true),
            WeatherConditionIcon::Cloudy.colored_icon(false)
        );
        assert_eq!(
            WeatherConditionIcon::Cloudy.emoji(true),
            WeatherConditionIcon::Cloudy.emoji(false)
        );
    }

    #[test]
    fn other_conditions_ignore_is_day() {
        for variant in [
            WeatherConditionIcon::Unknown,
            WeatherConditionIcon::LightShowers,
            WeatherConditionIcon::HeavyShowers,
            WeatherConditionIcon::LightSnow,
            WeatherConditionIcon::HeavySnow,
            WeatherConditionIcon::Thunderstorm,
            WeatherConditionIcon::Fog,
        ] {
            assert_eq!(variant.icon(true), variant.icon(false), "{variant:?}");
            assert_eq!(variant.emoji(true), variant.emoji(false), "{variant:?}");
        }
    }
}
