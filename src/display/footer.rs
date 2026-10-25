use crate::display::colored_text;
use crate::models::AnsiColor;
use chrono::{DateTime, Local};

pub fn format_footer(timestamp: DateTime<Local>, use_colors: bool, stale: bool) -> String {
    let time = timestamp.format("%H:%M:%S");
    if use_colors {
        let q = colored_text("[q]", AnsiColor::BrightBlack);
        let r = colored_text("[r]", AnsiColor::BrightBlack);
        let label = colored_text("•  last update:", AnsiColor::BrightBlack);
        let stale_mark = if stale {
            format!(" {}", colored_text("(stale)", AnsiColor::Yellow))
        } else {
            String::new()
        };
        format!("{q} quit  {r} refresh  {label} {time}{stale_mark}")
    } else {
        let stale_mark = if stale { " (stale)" } else { "" };
        format!("[q] quit  [r] refresh  •  last update: {time}{stale_mark}")
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
        let footer = format_footer(ts, false, false);
        assert_eq!(footer, "[q] quit  [r] refresh  •  last update: 14:32:05");
    }

    #[test]
    fn test_format_footer_with_color_contains_ansi_escape() {
        let ts = Local
            .with_ymd_and_hms(2026, 5, 5, 14, 32, 5)
            .single()
            .unwrap();
        let footer = format_footer(ts, true, false);
        assert!(footer.contains("14:32:05"));
        assert!(
            footer.contains("\x1b["),
            "expected ANSI escape in colored footer"
        );
        assert!(!footer.contains("(stale)"));
    }

    #[test]
    fn test_format_footer_stale_no_color() {
        let ts = Local
            .with_ymd_and_hms(2026, 5, 5, 14, 32, 5)
            .single()
            .unwrap();
        let footer = format_footer(ts, false, true);
        assert_eq!(
            footer,
            "[q] quit  [r] refresh  •  last update: 14:32:05 (stale)"
        );
    }

    #[test]
    fn test_format_footer_stale_with_color_is_yellow() {
        let ts = Local
            .with_ymd_and_hms(2026, 5, 5, 14, 32, 5)
            .single()
            .unwrap();
        let footer = format_footer(ts, true, true);
        assert!(
            footer.ends_with(" \x1b[33m(stale)\x1b[0m"),
            "expected yellow (stale) suffix, got {footer:?}"
        );
    }
}
