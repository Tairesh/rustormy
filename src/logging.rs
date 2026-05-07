use std::collections::VecDeque;
use std::io::{self, Write};
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Warn,
    Info,
    Debug,
}

impl Level {
    pub fn label(self) -> &'static str {
        match self {
            Self::Warn => "WARN",
            Self::Info => "INFO",
            Self::Debug => "DEBUG",
        }
    }

    pub fn threshold(self) -> u8 {
        match self {
            Self::Warn => 1,
            Self::Info => 2,
            Self::Debug => 3,
        }
    }

    fn ansi_code(self) -> &'static str {
        match self {
            Self::Warn => "\x1b[33m",
            Self::Info => "\x1b[36m",
            Self::Debug => "\x1b[90m",
        }
    }
}

pub struct Capture {
    lines: VecDeque<String>,
    capacity: usize,
    dropped: usize,
}

pub struct DrainedCapture {
    pub lines: Vec<String>,
    pub dropped: usize,
}

impl Capture {
    pub fn new(capacity: usize) -> Self {
        Self {
            lines: VecDeque::with_capacity(capacity),
            capacity,
            dropped: 0,
        }
    }

    pub fn push(&mut self, line: String) {
        if self.lines.len() == self.capacity {
            self.lines.pop_front();
            self.dropped += 1;
        }
        self.lines.push_back(line);
    }

    pub fn drain(&mut self) -> DrainedCapture {
        let lines = self.lines.drain(..).collect();
        let dropped = self.dropped;
        self.dropped = 0;
        DrainedCapture { lines, dropped }
    }
}

pub struct LoggerState {
    pub level: u8,
    pub use_colors: bool,
    pub capture: Option<Mutex<Capture>>,
}

static LOGGER: OnceLock<LoggerState> = OnceLock::new();

const CAPTURE_CAPACITY: usize = 1000;

pub fn state() -> Option<&'static LoggerState> {
    LOGGER.get()
}

pub fn init(level: u8, use_colors: bool) {
    let _ = LOGGER.set(LoggerState {
        level: clamp_level(level),
        use_colors,
        capture: None,
    });
}

pub fn init_with_capture(level: u8, use_colors: bool) -> CaptureGuard {
    let _ = LOGGER.set(LoggerState {
        level: clamp_level(level),
        use_colors,
        capture: Some(Mutex::new(Capture::new(CAPTURE_CAPACITY))),
    });
    CaptureGuard { _private: () }
}

pub struct CaptureGuard {
    _private: (),
}

impl Drop for CaptureGuard {
    fn drop(&mut self) {
        flush_capture();
    }
}

pub fn flush_capture() {
    let Some(state) = LOGGER.get() else { return };
    let Some(mutex) = state.capture.as_ref() else {
        return;
    };
    let drained = match mutex.lock() {
        Ok(mut guard) => guard.drain(),
        Err(poisoned) => poisoned.into_inner().drain(),
    };
    let mut stderr = io::stderr().lock();
    if drained.dropped > 0 {
        let header = format_line(
            state,
            Level::Info,
            &format!(
                "(capture buffer overflow: {} earlier lines dropped)",
                drained.dropped
            ),
        );
        let _ = writeln!(stderr, "{header}");
    }
    for line in drained.lines {
        let _ = writeln!(stderr, "{line}");
    }
}

pub(crate) fn clamp_level(level: u8) -> u8 {
    level.min(Level::Debug.threshold())
}

pub(crate) fn level_passes(state_level: u8, line_level: Level) -> bool {
    state_level >= line_level.threshold()
}

pub(crate) fn format_line(state: &LoggerState, level: Level, message: &str) -> String {
    if state.use_colors {
        format!(
            "{}[{}]\x1b[0m {}",
            level.ansi_code(),
            level.label(),
            message
        )
    } else {
        format!("[{}] {}", level.label(), message)
    }
}

pub(crate) fn emit(state: &'static LoggerState, level: Level, message: &str) {
    let line = format_line(state, level, message);
    if let Some(mutex) = state.capture.as_ref()
        && let Ok(mut guard) = mutex.lock()
    {
        guard.push(line);
        return;
    }
    let _ = writeln!(io::stderr().lock(), "{line}");
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        if let Some(state) = $crate::logging::state()
            && $crate::logging::level_passes(state.level, $crate::logging::Level::Warn)
        {
            $crate::logging::emit(state, $crate::logging::Level::Warn, &format!($($arg)*));
        }
    }};
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        if let Some(state) = $crate::logging::state()
            && $crate::logging::level_passes(state.level, $crate::logging::Level::Info)
        {
            $crate::logging::emit(state, $crate::logging::Level::Info, &format!($($arg)*));
        }
    }};
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        if let Some(state) = $crate::logging::state()
            && $crate::logging::level_passes(state.level, $crate::logging::Level::Debug)
        {
            $crate::logging::emit(state, $crate::logging::Level::Debug, &format!($($arg)*));
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(level_threshold: u8, line_level: Level, message: &str, use_colors: bool) -> String {
        let state = LoggerState {
            level: level_threshold,
            use_colors,
            capture: None,
        };
        format_line(&state, line_level, message)
    }

    #[test]
    fn formats_warn_without_colors() {
        let out = render(1, Level::Warn, "boom", false);
        assert_eq!(out, "[WARN] boom");
    }

    #[test]
    fn formats_info_without_colors() {
        let out = render(2, Level::Info, "ok", false);
        assert_eq!(out, "[INFO] ok");
    }

    #[test]
    fn colored_warn_contains_ansi() {
        let out = render(1, Level::Warn, "boom", true);
        assert!(out.contains("\x1b["), "expected ANSI escape, got {out:?}");
        assert!(out.contains("[WARN]"));
        assert!(out.contains("boom"));
    }

    #[test]
    fn level_passes_returns_true_for_equal_or_lower() {
        assert!(level_passes(1, Level::Warn));
        assert!(!level_passes(1, Level::Info));
        assert!(!level_passes(0, Level::Warn));
        assert!(level_passes(3, Level::Debug));
        assert!(!level_passes(2, Level::Debug));
    }

    #[test]
    fn level_clamping_in_init_state() {
        assert_eq!(clamp_level(0), 0);
        assert_eq!(clamp_level(1), 1);
        assert_eq!(clamp_level(3), 3);
        assert_eq!(clamp_level(99), 3);
    }

    #[test]
    fn capture_records_lines_when_active() {
        let mut cap = Capture::new(1000);
        cap.push("[WARN] one".to_string());
        cap.push("[INFO] two".to_string());
        let drained = cap.drain();
        assert_eq!(drained.lines, vec!["[WARN] one", "[INFO] two"]);
        assert_eq!(drained.dropped, 0);
    }

    #[test]
    fn capture_drops_oldest_when_full() {
        let mut cap = Capture::new(3);
        cap.push("a".to_string());
        cap.push("b".to_string());
        cap.push("c".to_string());
        cap.push("d".to_string());
        cap.push("e".to_string());
        let drained = cap.drain();
        assert_eq!(drained.lines, vec!["c", "d", "e"]);
        assert_eq!(drained.dropped, 2);
    }

    #[test]
    fn capture_drain_resets_state() {
        let mut cap = Capture::new(2);
        cap.push("x".to_string());
        cap.push("y".to_string());
        cap.push("z".to_string());
        let _ = cap.drain();
        cap.push("after".to_string());
        let drained = cap.drain();
        assert_eq!(drained.lines, vec!["after"]);
        assert_eq!(drained.dropped, 0);
    }
}
