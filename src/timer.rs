use std::{fmt::Display, time::Duration};
pub const SECOND: Duration = Duration::from_secs(1);
pub const MINUTE: u64 = 60;

pub struct Timer {
    name: String,
    start_duration: Duration,
    remaining: Duration,
}

impl Display for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let seconds = self.remaining().as_secs() % MINUTE;
        let minutes = self.remaining().as_secs() / MINUTE;
        let seconds_zero_char = if ((seconds as f64) / 10.0) < 1.0 {
            "0"
        } else {
            ""
        };

        let zero_char = if minutes == 0 { "0" } else { "" };
        write!(
            f,
            "{} {zero_char}{minutes}:{seconds_zero_char}{seconds}",
            self.name()
        )
    }
}

impl Timer {
    pub fn new(name: &str, duration: u64) -> Self {
        Timer {
            name: String::from(name),
            start_duration: Duration::from_secs(duration),
            remaining: Duration::from_secs(duration),
        }
    }

    /// Advance timer forward by one second.
    pub fn advance(&mut self) {
        self.remaining = self.remaining.saturating_sub(SECOND);
    }

    /// Reset timer to its starting duration.
    pub fn reset(&mut self) {
        self.remaining = self.remaining.saturating_add(self.start_duration);
    }

    /// Get remaining duration of a timer.
    pub fn remaining(&self) -> &Duration {
        &self.remaining
    }

    /// Get the name of the timer.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Format duration as minutes:seconds.
pub fn format_duration(duration: &Duration) -> String {
    let seconds = duration.as_secs() % MINUTE;
    let minutes = duration.as_secs() / MINUTE;
    let seconds_zero_char = if (seconds as f64) / 10.0 < 1.0 {
        "0"
    } else {
        ""
    };

    let zero_char = if minutes == 0 { "0" } else { "" };

    format!("{zero_char}{minutes}:{seconds_zero_char}{seconds}")
}
