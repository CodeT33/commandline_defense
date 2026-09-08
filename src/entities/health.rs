pub(crate) struct HealthStatsInner {
    current_health: f32,
    max_health: f32,
}

impl HealthStatsInner {
    pub(crate) fn new(max_health: f32) -> Self {
        Self { current_health: max_health, max_health }
    }

    pub(crate) fn ratio(&self) -> f32 {
        self.current_health / self.max_health
    }

    pub(crate) fn change_health(&mut self, amount: f32) {
        self.current_health = (self.current_health + amount).clamp(0.0, self.max_health);
    }

    #[allow(unused)]
    pub(crate) fn current_health(&self) -> f32 {
        self.current_health
    }

    pub(crate) fn is_dead(&self) -> bool {
        self.current_health == 0.0
    }
}
