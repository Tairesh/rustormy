use std::fmt::Display;

pub fn colored_text(text: impl Display, color: AnsiColor) -> String {
    format!("\x1b[{color}m{text}\x1b[0m")
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum AnsiColor {
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    White = 37,
    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightWhite = 97,
}

impl Display for AnsiColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}
