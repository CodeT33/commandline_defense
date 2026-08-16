use crate::consts;
use crate::consts::MAP_SIZE_TILES;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::RngExt;

#[derive(Component)]
pub struct Jitter;

pub fn jitter_rectangle(
    mut q: Query<&mut Transform, With<Jitter>>, windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let half = Vec2::new(window.width(), window.height()) * 0.5 - consts::RECT_SIZE * 0.5;
    let mut rng = rand::rng();

    for mut tf in &mut q {
        tf.translation.x = rng.random_range(-half.x..half.x);
        tf.translation.y = rng.random_range(-half.y..half.y);
    }
}

pub fn spawn_jitter_rect(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Jitter,
        Sprite {
            image: asset_server.load(consts::paths::sprite::ANGRY_BIRB),
            custom_size: Some(consts::RECT_SIZE),
            ..default()
        },
        Transform::from_xyz(140.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_4))
            .with_scale(Vec3::splat(1.0)),
    ));
}

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
