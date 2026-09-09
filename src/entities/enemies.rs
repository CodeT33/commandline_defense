use crate::consts;
use crate::ecs_elements::components::{
    ColliderShape, ColliderTypeA, CreationTime, Enemy, HealthStats,
};
use crate::ecs_elements::events::PlayerHasDied;
use crate::ecs_elements::messages::{EnemyReachedEnd, SpawnEnemy};
use crate::ecs_elements::resources::{
    DebugSettings, MapResource, PlayerSuiteResource, TexturePackSettings,
};
use crate::entities::health::HealthStatsInner;
use crate::map::map_logic_parsing::EnemyPath;
use crate::scheduling::IntervalTimer;
use crate::texture_packs::TexturePackAssets;
use bevy::ecs::entity::EntityHashMap;
use bevy::prelude::*;
use std::f32;

#[allow(unused)]
#[derive(Copy, Clone, Debug)]
pub(crate) enum EnemyType {
    WideBirb,
    Mausmeister,
    Zapano,
    Rocher,
}

pub(crate) struct EnemyData {
    enemy_type: EnemyType,
    path_progress: f32,
    targeted_by: EntityHashMap<f32>,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct EnemyStats {
    pub(crate) health: f32,
    pub(crate) player_health_penalty: u16,
    pub(crate) speed_tps: f32,
    pub(crate) relative_collider_size: f32,
    pub(crate) texture_size_tiles: f32,
    pub(crate) asset: TexturePackAssets,
}

pub(crate) fn move_enemies(
    map_resource: Res<MapResource>,
    mut enemy: Query<(Entity, &mut Transform, &mut Enemy, &CreationTime)>,
    mut reached_end_writer: MessageWriter<EnemyReachedEnd>, time: Res<Time>,
) {
    let path_len = map_resource.enemy_path().get_length();

    for (entity, mut transform, mut enemy, creation_time) in &mut enemy {
        let path_duration_secs = path_len as f32 / enemy.enemy_type.get_stats().speed_tps;
        let path_duration_ms = (path_duration_secs * 1000.0).round() as u64;
        let elapsed_ms = creation_time.elapsed_ms(&time);
        let progress = elapsed_ms.min(path_duration_ms) as f32 / path_duration_ms as f32;
        enemy.path_progress = progress;
        if progress == 1.0 {
            reached_end_writer.write(EnemyReachedEnd(entity));
        }
        *transform = get_enemy_transform(progress, map_resource.enemy_path());
    }
}

pub(crate) fn request_enemy_spawns(
    mut enemy_spawns: MessageWriter<SpawnEnemy>, mut timer: Local<Option<IntervalTimer>>,
    time: Res<Time>, debug_settings: Res<DebugSettings>,
) {
    let t = timer
        .get_or_insert_with(|| IntervalTimer::new(debug_settings.enemy_spawn_interval_ms as u32));
    if t.get_interval_ms() as u64 != debug_settings.enemy_spawn_interval_ms {
        t.set_interval_ms(debug_settings.enemy_spawn_interval_ms as u32);
    }

    while let Some(tick_time) = t.tick_if_ready(&time) {
        enemy_spawns.write(SpawnEnemy { enemy_type: EnemyType::Rocher, time: tick_time });
    }
}

pub(crate) fn handle_enemy_spawns(
    mut enemy_spawns: MessageReader<SpawnEnemy>, mut commands: Commands,
    asset_server: Res<AssetServer>, texture_pack_settings: Res<TexturePackSettings>,
    map_resource: Res<MapResource>,
) {
    for message in enemy_spawns.read() {
        let stats = message.enemy_type.get_stats();
        commands.spawn((
            Enemy(EnemyData::new(message.enemy_type)),
            HealthStats(HealthStatsInner::new(stats.health)),
            CreationTime(message.time),
            ColliderTypeA,
            ColliderShape::circle(stats.texture_size_tiles * stats.relative_collider_size / 2.0),
            Sprite {
                image: asset_server.load(texture_pack_settings.get_asset_path(stats.asset)),
                custom_size: Some(Vec2::splat(stats.texture_size_tiles)),
                image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
                ..default()
            },
            get_enemy_transform(0.0, map_resource.enemy_path()),
        ));
    }
}

pub(crate) fn get_enemy_transform(progress: f32, path: &EnemyPath) -> Transform {
    let progress = progress.clamp(0.0, 1.0);

    let Some(start) = path.corners().first() else {
        return Transform::from_translation(Vec2::ZERO.extend(consts::rendering_layers::ENTITY));
    };

    if path.corners().len() < 2 {
        let position = start.position().0.as_vec2() + Vec2::splat(0.5);

        return Transform::from_translation(position.extend(consts::rendering_layers::ENTITY));
    }

    let path_length = path.get_length() as f32;
    let target_distance = path_length * progress;

    let mut current_len = 0.0;

    for both in path.corners().windows(2) {
        let a = both[0];
        let b = both[1];

        let segment_len =
            (a.position().max(*b.position()) - a.position().min(*b.position())).max_element();
        let new_len = current_len + segment_len as f32;

        if (new_len) < target_distance {
            current_len = new_len;
            continue;
        }

        let normalized_diff = (b.position().as_vec2() - a.position().as_vec2()).normalize();
        let pos = a.position().as_vec2()
            + (normalized_diff * (target_distance - current_len))
            + Vec2::splat(0.5);
        return Transform::from_translation(pos.extend(consts::rendering_layers::ENTITY))
            .with_rotation(Quat::from_rotation_z(
                normalized_diff.to_angle() - f32::consts::FRAC_PI_2,
            ));
    }
    let pos = path.corners().last().unwrap().position().as_vec2() + Vec2::splat(0.5);
    Transform::from_translation(pos.extend(consts::rendering_layers::ENTITY))
}

impl EnemyData {
    pub(crate) fn new(enemy_type: EnemyType) -> Self {
        Self { enemy_type, path_progress: 0.0, targeted_by: Default::default() }
    }

    pub(crate) fn get_path_progress(&self) -> f32 {
        self.path_progress
    }

    pub(crate) fn get_type(&self) -> EnemyType {
        self.enemy_type
    }

    pub(crate) fn remove_target_from(&mut self, bullet: Entity) {
        self.targeted_by.remove(&bullet);
    }

    pub(crate) fn add_target_from_bullet(&mut self, bullet: Entity, damage: f32) {
        self.targeted_by.insert(bullet, damage);
    }

    pub(crate) fn get_planned_bullet_damage(&self) -> f32 {
        self.targeted_by.values().sum()
    }
}

pub(crate) fn handle_enemies_reaching_end(
    mut reached_end: MessageReader<EnemyReachedEnd>, enemies: Query<&Enemy>,
    mut player: ResMut<PlayerSuiteResource>, mut commands: Commands,
) {
    for (entity, enemy) in
        reached_end.read().filter_map(|e| enemies.get(e.0).ok().map(|d| (e.0, d)))
    {
        commands.entity(entity).try_despawn();
        player.health =
            player.health.saturating_sub(enemy.enemy_type.get_stats().player_health_penalty);
    }
    if player.health == 0 {
        commands.trigger(PlayerHasDied);
    }
}
