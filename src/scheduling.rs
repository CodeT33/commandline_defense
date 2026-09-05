use bevy::prelude::Time;

pub struct TimePoint {
    ms: u64,
}

impl TimePoint {
    pub fn now(time: &Time) -> Self {
        Self { ms: time.elapsed().as_millis() as u64 }
    }

    pub fn from_ms(ms: u64) -> Self {
        Self { ms }
    }

    pub fn elapsed_ms(&self, time: &Time) -> u64 {
        (time.elapsed().as_millis() as u64).saturating_sub(self.ms)
    }
}
