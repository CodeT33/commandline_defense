use crate::entities::bullets::BulletData;
use crate::entities::bullets::BulletEmissionDataInner;
use crate::entities::enemies::EnemyData;
use crate::entities::health::HealthStatsInner;
use crate::entities::tower::TowerDataInner;
use crate::scheduling::TimePoint;
use bevy::ecs::entity::EntityHashSet;
use bevy::prelude::{Circle, Component, Entity, Rectangle};

#[derive(Component)]
pub(crate) struct Enemy(pub(crate) EnemyData);

#[derive(Component)]
pub(crate) struct Bullet(pub(crate) BulletData);

#[derive(Component)]
pub(crate) struct HealthStats(pub(crate) HealthStatsInner);

#[derive(Component)]
pub(crate) struct CreationTime(pub(crate) TimePoint);

#[derive(Component)]
pub(crate) struct BulletEmissionData(pub(crate) BulletEmissionDataInner);

#[derive(Component, Copy, Clone)]
pub(crate) enum ColliderShape {
    Rectangle(Rectangle),
    Circle(Circle),
}

#[derive(Component)]
pub(crate) struct DeleteWhenOutOfMap;

#[derive(Component)]
pub(crate) struct Map;

#[derive(Component)]
pub(crate) struct ColliderTypeA;

#[derive(Component)]
pub(crate) struct ColliderTypeB;

#[derive(Component, Default)]
pub(crate) struct Tower {
    pub(crate) enemies_in_range: EntityHashSet,
    pub(crate) target: Vec<Entity>,
}

#[derive(Component)]
pub(crate) struct TowerData(pub(crate) TowerDataInner);

#[derive(Component)]
pub(crate) struct GridOverlay;

#[derive(Component)]
pub(crate) struct GridLine;

#[derive(Component)]
pub(crate) struct GridPositionLabel;

#[derive(Component)]
pub(crate) struct TileHighlight;
