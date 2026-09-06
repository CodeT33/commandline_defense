use crate::entities::enemies::EnemyType;
use crate::scheduling::TimePoint;
use bevy::prelude::Event;

#[derive(Event)]
pub struct SpawnEnemy {
    pub enemy_type: EnemyType,
    pub time: TimePoint,
}
