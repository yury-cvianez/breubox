use std::time::{Instant, SystemTime};

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
}