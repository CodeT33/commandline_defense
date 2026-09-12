use crate::cli::command_input::{OpenCommand, Settings};
use crate::collision::CollisionPair;
use crate::coordinates::GridCoordinate;
use crate::entities::bullets::BulletType;
use crate::entities::enemies::EnemyType;
use crate::entities::tower::TowerType;
use crate::scheduling::TimePoint;
use bevy::prelude::{Deref, DerefMut, Entity, Message, Rot2, Vec2};

#[derive(Message)]
pub(crate) struct SpawnEnemy {
    pub(crate) enemy_type: EnemyType,
    pub(crate) time: TimePoint,
}

#[derive(Message)]
pub(crate) struct SpawnBullet {
    pub(crate) bullet_type: BulletType,
    pub(crate) time: TimePoint,
    pub(crate) position: Vec2,
    pub(crate) direction: Rot2,
    pub(crate) speed_tps: f32,
    pub(crate) target_entity: Option<Entity>,
}

#[derive(Message)]
pub(crate) struct PlaceTowerMessage {
    pub(crate) tower_type: TowerType,
    pub(crate) tower_pos: GridCoordinate,
}

#[derive(Message, Deref, DerefMut)]
pub(crate) struct CollisionStarted(pub(crate) CollisionPair);

#[derive(Message, Deref, DerefMut)]
pub(crate) struct CollisionSustained(pub(crate) CollisionPair);

#[derive(Message, Deref, DerefMut)]
pub(crate) struct CollisionEnded(pub(crate) CollisionPair);

#[derive(Message, Debug, PartialEq, Copy, Clone)]
pub(crate) enum CommandEvent {
    Select { tile: GridCoordinate },
    Place { tower_type: TowerType, tower_pos: GridCoordinate },
    Clear,
    Pause,
    Resume,
    ExitGame,
    Set(Settings),
    Open(OpenCommand),
}

#[derive(Message)]
pub(crate) struct EnemyReachedEnd(pub(crate) Entity);
