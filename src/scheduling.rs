use bevy::prelude::Time;
use std::ops::{Add, Sub};
use std::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub(crate) struct TimePoint(Duration);

impl TimePoint {
    pub(crate) fn now(time: &Time) -> Self {
        Self(time.elapsed())
    }

    pub(crate) fn from_ms(ms: u64) -> Self {
        Self(Duration::from_millis(ms))
    }

    pub(crate) fn elapsed_ms(&self, time: &Time) -> u64 {
        time.elapsed().saturating_sub(self.0).as_millis() as u64
    }
}

impl Add<Duration> for TimePoint {
    type Output = TimePoint;

    fn add(self, rhs: Duration) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub for TimePoint {
    type Output = Duration;
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

pub(crate) struct IntervalTimer {
    last_occurrence_ms: Option<u64>,
    paused: bool,
    interval_ms: u32,
}

impl IntervalTimer {
    pub(crate) fn new(interval_ms: u32) -> Self {
        Self { last_occurrence_ms: None, paused: false, interval_ms: interval_ms.max(1) }
    }

    pub(crate) fn get_interval_ms(&self) -> u32 {
        self.interval_ms
    }

    pub(crate) fn set_interval_ms(&mut self, interval_ms: u32) {
        self.interval_ms = interval_ms.max(1);
    }

    /// Call this function in a loop until it returns None to ensure no ticks are dropped.\
    /// When a tick is available, the function returns the time at which the tick has occurred. Otherwise, it returns None.
    pub(crate) fn tick_if_ready(&mut self, time: &Time) -> Option<TimePoint> {
        let now_ms = time.elapsed().as_millis() as u64;
        let was_paused = self.paused;
        self.paused = false;
        if let Some(last_occurrence_ms) = &mut self.last_occurrence_ms {
            if *last_occurrence_ms + self.interval_ms as u64 <= now_ms {
                if was_paused {
                    *last_occurrence_ms = now_ms;
                } else {
                    *last_occurrence_ms += self.interval_ms as u64;
                }
                Some(TimePoint::from_ms(*last_occurrence_ms))
            } else {
                None
            }
        } else {
            self.last_occurrence_ms = Some(now_ms);
            Some(TimePoint::from_ms(now_ms))
        }
    }

    pub(crate) fn get_next_tick_time(&self, time: &Time) -> Option<TimePoint> {
        let now_ms = time.elapsed().as_millis() as u64;
        if let Some(last_occurrence_ms) = self.last_occurrence_ms {
            if last_occurrence_ms + self.interval_ms as u64 <= now_ms {
                Some(TimePoint::from_ms(if self.paused {
                    now_ms
                } else {
                    last_occurrence_ms + self.interval_ms as u64
                }))
            } else {
                None
            }
        } else {
            Some(TimePoint::from_ms(now_ms))
        }
    }

    /// Hold the schedule. The next due tick resumes from the poll time instead
    /// of replaying the backlog accumulated while paused.
    pub(crate) fn pause(&mut self) {
        self.paused = true;
    }
}
