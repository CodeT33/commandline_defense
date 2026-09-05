use crate::collision::CollisionPair;
use crate::consts;
use crate::ecs_elements::components::{
    Bullet, BulletEmissionData, ColliderShape, ColliderTypeB, CreationTime, Enemy, Tower,
};
use crate::ecs_elements::messages::CollisionStarted;
use crate::ecs_elements::resources::TexturePackSettings;
use crate::scheduling::TimePoint;
use crate::texture_packs::TexturePackAssets;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use std::f32::consts::PI;

pub struct BulletEmissionDataInner {
    last_spawn_time_ms: Option<u64>,
    paused: bool,
    direction: Rot2,
    bullet_speed_tps: f32,
    spawn_cooldown_ms: u32,
}

impl Default for BulletEmissionDataInner {
    fn default() -> Self {
        Self {
            last_spawn_time_ms: None,
            paused: false,
            direction: Rot2::degrees(0.0),
            bullet_speed_tps: consts::PROJECTILE_SPEED_TILES_PER_SECOND,
            spawn_cooldown_ms: consts::TOWER_COOLDOWN_MS,
        }
    }
}

impl BulletEmissionDataInner {
    pub fn new(spawn_cooldown_ms: u32, bullet_speed_tps: f32) -> Self {
        Self { spawn_cooldown_ms, bullet_speed_tps, ..Default::default() }
    }

    /// Call this function in a loop until it returns None to ensure no bullets are dropped.\
    /// When a shot is available, the function returns the time at which the shot was fired. Otherwise, it returns None.
    pub fn shoot_if_ready(&mut self, time: &Time) -> Option<TimePoint> {
        let current_time_ms = time.elapsed().as_millis() as u64;
        let was_paused = self.paused;
        self.paused = false;
        if let Some(last_spawn_time_ms) = &mut self.last_spawn_time_ms {
            if *last_spawn_time_ms + self.spawn_cooldown_ms as u64 <= current_time_ms {
                if was_paused {
                    *last_spawn_time_ms = current_time_ms;
                } else {
                    *last_spawn_time_ms += self.spawn_cooldown_ms as u64;
                }
                Some(TimePoint::from_ms(*last_spawn_time_ms))
            } else {
                None
            }
        } else {
            self.last_spawn_time_ms = Some(current_time_ms);
            Some(TimePoint::from_ms(current_time_ms))
        }
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }
}

pub fn move_bullets(mut q: Query<(&mut Transform, &Bullet, &CreationTime)>, time: Res<Time>) {
    for (mut tf, bullet, creation_time) in &mut q {
        let velocity = bullet.velocity * time.delta_secs();
        tf.translation.x += velocity.x;
        tf.translation.y += velocity.y;

        tf.rotation = Quat::from_rotation_z(
            (creation_time.0.elapsed_ms(&time) % consts::BULLET_ROTATION_DURATION_MS) as f32
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
            .flat_map(|e| enemies_q.get(*e))
            .max_by(|a, b| a.1.path_progress.total_cmp(&b.1.path_progress));
        let Some((enemy_transform, _)) = first_enemy else {
            continue;
        };
        let angle = (enemy_transform.translation.truncate() - t.translation.truncate()).to_angle();
        t.rotation = Quat::from_rotation_z(angle - PI / 2.0);
        bullet_data.0.direction = Rot2::radians(angle);
    }
}

pub fn spawn_bullets(
    mut commands: Commands,
    mut q: Query<(&Transform, &Tower, &mut BulletEmissionData), With<Tower>>, time: Res<Time>,
    asset_server: Res<AssetServer>, texture_pack_settings: Res<TexturePackSettings>,
) {
    for (transform, tower, mut data) in &mut q {
        let emission_data = &mut data.0;
        if tower.enemies_in_range.is_empty() {
            emission_data.pause();
            continue;
        }
        while let Some(shoot_time) = emission_data.shoot_if_ready(&time) {
            commands.spawn((
                Bullet {
                    velocity: emission_data.direction * Vec2::X * emission_data.bullet_speed_tps,
                },
                CreationTime(shoot_time),
                ColliderTypeB,
                ColliderShape::circle(consts::PROJECTILE_RADIUS),
                Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    consts::rendering_layers::ENTITY,
                ),
                Sprite {
                    image: asset_server.load(
                        texture_pack_settings
                            .get_asset_path(TexturePackAssets::Projectiles_MetalBall),
                    ),
                    custom_size: consts::PROJECTILE_SIZE_TILES.into(),
                    ..default()
                },
            ));
        }
    }
}

pub fn handle_bullet_enemy_collisions(
    mut commands: Commands, mut collision_reader: MessageReader<CollisionStarted>,
    bullet_query: Query<(), With<Bullet>>, enemy_query: Query<(), With<Enemy>>,
) {
    for &CollisionStarted(CollisionPair { type_a, type_b }) in collision_reader.read() {
        if !enemy_query.contains(type_a) || !bullet_query.contains(type_b) {
            continue;
        }

        commands.entity(type_a).try_despawn();
        commands.entity(type_b).try_despawn();
    }
}
