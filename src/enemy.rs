use crate::consts;
use crate::coordinates::GridCoordinate;
use crate::ecs_elements::components::{ColliderShape, ColliderTypeA, CreationTime, Enemy, Tower};
use crate::ecs_elements::resources::{
    DebugSettings, MapResource, TexturePackSettings, TowerRangeMap,
};
use crate::game_map::map_logic_parsing::EnemyPath;
use crate::texture_packs::TexturePackAssets;
use bevy::math::U16Vec2;
use bevy::prelude::*;
use std::f32;
use std::time::Duration;

pub fn get_enemy_transform(progress: f32, path: &EnemyPath) -> Transform {
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

pub fn move_enemies(
    map_resource: Res<MapResource>, mut enemy: Query<(&mut Transform, &mut Enemy, &CreationTime)>,
    time: Res<Time>,
) {
    let path_len = map_resource.0.enemy_path.get_length();
    let path_duration_secs = path_len as f32 / consts::ENEMY_SPEED_TILES_PER_SECOND;
    let path_duration_ms = (path_duration_secs * 1000.0).round() as u64;

    for (mut transform, mut enemy, creation_time) in &mut enemy {
        let elapsed_ms = creation_time.elapsed_ms(&time);
        let progress = elapsed_ms.min(path_duration_ms) as f32 / path_duration_ms as f32;
        enemy.path_progress = progress;
        *transform = get_enemy_transform(progress, map_resource.0.enemy_path());
    }
}

pub fn update_towers_in_range(
    tower_range_map: Res<TowerRangeMap>, enemy: Query<(Entity, &Transform), With<Enemy>>,
    mut towers: Query<&mut Tower>,
) {
    for mut tower in &mut towers {
        tower.enemies_in_range.clear();
    }

    for (entity, transform) in &enemy {
        for tile in enemy_covered_tiles(
            transform.translation.truncate(),
            consts::ENEMY_SIZE_TILES,
            tower_range_map.0.size,
        ) {
            for &tower_entity in
                tower_range_map.0.towers_in_range_at(GridCoordinate { position: tile })
            {
                towers.get_mut(tower_entity).unwrap().enemies_in_range.insert(entity);
            }
        }
    }
}

fn enemy_covered_tiles(
    center: Vec2, size: Vec2, map_size: U16Vec2,
) -> impl Iterator<Item = U16Vec2> {
    let min = (center - size / 2.0).floor().max(Vec2::ZERO);
    let max = ((center + size / 2.0).ceil() - Vec2::ONE)
        .max(Vec2::ZERO)
        .min(map_size.as_vec2() - Vec2::ONE);

    let min = min.as_u16vec2();
    let max = max.as_u16vec2();
    (min.y..=max.y).flat_map(move |y| (min.x..=max.x).map(move |x| U16Vec2::new(x, y)))
}

pub fn spawn_enemies(
    mut commands: Commands, mut timer: Local<Option<Timer>>, time: Res<Time>,
    asset_server: Res<AssetServer>, texture_pack_settings: Res<TexturePackSettings>,
    debug_settings: Res<DebugSettings>,
) {
    let t = timer.get_or_insert_with(|| {
        Timer::new(
            Duration::from_millis(debug_settings.enemy_spawn_interval_ms),
            TimerMode::Repeating,
        )
    });
    if t.duration().as_millis() as u64 != debug_settings.enemy_spawn_interval_ms {
        t.set_duration(Duration::from_millis(debug_settings.enemy_spawn_interval_ms));
    }
    t.tick(time.delta());
    if !t.just_finished() {
        return;
    }
    commands.spawn((
        Enemy { path_progress: 0.0 },
        CreationTime::new(&time),
        ColliderTypeA,
        ColliderShape::circle(consts::ENEMY_RADIUS),
        Sprite {
            image: asset_server.load(
                texture_pack_settings.get_asset_path(TexturePackAssets::Towers_GatlingTower_000),
            ),
            custom_size: consts::ENEMY_SIZE_TILES.into(),
            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, consts::rendering_layers::ENTITY),
    ));
}
