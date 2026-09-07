use crate::collision::CollisionPair;
use crate::consts;
use crate::ecs_elements::components::{
    Bullet, BulletEmissionData, ColliderShape, ColliderTypeB, CreationTime, DeleteWhenOutOfMap,
    Enemy, HealthStats, Tower, TowerData,
};
use crate::ecs_elements::messages::{CollisionEnded, CollisionStarted, SpawnBullet};
use crate::ecs_elements::resources::TexturePackSettings;
use crate::entities::health::HealthStatsInner;
use crate::scheduling::IntervalTimer;
use crate::texture_packs::TexturePackAssets;
use bevy::asset::AssetServer;
use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Copy, Clone, Debug)]
pub enum BulletType {
    Bullet,
    MetalBall,
    Apple,
}

pub struct BulletStats {
    pub damage: f32,
    pub health: f32,
    pub spins: bool,
    pub relative_collider_size: f32,
    pub texture_size_tiles: f32,
    pub asset: TexturePackAssets,
}

#[allow(unused)]
pub struct BulletData {
    bullet_type: BulletType,
    rotation: Rot2,
    speed_tps: f32,
}

pub struct BulletEmissionDataInner {
    pub timer: IntervalTimer,
    direction: Rot2,
}

impl Default for BulletEmissionDataInner {
    fn default() -> Self {
        Self { timer: IntervalTimer::new(consts::TOWER_COOLDOWN_MS), direction: Rot2::degrees(0.0) }
    }
}

impl BulletEmissionDataInner {
    pub fn new(spawn_cooldown_ms: u32) -> Self {
        Self { timer: IntervalTimer::new(spawn_cooldown_ms), ..Default::default() }
    }
}

pub fn move_bullets(mut q: Query<(&mut Transform, Ref<Bullet>, &CreationTime)>, time: Res<Time>) {
    for (mut tf, bullet, creation_time) in &mut q {
        let delta_time = if bullet.is_added() {
            creation_time.0.elapsed_ms(&time) as f32 / 1000.0
        } else {
            time.delta_secs()
        };
        let velocity = bullet.0.rotation * Vec2::X * bullet.0.speed_tps * delta_time;
        tf.translation += velocity.extend(0.0);

        tf.rotation = if bullet.0.bullet_type.get_stats().spins {
            Quat::from_rotation_z(
                (creation_time.0.elapsed_ms(&time) % consts::BULLET_ROTATION_DURATION_MS) as f32
                    / consts::BULLET_ROTATION_DURATION_MS as f32
                    * PI
                    * 2.0,
            )
        } else {
            Quat::from_rotation_z(velocity.to_angle() + PI / -2.0)
        }
    }
}

pub fn update_towers_in_range_and_rotate(
    enemies: Query<Entity, With<Enemy>>,
    enemy_transforms: Query<(&Transform, &Enemy), Without<Tower>>,
    mut towers: Query<(Entity, &mut Transform, &mut Tower, &mut BulletEmissionData)>,
    mut collision_started: MessageReader<CollisionStarted>,
    mut collision_ended: MessageReader<CollisionEnded>,
) {
    for CollisionStarted(CollisionPair { type_a, type_b }) in collision_started.read() {
        let Some(Ok((_, _, mut tower, _))) =
            enemies.contains(*type_a).then(|| towers.get_mut(*type_b))
        else {
            continue;
        };
        tower.enemies_in_range.insert(*type_a);
    }
    for CollisionEnded(CollisionPair { type_a, type_b }) in collision_ended.read() {
        let Ok((_, _, mut tower, _)) = towers.get_mut(*type_b) else {
            continue;
        };
        tower.enemies_in_range.remove(type_a);
    }

    for (_, mut t, tower, mut bullet_data) in &mut towers {
        let first_enemy = tower
            .enemies_in_range
            .iter()
            .flat_map(|e| enemy_transforms.get(*e))
            .max_by(|a, b| a.1.0.get_path_progress().total_cmp(&b.1.0.get_path_progress()));
        let Some((enemy_transform, _)) = first_enemy else {
            continue;
        };
        let angle = (enemy_transform.translation.truncate() - t.translation.truncate()).to_angle();
        t.rotation = Quat::from_rotation_z(angle - PI / 2.0);
        bullet_data.0.direction = Rot2::radians(angle);
    }
}

pub fn request_bullet_spawns(
    mut bullet_spawns: MessageWriter<SpawnBullet>,
    mut q: Query<(&Transform, &Tower, &TowerData, &mut BulletEmissionData), With<Tower>>,
    time: Res<Time>,
) {
    for (transform, tower, tower_data, mut data) in &mut q {
        let emission_data = &mut data.0;
        if tower.enemies_in_range.is_empty() {
            emission_data.timer.pause();
            continue;
        }
        while let Some(shoot_time) = emission_data.timer.tick_if_ready(&time) {
            bullet_spawns.write(SpawnBullet {
                bullet_type: tower_data.0.bullet_type,
                time: shoot_time,
                position: transform.translation.truncate(),
                direction: emission_data.direction,
                speed_tps: tower_data.0.bullet_speed_tps,
            });
        }
    }
}

pub fn handle_bullet_spawns(
    mut bullet_spawns: MessageReader<SpawnBullet>, mut commands: Commands,
    asset_server: Res<AssetServer>, texture_pack_settings: Res<TexturePackSettings>,
) {
    for message in bullet_spawns.read() {
        let stats = message.bullet_type.get_stats();
        commands.spawn((
            Bullet(BulletData::new(message.bullet_type, message.direction, message.speed_tps)),
            HealthStats(HealthStatsInner::new(stats.health)),
            CreationTime(message.time),
            ColliderTypeB,
            ColliderShape::circle(stats.texture_size_tiles * stats.relative_collider_size / 2.0),
            DeleteWhenOutOfMap,
            Transform::from_translation(message.position.extend(consts::rendering_layers::ENTITY)),
            Sprite {
                image: asset_server.load(texture_pack_settings.get_asset_path(stats.asset)),
                custom_size: Some(Vec2::splat(stats.texture_size_tiles)),
                image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
                ..default()
            },
        ));
    }
}

pub fn handle_bullet_enemy_collisions(
    mut commands: Commands, mut collision_reader: MessageReader<CollisionStarted>,
    mut bullet_query: Query<(&mut HealthStats, &Bullet), Without<Enemy>>,
    mut enemy_query: Query<&mut HealthStats, With<Enemy>>,
) {
    for &CollisionStarted(CollisionPair { type_a, type_b }) in collision_reader.read() {
        let (Ok((mut bullet_health, bullet)), Ok(mut enemy_health)) =
            (bullet_query.get_mut(type_b), enemy_query.get_mut(type_a))
        else {
            continue;
        };
        enemy_health.0.change_health(-bullet.0.bullet_type.get_stats().damage);
        if enemy_health.0.is_dead() {
            commands.entity(type_a).try_despawn();
        }
        bullet_health.0.change_health(-1.0);
        if bullet_health.0.is_dead() {
            commands.entity(type_b).try_despawn();
        }
    }
}

impl BulletData {
    pub fn new(bullet_type: BulletType, rotation: Rot2, speed_tps: f32) -> Self {
        Self { bullet_type, rotation, speed_tps }
    }
}
