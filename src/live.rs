use crate::app::App;
use crate::display::footer::format_footer;
use crate::display::formatter::WeatherFormatter;
use crate::errors::RustormyError;
use crate::models::Weather;
use chrono::{DateTime, Local};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::{cursor, execute, terminal};
use std::io::{self, Write};
use std::time::{Duration, Instant};

fn write_payload<W: Write>(writer: &mut W, text: &str) -> io::Result<()> {
    for line in text.split_inclusive('\n') {
        if let Some(stripped) = line.strip_suffix('\n') {
            writer.write_all(stripped.as_bytes())?;
            writer.write_all(b"\r\n")?;
        } else {
            writer.write_all(line.as_bytes())?;
        }
    }
    Ok(())
}

pub struct TerminalGuard;

impl TerminalGuard {
    pub fn enter() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        if let Err(e) = execute!(io::stdout(), terminal::EnterAlternateScreen, cursor::Hide) {
            let _ = terminal::disable_raw_mode();
            return Err(e);
        }
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), cursor::Show, terminal::LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

pub fn render(
    stdout: &mut io::Stdout,
    formatter: &WeatherFormatter,
    weather: &Weather,
    timestamp: DateTime<Local>,
    show_footer: bool,
    use_colors: bool,
) -> io::Result<()> {
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
    )?;
    let body = formatter.render_to_string(weather);
    write_payload(stdout, &body)?;
    if show_footer {
        let footer = format_footer(timestamp, use_colors);
        stdout.write_all(footer.as_bytes())?;
        stdout.write_all(b"\r\n")?;
    }
    stdout.flush()?;
    Ok(())
}

pub fn run(app: &mut App) -> Result<(), RustormyError> {
    let level = app.config().verbose();
    let use_colors = app.config().format().use_colors;
    let _capture = crate::logging::init_with_capture(level, use_colors);

    // First fetch runs before entering the alt screen so the user's
    // existing terminal contents stay visible during the initial API call.
    let mut weather = app.fetch_with_fallback()?;
    let mut now = Local::now();

    // Drain any logs produced during the first fetch directly to stderr
    // so they remain in the user's scrollback after the alt-screen exits.
    crate::logging::flush_capture();

    let _terminal = TerminalGuard::enter()?;
    let mut stdout = io::stdout();

    loop {
        let show_footer = app.config().live_mode_footer();
        let use_colors = app.config().format().use_colors;
        render(
            &mut stdout,
            app.formatter(),
            &weather,
            now,
            show_footer,
            use_colors,
        )?;

        let interval = Duration::from_secs(app.config().live_mode_interval());
        let deadline = Instant::now() + interval;

        loop {
            let remaining_wait = deadline.saturating_duration_since(Instant::now());
            if remaining_wait.is_zero() {
                break;
            }
            if event::poll(remaining_wait)? {
                match event::read()? {
                    Event::Key(KeyEvent {
                        code,
                        modifiers,
                        kind: KeyEventKind::Press,
                        ..
                    }) => match (code, modifiers) {
                        (KeyCode::Char('q') | KeyCode::Esc, _)
                        | (KeyCode::Char('c'), KeyModifiers::CONTROL) => return Ok(()),
                        (KeyCode::Char('r'), _) => break,
                        _ => {}
                    },
                    Event::Resize(_, _) => {
                        render(
                            &mut stdout,
                            app.formatter(),
                            &weather,
                            now,
                            show_footer,
                            use_colors,
                        )?;
                    }
                    _ => {}
                }
            }
        }

        weather = app.fetch_with_fallback()?;
        now = Local::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn capture(text: &str) -> Vec<u8> {
        let mut buf = Vec::new();
        write_payload(&mut buf, text).unwrap();
        buf
    }

    #[test]
    fn test_write_payload_empty() {
        assert_eq!(capture(""), b"");
    }

    #[test]
    fn test_write_payload_single_line_no_newline() {
        assert_eq!(capture("hello"), b"hello");
    }

    #[test]
    fn test_write_payload_single_line_with_newline() {
        assert_eq!(capture("hello\n"), b"hello\r\n");
    }

    #[test]
    fn test_write_payload_multi_line() {
        assert_eq!(capture("a\nb\nc\n"), b"a\r\nb\r\nc\r\n");
    }

    #[test]
    fn test_write_payload_no_trailing_newline_in_multiline() {
        assert_eq!(capture("a\nb\nc"), b"a\r\nb\r\nc");
    }
}
