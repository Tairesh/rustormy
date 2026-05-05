use crate::display::colored_text;
use crate::models::AnsiColor;
use chrono::{DateTime, Local};

pub fn format_footer(timestamp: DateTime<Local>, use_colors: bool) -> String {
    let time = timestamp.format("%H:%M:%S");
    if use_colors {
        let q = colored_text("[q]", AnsiColor::BrightBlack);
        let r = colored_text("[r]", AnsiColor::BrightBlack);
        let label = colored_text("•  last update:", AnsiColor::BrightBlack);
        format!("{q} quit  {r} refresh  {label} {time}")
    } else {
        format!("[q] quit  [r] refresh  •  last update: {time}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_format_footer_no_color() {
        let ts = Local
            .with_ymd_and_hms(2026, 5, 5, 14, 32, 5)
            .single()
            .unwrap();
        let footer = format_footer(ts, false);
        assert_eq!(footer, "[q] quit  [r] refresh  •  last update: 14:32:05");
    }

    #[test]
    fn test_format_footer_with_color_contains_ansi_escape() {
        let ts = Local
            .with_ymd_and_hms(2026, 5, 5, 14, 32, 5)
            .single()
            .unwrap();
        let footer = format_footer(ts, true);
        assert!(footer.contains("14:32:05"));
        assert!(
            footer.contains("\x1b["),
            "expected ANSI escape in colored footer"
        );
    }
}
