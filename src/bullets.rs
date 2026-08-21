use crate::consts;
use crate::map::{Enemy, Tower};
use avian2d::prelude::*;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use std::f32::consts::PI;

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

impl Default for BulletEmissionData {
    fn default() -> Self {
        Self {
            last_spawn_time_ms: None,
            direction: Rot2::degrees(0.0),
            bullet_speed: consts::PROJECTILE_SPEED_TILES_PER_SECOND,
            spawn_cooldown_ms: consts::TOWER_COOLDOWN_MS,
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

    pub fn pause(&mut self) {
        self.last_spawn_time_ms = None;
    }
}

pub fn bullet_movement(mut q: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut tf, bullet) in &mut q {
        let velocity = bullet.velocity * 1.0 / consts::PHYSICS_FRAME_RATE as f32;
        tf.translation.x += velocity.x;
        tf.translation.y += velocity.y;
        tf.rotation = Quat::from_rotation_z(
            ((time.elapsed().as_millis() as u64 - bullet.spawn_time)
                % consts::BULLET_ROTATION_DURATION_MS) as f32
                / consts::BULLET_ROTATION_DURATION_MS as f32
                * PI
                * 2.0,
        )
    }
}

pub fn rotate_towers(
    tower_q: Query<(&mut Transform, &Tower, &mut BulletEmissionData)>,
    enemies_q: Query<(&Transform, &Enemy), Without<Tower>>,
) {
    for (mut t, tower, mut bullet_data) in tower_q {
        let first_enemy = tower
            .enemies_in_range
            .iter()
            .map(|e| enemies_q.get(*e).unwrap())
            .max_by(|a, b| a.1.path_offset.total_cmp(&b.1.path_offset));
        let Some((enemy_transform, _)) = first_enemy else {
            continue;
        };
        let angle = (enemy_transform.translation.truncate() - t.translation.truncate()).to_angle();
        t.rotation = Quat::from_rotation_z(angle - PI / 2.0);
        bullet_data.direction = Rot2::radians(angle);
    }
}

pub fn tower_shooting(
    mut commands: Commands,
    mut q: Query<(&Transform, &Tower, &mut BulletEmissionData), With<Tower>>, time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    for (transform, tower, mut data) in &mut q {
        if tower.enemies_in_range.is_empty() {
            data.pause();
            continue;
        }
        while let Some(shoot_time) = data.shoot_if_ready(time.elapsed().as_millis() as u64) {
            commands.spawn((
                Bullet {
                    velocity: data.direction * Vec2::X * data.bullet_speed,
                    spawn_time: shoot_time,
                },
                RigidBody::Kinematic,
                Collider::circle(consts::PROJECTILE_RADIUS),
                Sensor,
                CollisionEventsEnabled,
                Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    consts::rendering_layers::ENTITY,
                ),
                Sprite {
                    image: asset_server
                        .load(consts::assets::resource_packs::base_pack::projectiles::METAL_BALL),
                    custom_size: consts::PROJECTILE_SIZE_TILES.into(),
                    ..default()
                },
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
        commands.entity(bullet).try_despawn();
        commands.entity(enemy).try_despawn();
    }
}
