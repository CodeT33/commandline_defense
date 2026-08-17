mod command_line;
pub mod consts;
pub mod map;
mod movement;
pub mod grid;

use crate::command_line::{spawn_text_input, submit_text};
use crate::map::spawn_map;
use crate::movement::{jitter_rectangle, set_camera_position};
use bevy::input_focus::tab_navigation::TabNavigationPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use crate::grid::spawn_grid;

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
        .add_systems(Startup, (setup, set_camera_position).chain())
        .add_systems(FixedUpdate, jitter_rectangle)
        .add_systems(Update, (submit_text, set_camera_position))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, IsDefaultUiCamera));
    spawn_map(&mut commands, asset_server);
    spawn_grid(commands);
    //spawn_text_input(&mut commands);
}
