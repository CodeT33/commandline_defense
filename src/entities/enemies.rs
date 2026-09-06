use crate::consts;
use crate::ecs_elements::components::{
    ColliderShape, ColliderTypeA, CreationTime, Enemy, HealthStats,
};
use crate::ecs_elements::messages::SpawnEnemy;
use crate::ecs_elements::resources::{DebugSettings, MapResource, TexturePackSettings};
use crate::entities::health::HealthStatsInner;
use crate::map::map_logic_parsing::EnemyPath;
use crate::scheduling::IntervalTimer;
use crate::texture_packs::TexturePackAssets;
use bevy::prelude::*;
use std::f32;

#[derive(Copy, Clone, Debug)]
pub enum EnemyType {
    WideBirb,
    Mausmeister,
}

#[allow(unused)]
pub struct EnemyData {
    enemy_type: EnemyType,
    path_progress: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct EnemyStats {
    pub health: f32,
    pub speed_tps: f32,
    pub asset: TexturePackAssets,
}

impl EnemyType {
    pub fn get_stats(self) -> EnemyStats {
        match self {
            EnemyType::WideBirb => consts::enemies::ENEMY_TYPE_WIDE_BIRB,
            EnemyType::Mausmeister => consts::enemies::ENEMY_TYPE_MAUS_MEISTER,
        }
    }
}

pub fn move_enemies(
    map_resource: Res<MapResource>, mut enemy: Query<(&mut Transform, &mut Enemy, &CreationTime)>,
    time: Res<Time>,
) {
    let path_len = map_resource.0.enemy_path.get_length();
    let path_duration_secs = path_len as f32 / consts::ENEMY_SPEED_TILES_PER_SECOND;
    let path_duration_ms = (path_duration_secs * 1000.0).round() as u64;

    for (mut transform, mut enemy, creation_time) in &mut enemy {
        let elapsed_ms = creation_time.0.elapsed_ms(&time);
        let progress = elapsed_ms.min(path_duration_ms) as f32 / path_duration_ms as f32;
        enemy.0.path_progress = progress;
        *transform = get_enemy_transform(progress, map_resource.0.enemy_path());
    }
}

pub fn request_enemy_spawns(
    mut enemy_spawns: MessageWriter<SpawnEnemy>, mut timer: Local<Option<IntervalTimer>>,
    time: Res<Time>, debug_settings: Res<DebugSettings>,
) {
    let t = timer
        .get_or_insert_with(|| IntervalTimer::new(debug_settings.enemy_spawn_interval_ms as u32));
    if t.get_interval_ms() as u64 != debug_settings.enemy_spawn_interval_ms {
        t.set_interval_ms(debug_settings.enemy_spawn_interval_ms as u32);
    }

    while let Some(tick_time) = t.tick_if_ready(&time) {
        enemy_spawns.write(SpawnEnemy { enemy_type: EnemyType::Mausmeister, time: tick_time });
    }
}

pub fn handle_enemy_spawns(
    mut enemy_spawns: MessageReader<SpawnEnemy>, mut commands: Commands,
    asset_server: Res<AssetServer>, texture_pack_settings: Res<TexturePackSettings>,
) {
    for message in enemy_spawns.read() {
        commands.spawn((
            Enemy(EnemyData::new(message.enemy_type)),
            HealthStats(HealthStatsInner::new(message.enemy_type.get_stats().health)),
            CreationTime(message.time),
            ColliderTypeA,
            ColliderShape::circle(consts::ENEMY_BOUNDING_CIRCLE_RADIUS),
            Sprite {
                image: asset_server.load(
                    texture_pack_settings.get_asset_path(message.enemy_type.get_stats().asset),
                ),
                custom_size: consts::ENEMY_SPRITE_SIZE_TILES.into(),
                image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, consts::rendering_layers::ENTITY),
        ));
    }
}

fn get_enemy_transform(progress: f32, path: &EnemyPath) -> Transform {
    let progress = progress.clamp(0.0, 1.0);

    let Some(start) = path.corners().first() else {
        return Transform::IDENTITY;
    };

    if path.corners().len() < 2 {
        let position = start.position.as_vec2() + Vec2::splat(0.5);

        return Transform::from_translation(position.extend(consts::rendering_layers::ENTITY));
    }

    let path_length = path.get_length() as f32;
    let target_distance = path_length * progress;

    let mut current_len = 0.0;

    for both in path.corners().windows(2) {
        let a = both[0];
        let b = both[1];

        let segment_len = (a.position.max(b.position) - a.position.min(b.position)).max_element();
        let new_len = current_len + segment_len as f32;

        if (new_len) < target_distance {
            current_len = new_len;
            continue;
        }

        let normalized_diff = (b.position.as_vec2() - a.position.as_vec2()).normalize();
        let pos = a.position.as_vec2()
            + (normalized_diff * (target_distance - current_len))
            + Vec2::splat(0.5);
        return Transform::from_translation(pos.extend(consts::rendering_layers::ENTITY))
            .with_rotation(Quat::from_rotation_z(
                normalized_diff.to_angle() - f32::consts::FRAC_PI_2,
            ));
    }
    let pos = path.corners().last().unwrap().position.as_vec2() + Vec2::splat(0.5);
    Transform::from_translation(pos.extend(consts::rendering_layers::ENTITY))
}

impl EnemyData {
    pub fn new(enemy_type: EnemyType) -> Self {
        Self { enemy_type, path_progress: 0.0 }
    }

    pub fn get_path_progress(&self) -> f32 {
        self.path_progress
    }
}
