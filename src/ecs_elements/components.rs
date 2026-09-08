use crate::entities::bullets::BulletData;
use crate::entities::bullets::BulletEmissionDataInner;
use crate::entities::enemies::EnemyData;
use crate::entities::health::HealthStatsInner;
use crate::entities::tower::TowerDataInner;
use crate::scheduling::TimePoint;
use bevy::ecs::entity::EntityHashSet;
use bevy::prelude::{Circle, Component, Entity, Rectangle};

#[derive(Component)]
pub struct Enemy(pub EnemyData);

#[derive(Component)]
pub struct Bullet(pub BulletData);

#[derive(Component)]
pub struct HealthStats(pub HealthStatsInner);

#[derive(Component)]
pub struct CreationTime(pub TimePoint);

#[derive(Component)]
pub struct BulletEmissionData(pub BulletEmissionDataInner);

#[derive(Component, Copy, Clone)]
pub enum ColliderShape {
    Rectangle(Rectangle),
    Circle(Circle),
}

#[derive(Component)]
pub struct DeleteWhenOutOfMap;

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct ColliderTypeA;

#[derive(Component)]
pub struct ColliderTypeB;

#[derive(Component, Default)]
pub struct Tower {
    pub enemies_in_range: EntityHashSet,
    pub target: Vec<Entity>,
}

#[derive(Component)]
pub struct TowerData(pub TowerDataInner);

#[derive(Component)]
pub struct GridOverlay;

#[derive(Component)]
pub struct GridLine;

#[derive(Component)]
pub struct GridPositionLabel;

#[derive(Component)]
pub struct TileHighlight;
