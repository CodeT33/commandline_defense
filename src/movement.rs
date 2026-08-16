use bevy::input_focus::InputFocus;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::RngExt;

const RECT_SIZE: Vec2 = Vec2::new(140.0, 80.0);

#[derive(Component)]
pub struct Jitter;

pub fn jitter_rectangle(
    mut q: Query<&mut Transform, With<Jitter>>, windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let half = Vec2::new(window.width(), window.height()) * 0.5 - RECT_SIZE * 0.5;
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
            image: asset_server.load("sprites/uzsg4bc3e2mg1.png"),
            custom_size: Some(RECT_SIZE),
            ..default()
        },
        Transform::from_xyz(140.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_4))
            .with_scale(Vec3::splat(1.0)),
    ));
}

pub fn pan_camera(
    time: Res<Time>, keys: Res<ButtonInput<KeyCode>>, focus: Res<InputFocus>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    if focus.get().is_some() {
        return;
    }

    let Ok(mut tf) = camera.single_mut() else {
        return;
    };

    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }

    if let Some(dir) = dir.try_normalize() {
        tf.translation += (dir * 400.0 * time.delta_secs()).extend(0.0);
    }
}
