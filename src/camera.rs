use crate::game_map::map::MapResource;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[allow(clippy::type_complexity)]
pub fn set_camera_position(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    windows: Query<&Window, With<PrimaryWindow>>, map_resource: Res<MapResource>,
) {
    let map_size = Vec2::new(
        map_resource.0.map_tiles.map_size.x as f32,
        map_resource.0.map_tiles.map_size.y as f32,
    );

    let Some(window_size) = windows.single().ok().map(|w| w.size()) else {
        return;
    };

    let Ok(mut camera_transform) = camera.single_mut() else {
        return;
    };
    camera_transform.translation = Vec3::new(map_size.x / 2.0, map_size.y / 2.0, 0.0);
    camera_transform.scale = Vec3::splat(
        (map_size.map(f32::from) / Vec2 { x: window_size.x + 0.005, y: window_size.y + 0.005 })
            .max_element(),
    );
}
