use crate::consts;
use crate::consts::MAP_SIZE_TILES;
use crate::map::{Bullet, BulletSpawnData, Tower};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct SpawnTime(pub u64);

pub fn bullet_movement(mut q: Query<(&mut Transform, &Velocity, &SpawnTime)>, time: Res<Time>) {
    for (mut tf, velocity, spawn_time) in &mut q {
        let velocity = velocity.0 * 1.0 / consts::PHYSICS_FRAME_RATE as f32;
        tf.translation.x += velocity.x;
        tf.translation.y += velocity.y;
        tf.rotation = Quat::from_rotation_z(
            ((time.elapsed().as_millis() as u64 - spawn_time.0)
                % consts::BULLET_ROTATION_DURATION_MS) as f32
                / consts::BULLET_ROTATION_DURATION_MS as f32
                * PI
                * 2.0,
        )
    }
}

pub fn bullet_spawning(
    mut commands: Commands, mut q: Query<(&Transform, &mut BulletSpawnData), With<Tower>>,
    time: Res<Time>, asset_server: Res<AssetServer>,
) {
    for (transform, mut data) in &mut q {
        while let Some(shoot_time) = data.shoot_if_ready(time.elapsed().as_millis() as u64) {
            commands.spawn((
                Bullet,
                Velocity(data.direction * Vec2::X),
                Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0),
                Sprite {
                    image: asset_server.load(consts::paths::sprite::APPLE),
                    custom_size: consts::PROJECTILE_SIZE_TILES.into(),
                    ..default()
                },
                SpawnTime(shoot_time),
            ));
        }
    }
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
