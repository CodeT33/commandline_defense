use crate::consts;
use crate::map::{Enemy, MapResource};
use bevy::prelude::*;
use map_parsing::EnemyPath;

pub fn get_enemy_pos(progress: f32, path: &EnemyPath) -> Vec2 {
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

        return a.as_vec2()
            + ((b.as_vec2() - a.as_vec2()).normalize() * (target_pos - current_len as f32));
    }
    path.corners().last().unwrap().as_vec2()
}

pub fn move_enemies(
    map_resource: Res<MapResource>, time: Res<Time>, mut enemy: Query<&mut Transform, With<Enemy>>,
) {
    let progress = (time.elapsed().as_millis() as u64 % consts::ENEMY_PATH_DURATION_MS) as f32
        / consts::ENEMY_PATH_DURATION_MS as f32;
    for mut transform in &mut enemy {
        let mut pos = get_enemy_pos(progress, map_resource.0.enemy_path());
        pos.y = (map_resource.0.map_tiles().map_size().y - 1) as f32 - pos.y;
        pos += vec2(0.5, 0.5);
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}
