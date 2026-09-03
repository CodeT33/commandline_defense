use std::f32;

use crate::components::{Enemy, Tower};
use crate::consts;
use crate::coordinates::GridCoordinate;
use crate::game_map::map_logic_parsing::EnemyPath;
use crate::resources::{MapResource, TowerRangeMap};
use bevy::math::U16Vec2;
use bevy::prelude::*;

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
