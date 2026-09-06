pub struct HealthStatsInner {
    current_health: f32,
    max_health: f32,
}

impl HealthStatsInner {
    pub fn new(max_health: f32) -> Self {
        Self { current_health: max_health, max_health }
    }

    pub fn ratio(&self) -> f32 {
        self.current_health / self.max_health
    }
}
