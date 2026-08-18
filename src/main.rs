mod bullets;
mod camera;
mod command_line;
pub mod consts;
pub mod enemy;
pub mod grid;
pub mod map;

use crate::bullets::{bullet_collisions, bullet_movement, rotate_towers, tower_shooting};
use crate::camera::set_camera_position;
use crate::command_line::{spawn_text_input, submit_text};
use crate::enemy::{move_enemies, update_towers_in_range};
use crate::grid::spawn_grid;
use crate::map::{TowerRangeMap, spawn_map};
use avian2d::prelude::{PhysicsPlugins, PhysicsSystems};
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
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Time::<Fixed>::from_hz(consts::PHYSICS_FRAME_RATE as f64))
        .insert_resource(TowerRangeMap::default())
        .add_systems(Startup, (setup, set_camera_position).chain())
        .add_systems(
            FixedUpdate,
            (move_enemies, update_towers_in_range, rotate_towers, tower_shooting, bullet_movement)
                .chain(),
        )
        .add_systems(FixedPostUpdate, bullet_collisions.after(PhysicsSystems::StepSimulation))
        .add_systems(Update, (submit_text, set_camera_position))
        .run();
}

fn setup(
    mut commands: Commands, asset_server: Res<AssetServer>, tower_range_map: ResMut<TowerRangeMap>,
) {
    commands.spawn((Camera2d, IsDefaultUiCamera));
    spawn_map(&mut commands, asset_server, tower_range_map);
    spawn_grid(&mut commands);
    spawn_text_input(&mut commands);
}
