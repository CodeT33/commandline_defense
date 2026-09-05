mod camera;
pub mod cli;
pub mod collision;
pub mod consts;
pub mod coordinates;
pub mod ecs_elements;
pub mod entities;
pub mod map;
pub mod player_suite;
pub mod scheduling;
pub mod texture_packs;
mod ui_overlay;

use crate::camera::{camera_zoom_and_pan, set_camera_position};
use crate::cli::command_event_handling::handle_command_events;
use crate::cli::command_line::navigate_command_history;
use crate::cli::command_line_state_management::handle_command_line_state;
use crate::cli::spawn_game_cli;
use crate::collision::calculate_collisions;
use crate::map::map_rendering::spawn_map_visual_layer;
use crate::ui_overlay::debug::draw_bounding_boxes;
use crate::ui_overlay::grid::update_grid_preview;
use crate::ui_overlay::selection::update_selected_tile;
use crate::ui_overlay::spawn_ui_overlay;
use bevy::input_focus::tab_navigation::TabNavigationPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use ecs_elements::messages::{
    CollisionEnded, CollisionStarted, CollisionSustained, CommandEvent, PlaceTowerMessage,
};
use ecs_elements::resources::{
    CommandHistory, CommandState, DebugSettings, MapResource, PlayerSuiteResource, SelectionState,
    TexturePackSettings,
};
use entities::bullets::{
    handle_bullet_enemy_collisions, move_bullets, rotate_towers, spawn_bullets,
};
use entities::enemies::{move_enemies, spawn_enemies, update_towers_in_range};
use entities::tower::handle_tower_placing_events;

fn main() {
    let mut app = App::new();
    register_plugins(&mut app);
    register_resources(&mut app);
    register_messages(&mut app);
    register_systems(&mut app);
    app.run();
}

fn register_plugins(app: &mut App) {
    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: consts::WINDOW_TITLE.to_owned(),
                    resolution: consts::WINDOW_RESOLUTION.into(),
                    present_mode: PresentMode::Immediate,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin { file_path: "./".to_owned(), ..default() }),
        TabNavigationPlugin,
    ));
}

fn register_resources(app: &mut App) {
    app.init_resource::<CommandState>()
        .init_resource::<DebugSettings>()
        .init_resource::<SelectionState>()
        .init_resource::<TexturePackSettings>()
        .init_resource::<MapResource>()
        .insert_resource(Time::<Fixed>::from_hz(consts::PHYSICS_FRAME_RATE as f64))
        .init_resource::<CommandHistory>()
        .init_resource::<PlayerSuiteResource>();
}

fn register_messages(app: &mut App) {
    app.add_message::<CommandEvent>()
        .add_message::<PlaceTowerMessage>()
        .add_message::<CollisionStarted>()
        .add_message::<CollisionSustained>()
        .add_message::<CollisionEnded>();
}

fn register_systems(app: &mut App) {
    app.add_systems(Startup, (setup, set_camera_position).chain())
        .add_systems(
            // physics
            FixedUpdate,
            (
                // movement
                (move_enemies, move_bullets),
                // collision handling
                calculate_collisions,
                handle_bullet_enemy_collisions,
                update_towers_in_range,
                // rest
                rotate_towers,
                (spawn_bullets, spawn_enemies),
            )
                .chain(),
        )
        .add_systems(
            // display
            Update,
            (
                draw_bounding_boxes.run_if(|debug_settings: Res<DebugSettings>| {
                    debug_settings.enable_bounding_boxes
                }),
                camera_zoom_and_pan,
                update_grid_preview,
                update_selected_tile,
                handle_command_events,
                handle_command_line_state,
                navigate_command_history,
                handle_tower_placing_events,
            ),
        );
}

fn setup(
    mut commands: Commands, asset_server: Res<AssetServer>, map_resource: Res<MapResource>,
    texture_pack_settings: Res<TexturePackSettings>,
) {
    spawn_ui_overlay(&mut commands, &asset_server, &map_resource, &texture_pack_settings);
    spawn_game_cli(&mut commands);
    spawn_map_visual_layer(&mut commands, &asset_server, &map_resource, &texture_pack_settings);
    commands.spawn((Camera2d, IsDefaultUiCamera));
}
