use crate::entities::bullets::BulletData;
use crate::entities::bullets::BulletEmissionDataInner;
use crate::entities::enemies::EnemyData;
use crate::entities::health::HealthStatsInner;
use crate::entities::tower::TowerDataInner;
use crate::scheduling::TimePoint;
use bevy::prelude::{Circle, Component, Deref, DerefMut, Entity, Rectangle};

#[derive(Component, Deref, DerefMut)]
pub(crate) struct Enemy(pub(crate) EnemyData);

#[derive(Component, Deref, DerefMut)]
pub(crate) struct Bullet(pub(crate) BulletData);

#[derive(Component, Deref, DerefMut)]
pub(crate) struct TargetEnemy(pub(crate) Entity);

#[derive(Component, Deref, DerefMut)]
pub(crate) struct HealthStats(pub(crate) HealthStatsInner);

#[derive(Component, Deref, DerefMut)]
pub(crate) struct CreationTime(pub(crate) TimePoint);

#[derive(Component, Deref, DerefMut)]
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

#[derive(Component, Deref, DerefMut)]
pub(crate) struct TowerData(pub(crate) TowerDataInner);

#[derive(Component)]
pub(crate) struct GridOverlay;

#[derive(Component)]
pub(crate) struct GridLine;

#[derive(Component)]
pub(crate) struct GridPositionLabel;

#[derive(Component)]
pub(crate) struct TileHighlight;
