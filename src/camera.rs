use crate::consts::MAP_SIZE_TILES;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[allow(clippy::type_complexity)]
pub fn set_camera_position(
    mut camera: Query<&mut Transform, With<Camera2d>>, windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Some(window_size) = windows.single().ok().map(|w| w.size()) else {
        return;
    };

    let Ok(mut tf) = camera.single_mut() else {
        return;
    };

    tf.translation = Vec3::new(MAP_SIZE_TILES[0] as f32 / 2.0, MAP_SIZE_TILES[1] as f32 / 2.0, 0.0);
    tf.scale = Vec3::splat((Vec2::from(MAP_SIZE_TILES.map(f32::from)) / window_size).max_element());
}
