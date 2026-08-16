mod command_line;
pub mod consts;
mod movement;

use crate::command_line::{spawn_text_input, submit_text};
use crate::movement::{jitter_rectangle, pan_camera, spawn_jitter_rect};
use bevy::input_focus::tab_navigation::TabNavigationPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: consts::WINDOW_TITLE.to_owned(),
                resolution: consts::WINDOW_RESOLUTION.into(),
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(TabNavigationPlugin)
        .insert_resource(Time::<Fixed>::from_hz(144.0))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, jitter_rectangle)
        .add_systems(Update, (pan_camera, submit_text))
        .run();
}

fn setup(
    mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>,
) {
    commands.spawn((Camera2d, IsDefaultUiCamera));
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.85, 0.20, 0.25))),
        Transform::from_xyz(-120.0, 0.0, 0.0),
    ));

    spawn_jitter_rect(&mut commands, asset_server);
    spawn_text_input(&mut commands);
}
