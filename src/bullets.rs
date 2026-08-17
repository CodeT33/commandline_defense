use crate::consts;
use crate::map::{Enemy, Tower};
use avian2d::prelude::*;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct BulletEmissionData {
    last_spawn_time_ms: Option<u64>,
    pub direction: Rot2,
    pub bullet_speed: f32,
    pub spawn_cooldown_ms: u32,
}

impl Default for BulletEmissionData {
    fn default() -> Self {
        Self {
            last_spawn_time_ms: None,
            direction: Rot2::degrees(0.0),
            bullet_speed: 10.0,
            spawn_cooldown_ms: 1000,
        }
    }
}

impl BulletEmissionData {
    /// Call this function in a loop until it returns None to ensure no bullets are dropped.\
    /// When a shot is available, the function returns the time at which the shot was fired. Otherwise, it returns None.
    pub fn shoot_if_ready(&mut self, current_time_ms: u64) -> Option<u64> {
        if let Some(last_spawn_time_ms) = &mut self.last_spawn_time_ms {
            if *last_spawn_time_ms + self.spawn_cooldown_ms as u64 <= current_time_ms {
                *last_spawn_time_ms += self.spawn_cooldown_ms as u64;
                Some(*last_spawn_time_ms)
            } else {
                None
            }
        } else {
            self.last_spawn_time_ms = Some(current_time_ms);
            Some(current_time_ms)
        }
    }
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct SpawnTime(pub u64);

pub fn bullet_movement(mut q: Query<(&mut Transform, &Velocity, &SpawnTime)>, time: Res<Time>) {
    for (mut tf, velocity, spawn_time) in &mut q {
        let velocity = velocity.0 * 1.0 / consts::PHYSICS_FRAME_RATE as f32;
        tf.translation.x += velocity.x;
        tf.translation.y += velocity.y;
        tf.rotation = Quat::from_rotation_z(
            ((time.elapsed().as_millis() as u64 - spawn_time.0)
                % consts::BULLET_ROTATION_DURATION_MS) as f32
                / consts::BULLET_ROTATION_DURATION_MS as f32
                * PI
                * 2.0,
        )
    }
}

pub fn bullet_spawning(
    mut commands: Commands, mut q: Query<(&Transform, &mut BulletEmissionData), With<Tower>>,
    time: Res<Time>, asset_server: Res<AssetServer>,
) {
    for (transform, mut data) in &mut q {
        while let Some(shoot_time) = data.shoot_if_ready(time.elapsed().as_millis() as u64) {
            commands.spawn((
                Bullet,
                RigidBody::Kinematic,
                Collider::circle(consts::PROJECTILE_RADIUS),
                Sensor,
                CollisionEventsEnabled,
                Velocity(data.direction * Vec2::X * data.bullet_speed),
                Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0),
                Sprite {
                    image: asset_server.load(consts::paths::sprite::APPLE),
                    custom_size: consts::PROJECTILE_SIZE_TILES.into(),
                    ..default()
                },
                SpawnTime(shoot_time),
            ));
        }
    }
}

pub fn bullet_collisions(
    mut commands: Commands, mut collision_reader: MessageReader<CollisionStart>,
    bullet_query: Query<(), With<Bullet>>, enemy_query: Query<(), With<Enemy>>,
) {
    for event in collision_reader.read() {
        let bullet = if bullet_query.contains(event.collider1) {
            event.collider1
        } else if bullet_query.contains(event.collider2) {
            event.collider2
        } else {
            continue;
        };
        let enemy = if enemy_query.contains(event.collider1) {
            event.collider1
        } else if enemy_query.contains(event.collider2) {
            event.collider2
        } else {
            continue;
        };
        commands.entity(bullet).despawn();
        commands.entity(enemy).despawn();
    }
}
