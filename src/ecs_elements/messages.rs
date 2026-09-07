use crate::cli::command_line_state_management::Settings;
use crate::collision::CollisionPair;
use crate::coordinates::GridCoordinate;
use crate::entities::bullets::BulletType;
use crate::entities::enemies::EnemyType;
use crate::entities::tower::TowerType;
use crate::scheduling::TimePoint;
use bevy::prelude::{Entity, Message, Rot2, Vec2};

#[derive(Message)]
pub struct SpawnEnemy {
    pub enemy_type: EnemyType,
    pub time: TimePoint,
}

#[derive(Message)]
pub struct SpawnBullet {
    pub bullet_type: BulletType,
    pub time: TimePoint,
    pub position: Vec2,
    pub direction: Rot2,
    pub speed_tps: f32,
}

#[derive(Message)]
pub struct PlaceTowerMessage {
    pub tower_type: TowerType,
    pub tower_pos: GridCoordinate,
}

#[derive(Message)]
pub struct CollisionStarted(pub CollisionPair);

#[derive(Message)]
pub struct CollisionSustained(pub CollisionPair);

#[derive(Message)]
pub struct CollisionEnded(pub CollisionPair);

#[derive(Message, Debug)]
pub enum CommandEvent {
    Help,
    Select { tile: GridCoordinate },
    Place { tower_type: TowerType, tower_pos: GridCoordinate },
    Clear,
    Balance,
    ExitGame,
    Set { setting: Settings, value: f32 },
}

#[derive(Message)]
pub struct EnemyReachedEnd(pub Entity);
