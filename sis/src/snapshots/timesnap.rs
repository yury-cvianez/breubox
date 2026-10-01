use std::time::{Instant, SystemTime, Duration};

#[derive(Debug, Clone)]

pub struct Timestamp {
    pub wall: SystemTime,
    pub monotonic: Instant,
}

impl Timestamp {
    pub fn now() -> Self {
        Self {
            wall: SystemTime::now(),
            monotonic: Instant::now(),
        }
    }

    pub fn duration_since(&self, earlier: &Timestamp) -> std::time::Duration {
        self.monotonic.duration_since(earlier.monotonic)
    }

    pub fn elapsed(&self) -> std::time::Duration {
        self.monotonic.elapsed()
    }
}

