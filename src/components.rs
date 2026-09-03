use crate::tower::TowerDataInner;
use bevy::ecs::entity::EntityHashSet;
use bevy::math::{Rot2, Vec2};
use bevy::prelude::{Circle, Component, Rectangle};

#[derive(Component)]
pub struct Enemy {
    pub path_offset: f32,
    pub path_progress: f32,
}

#[derive(Component)]
pub struct Bullet {
    pub velocity: Vec2,
    pub spawn_time: u64,
}

#[derive(Component)]
pub struct BulletEmissionData {
    pub last_spawn_time_ms: Option<u64>,
    pub direction: Rot2,
    pub bullet_speed: f32,
    pub spawn_cooldown_ms: u32,
}

#[derive(Component, Copy, Clone)]
pub enum ColliderShape {
    Rectangle(Rectangle),
    Circle(Circle),
}

#[derive(Component)]
pub struct ColliderTypeA;

#[derive(Component)]
pub struct ColliderTypeB;

#[derive(Component, Default)]
pub struct Tower {
    pub enemies_in_range: EntityHashSet,
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
