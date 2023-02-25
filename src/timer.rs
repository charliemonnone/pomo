use std::time::Duration;
pub const SECOND: Duration = Duration::from_secs(1);

pub struct Timer {
    name: String,
    start_duration: Duration,
    remaining: Duration,
}

impl Timer {
    pub fn new(name: &str, duration: u64) -> Self {
        Timer {
            name: String::from(name),
            start_duration: Duration::from_secs(duration),
            remaining: Duration::from_secs(duration),
        }
    }
    /// Advance timer forward.
    pub fn advance(&mut self) {
        self.remaining = self.remaining.saturating_sub(SECOND)
    }

    /// Reset timer to its starting duration.
    pub fn reset(&mut self) {
        self.remaining = self.start_duration.clone();
    }

    /// Get remaining duration of active timer. 
    pub fn remaining(&self) -> &Duration {
        &self.remaining
    }

    /// Get the name of the timer.
    pub fn name(&self) -> &str {
        &self.name
    }
}
