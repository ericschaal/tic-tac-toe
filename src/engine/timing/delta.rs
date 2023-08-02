use std::time::{Duration, Instant};

pub struct Delta {
    instant: Instant
}

impl Default for Delta {
    fn default() -> Self {
        Self {
            instant: Instant::now()
        }
    }
}

impl Delta {
    pub fn reset(&mut self) {
        self.instant = Instant::now()
    }

    pub fn elapsed(&self) -> Duration {
        self.instant.elapsed()
    }
}