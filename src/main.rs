mod bullets;
mod camera;
mod command_line;
pub mod consts;
pub mod map;

use crate::bullets::{bullet_movement, bullet_spawning};
use crate::camera::set_camera_position;
use crate::command_line::{spawn_text_input, submit_text};
use crate::map::spawn_map;
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
        .insert_resource(Time::<Fixed>::from_hz(consts::PHYSICS_FRAME_RATE as f64))
        .add_systems(Startup, (setup, set_camera_position).chain())
        .add_systems(FixedUpdate, (bullet_spawning, bullet_movement).chain())
        .add_systems(Update, (submit_text, set_camera_position))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, IsDefaultUiCamera));
    spawn_map(&mut commands, asset_server);
    spawn_text_input(&mut commands);
}
