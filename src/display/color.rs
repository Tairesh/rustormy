use crate::models::AnsiColor;
use std::fmt::Display;

pub fn colored_text(text: impl Display, color: AnsiColor) -> String {
    format!("\x1b[{color}m{text}\x1b[0m")
}
