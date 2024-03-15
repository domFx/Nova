use std::time::{SystemTime, UNIX_EPOCH};

struct Time {
    start_time: f64,
    elapsed: f64
}

impl Time {
    pub fn start() -> Self {
        Self {
            start_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64(),
            elapsed: 0f64
        }
    }

    pub fn update(&mut self) {
        if self.start_time > 0f64 {
            self.elapsed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64() - self.start_time;
        }
    }

    pub fn stop(&mut self) {
        self.start_time = 0f64;
        self.elapsed = 0f64;
    }
}