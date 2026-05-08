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
                "      .-.    ",
                "   . ( (  *  ",
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
            Self::LightShowers if is_day => [
                "             ",
                " _`/\"\".-.    ",
                "  ,\\_(   ).  ",
                "   /(___(__) ",
                "     ' ' ' ' ",
                "    ' ' ' '  ",
                "             ",
            ],
            Self::LightShowers => [
                "     . *     ",
                "   * .-.     ",
                "    (   ).   ",
                "   *(___(__) ",
                "     ' ' ' ' ",
                "    ' ' ' '  ",
                "             ",
            ],
            Self::HeavyShowers if is_day => [
                "             ",
                " _`/\"\".-.    ",
                "  ,\\_(   ).  ",
                "   /(___(__) ",
                "   ‚'‚'‚'‚'  ",
                "   ‚'‚'‚'‚'  ",
                "             ",
            ],
            Self::HeavyShowers => [
                "     . *     ",
                "   * .-.     ",
                "    (   ).   ",
                "   *(___(__) ",
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
                "\x1b[38;5;111;1m   . \x1b[38;5;230;1m( (\x1b[38;5;111;1m  *  \x1b[0m",
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
            Self::LightShowers if is_day => [
                "             ",
                "\x1b[38;5;226m _`/\"\"\x1b[38;5;250m.-.    \x1b[0m",
                "\x1b[38;5;226m  ,\\_\x1b[38;5;250m(   ).  \x1b[0m",
                "\x1b[38;5;226m   /\x1b[38;5;250m(___(__) \x1b[0m",
                "\x1b[38;5;111m     ' ' ' ' \x1b[0m",
                "\x1b[38;5;111m    ' ' ' '  \x1b[0m",
                "             ",
            ],
            Self::LightShowers => [
                "\x1b[38;5;111;1m     . *     \x1b[0m",
                "\x1b[38;5;111;1m   *\x1b[38;5;250m .-.     \x1b[0m",
                "\x1b[38;5;250m    (   ).   \x1b[0m",
                "\x1b[38;5;111;1m   *\x1b[38;5;250m(___(__) \x1b[0m",
                "\x1b[38;5;111m     ' ' ' ' \x1b[0m",
                "\x1b[38;5;111m    ' ' ' '  \x1b[0m",
                "             ",
            ],
            Self::HeavyShowers if is_day => [
                "             ",
                "\x1b[38;5;226m _`/\"\"\x1b[38;5;240;1m.-.    \x1b[0m",
                "\x1b[38;5;226m  ,\\_\x1b[38;5;240;1m(   ).  \x1b[0m",
                "\x1b[38;5;226m   /\x1b[38;5;240;1m(___(__) \x1b[0m",
                "\x1b[38;5;21;1m   ‚'‚'‚'‚'  \x1b[0m",
                "\x1b[38;5;21;1m   ‚'‚'‚'‚'  \x1b[0m",
                "             ",
            ],
            Self::HeavyShowers => [
                "\x1b[38;5;111;1m     . *     \x1b[0m",
                "\x1b[38;5;111;1m   *\x1b[38;5;240;1m .-.     \x1b[0m",
                "\x1b[38;5;240;1m    (   ).   \x1b[0m",
                "\x1b[38;5;111;1m   *\x1b[38;5;240;1m(___(__) \x1b[0m",
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
            WeatherConditionIcon::LightShowers if is_day => "🌦️ ",
            WeatherConditionIcon::LightShowers | WeatherConditionIcon::HeavyShowers => "🌧️ ",
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

    fn rendered_width(s: &str) -> usize {
        let mut width = 0;
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c == '\x1b' {
                for c2 in chars.by_ref() {
                    if c2 == 'm' {
                        break;
                    }
                }
            } else if c == '⚡' {
                width += 2;
            } else {
                width += 1;
            }
        }
        width
    }

    #[test]
    fn all_icons_have_uniform_rendered_width() {
        const VARIANTS: &[WeatherConditionIcon] = &[
            WeatherConditionIcon::Unknown,
            WeatherConditionIcon::Clear,
            WeatherConditionIcon::PartlyCloudy,
            WeatherConditionIcon::Cloudy,
            WeatherConditionIcon::LightShowers,
            WeatherConditionIcon::HeavyShowers,
            WeatherConditionIcon::LightSnow,
            WeatherConditionIcon::HeavySnow,
            WeatherConditionIcon::Thunderstorm,
            WeatherConditionIcon::Fog,
        ];
        let mut failures: Vec<String> = Vec::new();
        for &v in VARIANTS {
            for is_day in [true, false] {
                for (kind, icon) in [
                    ("icon", v.icon(is_day)),
                    ("colored_icon", v.colored_icon(is_day)),
                ] {
                    for (i, line) in icon.iter().enumerate() {
                        let width = rendered_width(line);
                        if width != 13 {
                            failures.push(format!(
                                "{v:?} is_day={is_day} {kind} line {i}: {width} cells: {line:?}"
                            ));
                        }
                    }
                }
            }
        }
        assert!(
            failures.is_empty(),
            "non-13-width icon lines:\n  {}",
            failures.join("\n  ")
        );
    }

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
            WeatherConditionIcon::LightSnow,
            WeatherConditionIcon::HeavySnow,
            WeatherConditionIcon::Thunderstorm,
            WeatherConditionIcon::Fog,
        ] {
            assert_eq!(variant.icon(true), variant.icon(false), "{variant:?}");
            assert_eq!(variant.emoji(true), variant.emoji(false), "{variant:?}");
        }
    }

    #[test]
    fn light_showers_day_and_night_differ() {
        assert_ne!(
            WeatherConditionIcon::LightShowers.icon(true),
            WeatherConditionIcon::LightShowers.icon(false)
        );
        assert_ne!(
            WeatherConditionIcon::LightShowers.colored_icon(true),
            WeatherConditionIcon::LightShowers.colored_icon(false)
        );
        assert_ne!(
            WeatherConditionIcon::LightShowers.emoji(true),
            WeatherConditionIcon::LightShowers.emoji(false)
        );
    }

    #[test]
    fn heavy_showers_day_and_night_differ() {
        assert_ne!(
            WeatherConditionIcon::HeavyShowers.icon(true),
            WeatherConditionIcon::HeavyShowers.icon(false)
        );
        assert_ne!(
            WeatherConditionIcon::HeavyShowers.colored_icon(true),
            WeatherConditionIcon::HeavyShowers.colored_icon(false)
        );
        assert_eq!(
            WeatherConditionIcon::HeavyShowers.emoji(true),
            WeatherConditionIcon::HeavyShowers.emoji(false)
        );
    }
}
