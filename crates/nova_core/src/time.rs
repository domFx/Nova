use std::time::{SystemTime, UNIX_EPOCH};

/// Engine time tracking system
pub struct Time {
    start_time: f64,
    elapsed: f64
}

impl Time {
    /// Creates a new Time instance with the current system time as the start time
    pub fn start() -> Self {
        Self {
            start_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64(),
            elapsed: 0f64
        }
    }

    /// Updates the elapsed time between now and the captured start time
    pub fn update(&mut self) {
        if self.start_time > 0f64 {
            self.elapsed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64() - self.start_time;
        }   
    }
}