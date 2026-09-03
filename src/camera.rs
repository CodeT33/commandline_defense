use crate::consts;
use crate::resources::MapResource;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn set_camera_position(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    windows: Query<&Window, With<PrimaryWindow>>, map_resource: Res<MapResource>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let Ok(mut camera_transform) = camera.single_mut() else {
        return;
    };

    let map_size = Vec2::new(
        map_resource.0.map_tiles.map_size.x as f32,
        map_resource.0.map_tiles.map_size.y as f32,
    );

    let window_size = window.size();

    camera_transform.translation = Vec3::new(map_size.x / 2.0, map_size.y / 2.0, 0.0);

    camera_transform.scale =
        Vec3::splat((map_size / (window_size + Vec2::splat(0.005))).max_element());
}

pub fn camera_zoom_and_pan(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    windows: Query<&Window, With<PrimaryWindow>>, buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: MessageReader<MouseMotion>, mut mouse_wheel: MessageReader<MouseWheel>,
) {
    let Ok(mut camera_transform) = camera.single_mut() else {
        return;
    };

    let Ok(window) = windows.single() else {
        return;
    };

    let settings = consts::viewports::BASIC_CAMERA;

    // Pan

    if buttons.pressed(MouseButton::Right) {
        let mut mouse_delta = Vec2::ZERO;

        for event in mouse_motion.read() {
            mouse_delta += event.delta;
        }

        let movement = mouse_delta * camera_transform.scale.x;

        camera_transform.translation.x -= movement.x;
        camera_transform.translation.y += movement.y;
    }

    // Zooming

    let mut wheel_delta = 0.0;

    for event in mouse_wheel.read() {
        wheel_delta += event.y;
    }

    if wheel_delta == 0.0 {
        return;
    }

    // Mouse position in window

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let window_size = window.size();

    let mouse_from_center =
        Vec2::new(cursor_position.x - window_size.x / 2.0, window_size.y / 2.0 - cursor_position.y);

    // World position under mouse before zoom

    let old_zoom = camera_transform.scale.x;

    let world_before = camera_transform.translation.truncate() + mouse_from_center * old_zoom;

    // New zoom

    let zoom_factor = 1.0 - wheel_delta * settings.zoom_speed;

    let new_zoom = (old_zoom * zoom_factor).clamp(settings.min_zoom, settings.max_zoom);

    if (new_zoom - old_zoom).abs() < f32::EPSILON {
        return;
    }

    camera_transform.scale = Vec3::splat(new_zoom);

    // World position under mouse after zoom

    let world_after = camera_transform.translation.truncate() + mouse_from_center * new_zoom;

    // Correct camera

    let correction = world_before - world_after;

    camera_transform.translation += correction.extend(0.0);
}

pub struct Viewport {
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub zoom_speed: f32,
}
