mod bullets;
mod camera;
pub mod consts;
pub mod enemy;
pub mod game_cli;
pub mod map;
mod ui_overlay;

use crate::bullets::{bullet_collisions, bullet_movement, rotate_towers, tower_shooting};
use crate::camera::set_camera_position;
use crate::enemy::{move_enemies, update_towers_in_range};
use crate::game_cli::command_event_handling::handle_command_events;
use crate::game_cli::command_line::{CommandHistory, navigate_command_history, spawn_command_line};
use crate::game_cli::command_line_state_management::{
    CommandEvent, CommandState, handle_command_line_state,
};
use crate::game_cli::spawn_game_cli;
use crate::map::{TowerRangeMap, spawn_map};
use crate::ui_overlay::grid::{spawn_grid, update_grid_preview};
use crate::ui_overlay::selection::{SelectionState, update_selected_tile};
use crate::ui_overlay::spawn_ui_overlay;
use avian2d::prelude::{PhysicsPlugins, PhysicsSystems};
use bevy::input_focus::tab_navigation::TabNavigationPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;

fn main() {
    App::new()
        //plugins
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
        //resources
        .init_resource::<CommandState>()
        .init_resource::<SelectionState>()
        .insert_resource(Time::<Fixed>::from_hz(consts::PHYSICS_FRAME_RATE as f64))
        .insert_resource(TowerRangeMap::default())
        .insert_resource(CommandHistory::default())
        //messages
        .add_message::<CommandEvent>()
        //systems
        .add_systems(Startup, (setup, set_camera_position).chain())
        .add_systems(
            FixedUpdate,
            (move_enemies, update_towers_in_range, rotate_towers, tower_shooting, bullet_movement)
                .chain(),
        )
        .add_systems(FixedPostUpdate, bullet_collisions.after(PhysicsSystems::StepSimulation))
        .add_systems(
            Update,
            (
                update_grid_preview,
                update_selected_tile,
                handle_command_events,
                handle_command_line_state,
                navigate_command_history,
                set_camera_position,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands, asset_server: Res<AssetServer>, tower_range_map: ResMut<TowerRangeMap>,
) {
    spawn_map(&mut commands, asset_server, tower_range_map);
    spawn_ui_overlay(&mut commands);
    spawn_game_cli(&mut commands);
    commands.spawn((Camera2d, IsDefaultUiCamera));
}
