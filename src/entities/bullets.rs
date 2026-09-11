use crate::consts;
use crate::ecs_elements::components::{
    Bullet, ColliderShape, ColliderTypeB, CreationTime, DeleteWhenOutOfMap, Enemy, HealthStats,
    TargetEnemy,
};
use crate::ecs_elements::messages::{CollisionStarted, SpawnBullet};
use crate::ecs_elements::resources::TexturePackSettings;
use crate::entities::health::HealthStatsInner;
use crate::scheduling::IntervalTimer;
use crate::texture_packs::TexturePackAssets;
use crate::tiers::ValueTiers;
use crate::tiers::ValueType::{BulletDamage, BulletPierce};
use bevy::asset::AssetServer;
use bevy::prelude::*;
use std::f32::consts::PI;
use std::ops::Deref;

#[derive(Copy, Clone, Debug)]
pub(crate) enum BulletType {
    MediumMetalBall,
    BigMetalBall,

    SmallGoldBullet,
    MediumGoldBullet,

    SmallCopperBullet,
    MediumCopperBullet,

    SmallOrangeRocket,
    MediumOrangeRocket,

    EitshtuProjectile,
    AcitonionProjectile,
    StrormProjectile,
    InfernonProjectile,
    IcebyteProjectile,

    // meme stuff
    AppleBall,
    OrangeBall,
    DonsBananos,
}

pub(crate) struct BulletStats {
    pub(crate) damage: ValueTiers,
    pub(crate) pierce: ValueTiers,
    pub(crate) spins: bool,
    pub(crate) relative_collider_size: f32,
    pub(crate) texture_size_tiles: f32,
    pub(crate) asset: TexturePackAssets,
}

pub(crate) struct BulletData {
    bullet_type: BulletType,
    rotation: Rot2,
    speed_tps: f32,
}

pub(crate) struct BulletEmissionDataInner {
    pub(crate) timer: IntervalTimer,
}

impl Default for BulletEmissionDataInner {
    fn default() -> Self {
        Self { timer: IntervalTimer::new(consts::TOWER_COOLDOWN_MS) }
    }
}

impl BulletEmissionDataInner {
    pub(crate) fn new(spawn_cooldown_ms: u32) -> Self {
        Self { timer: IntervalTimer::new(spawn_cooldown_ms) }
    }
}

pub(crate) fn move_bullets(
    mut q: Query<(&mut Transform, Ref<Bullet>, &CreationTime)>, time: Res<Time>,
) {
    for (mut tf, bullet, creation_time) in &mut q {
        let delta_time = if bullet.is_added() {
            creation_time.elapsed_ms(&time) as f32 / 1000.0
        } else {
            time.delta_secs()
        };
        let velocity = bullet.rotation * Vec2::X * bullet.speed_tps * delta_time;
        tf.translation += velocity.extend(0.0);

        tf.rotation = if bullet.bullet_type.get_attributes().spins {
            Quat::from_rotation_z(
                (creation_time.elapsed_ms(&time) % consts::BULLET_ROTATION_DURATION_MS) as f32
                    / consts::BULLET_ROTATION_DURATION_MS as f32
                    * PI
                    * 2.0,
            )
        } else {
            Quat::from_rotation_z(velocity.to_angle() + PI / -2.0)
        }
    }
}

pub(crate) fn handle_bullet_spawns(
    mut bullet_spawns: MessageReader<SpawnBullet>, mut commands: Commands,
    asset_server: Res<AssetServer>, texture_pack_settings: Res<TexturePackSettings>,
) {
    for message in bullet_spawns.read() {
        let stats = message.bullet_type.get_attributes();
        let mut bullet_entity = commands.spawn((
            Bullet(BulletData::new(message.bullet_type, message.direction, message.speed_tps)),
            HealthStats(HealthStatsInner::new(stats.pierce.get_value(BulletPierce))),
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
        if let Some(entity) = message.target_entity {
            bullet_entity.insert(TargetEnemy(entity));
        }
    }
}

pub(crate) fn handle_bullet_enemy_collisions(
    mut commands: Commands, mut collision_reader: MessageReader<CollisionStarted>,
    mut bullet_query: Query<(&mut HealthStats, &Bullet), Without<Enemy>>,
    mut enemy_query: Query<&mut HealthStats, With<Enemy>>,
) {
    for pair in collision_reader.read() {
        let (Ok((mut bullet_health, bullet)), Ok(mut enemy_health)) =
            (bullet_query.get_mut(pair.type_b), enemy_query.get_mut(pair.type_a))
        else {
            continue;
        };
        enemy_health
            .change_health(-bullet.bullet_type.get_attributes().damage.get_value(BulletDamage));
        if enemy_health.is_dead() {
            commands.entity(pair.type_a).try_despawn();
        }
        bullet_health.change_health(-1.0);
        if bullet_health.is_dead() {
            commands.entity(pair.type_b).try_despawn();
        }
    }
}

pub(crate) fn bullet_spawn_observer(
    trigger: On<Add, TargetEnemy>, target_enemy_query: Query<(&TargetEnemy, &Bullet)>,
    mut enemy_query: Query<&mut Enemy>,
) {
    let bullet_entity = trigger.entity;
    let Ok((target_enemy, bullet_data)) = target_enemy_query.get(bullet_entity) else {
        return;
    };
    let Ok(mut enemy) = enemy_query.get_mut(*target_enemy.deref()) else {
        return;
    };
    enemy.add_target_from_bullet(
        bullet_entity,
        bullet_data.bullet_type.get_attributes().damage.get_value(BulletDamage),
    );
}

pub(crate) fn bullet_despawn_observer(
    trigger: On<Despawn, Bullet>, target_enemy_query: Query<&TargetEnemy, With<Bullet>>,
    mut enemy_query: Query<&mut Enemy>,
) {
    let bullet_entity = trigger.entity;
    let Ok(target_enemy) = target_enemy_query.get(bullet_entity) else {
        return;
    };
    let Ok(mut enemy) = enemy_query.get_mut(*target_enemy.deref()) else {
        return;
    };
    enemy.remove_target_from(bullet_entity);
}

impl BulletData {
    pub(crate) fn new(bullet_type: BulletType, rotation: Rot2, speed_tps: f32) -> Self {
        Self { bullet_type, rotation, speed_tps }
    }
}
