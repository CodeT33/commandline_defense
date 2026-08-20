use std::f32;

use crate::consts;
use crate::map::{Enemy, MapResource, Tower, TowerRangeMap};
use bevy::math::U16Vec2;
use bevy::prelude::*;
use map_parsing::EnemyPath;

pub fn get_enemy_transform(progress: f32, path: &EnemyPath) -> Transform {
    let progress = progress.clamp(0.0, 1.0);
    let len = path.get_length();
    let target_pos = len as f32 * progress;
    let mut current_len = 0;

    for both in path.corners().windows(2) {
        let a = both[0];
        let b = both[1];

        let segment_len = (a.max(b) - a.min(b)).max_element();
        let new_len = current_len + segment_len;

        if (new_len as f32) < target_pos {
            current_len = new_len;
            continue;
        }

        let normalized_diff = (b.as_vec2() - a.as_vec2()).normalize();
        let pos =
            a.as_vec2() + (normalized_diff * (target_pos - current_len as f32)) + Vec2::splat(0.5);
        return Transform::from_translation(pos.extend(consts::rendering_layers::ENTITY))
            .with_rotation(Quat::from_rotation_z(
                normalized_diff.to_angle() - f32::consts::FRAC_PI_2,
            ));
    }
    let pos = path.corners().last().unwrap().as_vec2() + Vec2::splat(0.5);
    Transform::from_translation(pos.extend(consts::rendering_layers::ENTITY))
}

pub fn move_enemies(
    map_resource: Res<MapResource>, time: Res<Time>, mut enemy: Query<(&mut Transform, &mut Enemy)>,
) {
    let base_progress = (time.elapsed().as_millis() as u64 % consts::ENEMY_PATH_DURATION_MS) as f32
        / consts::ENEMY_PATH_DURATION_MS as f32;
    for (mut transform, mut enemy) in &mut enemy {
        let progress = (base_progress + enemy.path_offset) % 1.0;
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
            tower_range_map.size,
        ) {
            for &tower_entity in tower_range_map.towers_in_range_at(tile) {
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
