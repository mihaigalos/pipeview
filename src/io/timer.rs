use std::time::{Duration, Instant};

pub struct Timer {
    pub countdown: Duration,
    pub delta: Duration,
    pub last_instant: Instant,
    pub period: Duration,
    pub ready: bool,
    pub start: Instant,
}

impl Timer {
    pub fn new(resolution_ms: u64) -> Self {
        let now = Instant::now();
        Self {
            countdown: Duration::default(),
            delta: Duration::default(),
            last_instant: now,
            period: Duration::from_millis(resolution_ms),
            ready: true,
            start: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();

        self.delta = now - self.last_instant;
        self.last_instant = now;
        self.countdown = self.countdown.checked_sub(self.delta).unwrap_or_else(|| {
            self.ready = true;
            self.period
        })
    }
}
