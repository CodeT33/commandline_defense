use std::f32;

use crate::consts;
use crate::map::{Enemy, MapResource};
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
        return Transform::from_translation(pos.extend(0.0)).with_rotation(Quat::from_rotation_z(
            normalized_diff.to_angle() - f32::consts::FRAC_PI_2,
        ));
    }
    let pos = path.corners().last().unwrap().as_vec2() + Vec2::splat(0.5);
    Transform::from_translation(pos.extend(0.0))
}

pub fn move_enemies(
    map_resource: Res<MapResource>, time: Res<Time>, mut enemy: Query<(&mut Transform, &Enemy)>,
) {
    let base_progress = (time.elapsed().as_millis() as u64 % consts::ENEMY_PATH_DURATION_MS) as f32
        / consts::ENEMY_PATH_DURATION_MS as f32;
    for (mut transform, enemy) in &mut enemy {
        let progress = (base_progress + enemy.path_offset) % 1.0;
        *transform =
            get_enemy_transform(progress, map_resource.0.enemy_path()).with_scale(transform.scale);
    }
}
