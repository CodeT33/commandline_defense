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
            + ((b.as_vec2() - a.as_vec2()).normalize() * (target_pos - current_len as f32))
            + vec2(0.5, 0.5);
    }
    path.corners().last().unwrap().as_vec2() + vec2(0.5, 0.5)
}
