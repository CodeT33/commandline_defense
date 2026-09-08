use crate::consts;
use crate::ecs_elements::resources::MapResource;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub(crate) fn set_camera_position(
    mut camera: Query<(&mut Transform, &mut Projection), With<Camera2d>>,
    windows: Query<&Window, With<PrimaryWindow>>, map_resource: Res<MapResource>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let Ok((mut camera_transform, mut projection)) = camera.single_mut() else {
        return;
    };

    let map_size = map_resource.map_tiles().map_size().as_vec2();

    let window_size = window.size();

    camera_transform.translation = Vec3::new(map_size.x / 2.0, map_size.y / 2.0, 0.0);

    if let Projection::Orthographic(ref mut projection) = *projection {
        projection.scale = (map_size / window_size).max_element();
    }
}

pub(crate) fn camera_zoom_and_pan(
    mut camera: Query<(&mut Transform, &mut Projection), With<Camera2d>>,
    windows: Query<&Window, With<PrimaryWindow>>, buttons: Res<ButtonInput<MouseButton>>,
    mut mouse_wheel: MessageReader<MouseWheel>, mut last_cursor_pos: Local<Option<Vec2>>,
) {
    let Ok((mut camera_transform, mut projection)) = camera.single_mut() else {
        return;
    };

    let Projection::Orthographic(ref mut projection) = *projection else {
        return;
    };

    let Ok(window) = windows.single() else {
        return;
    };

    // Pan
    let Some(current_cursor_pos) = window.cursor_position() else {
        *last_cursor_pos = None;
        return;
    };
    let mouse_delta = last_cursor_pos.map(|p| current_cursor_pos - p).unwrap_or(Vec2::ZERO);
    *last_cursor_pos = Some(current_cursor_pos);

    if buttons.pressed(MouseButton::Right) {
        let movement = mouse_delta * projection.scale;

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

    let mouse_from_center = (current_cursor_pos - window.size() / 2.0) * Vec2::new(1.0, -1.0);

    // World position under mouse before zoom

    let old_zoom = projection.scale;

    let world_before = camera_transform.translation.truncate() + mouse_from_center * old_zoom;

    // New zoom

    let settings = consts::viewports::BASIC_CAMERA;

    let zoom_factor = 1.0 - wheel_delta * settings.zoom_speed;

    let new_zoom = (old_zoom * zoom_factor).clamp(settings.min_zoom, settings.max_zoom);

    if (new_zoom - old_zoom).abs() < f32::EPSILON {
        return;
    }

    projection.scale = new_zoom;

    // camera_transform.scale = Vec3::splat(new_zoom);

    // World position under mouse after zoom

    let world_after = camera_transform.translation.truncate() + mouse_from_center * new_zoom;

    // Correct camera

    let correction = world_before - world_after;

    camera_transform.translation += correction.extend(0.0);
}

pub(crate) struct Viewport {
    pub(crate) min_zoom: f32,
    pub(crate) max_zoom: f32,
    pub(crate) zoom_speed: f32,
}
