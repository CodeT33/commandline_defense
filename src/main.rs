mod bullets;
mod camera;
pub mod collision;
pub mod consts;
pub mod coordinates;
pub mod enemy;
pub mod game_cli;
pub mod game_map;
pub mod player_suite;
pub mod texture_packs;
pub mod tower;
mod ui_overlay;

use crate::bullets::{bullet_collisions, bullet_movement, rotate_towers, tower_shooting};
use crate::camera::{camera_zoom_and_pan, set_camera_position};
use crate::collision::{
    CollisionEnded, CollisionStarted, CollisionSustained, calculate_collisions,
};
use crate::enemy::{move_enemies, update_towers_in_range};
use crate::game_cli::command_event_handling::{PlaceTowerMessage, handle_command_events};
use crate::game_cli::command_line::{CommandHistory, navigate_command_history};
use crate::game_cli::command_line_state_management::{
    CommandEvent, CommandState, handle_command_line_state,
};
use crate::game_cli::spawn_game_cli;
use crate::game_map::map_rendering::spawn_map_visual_layer;
use crate::player_suite::PlayerSuiteResource;
use crate::texture_packs::TexturePackSettings;
use crate::tower::handle_tower_placing_events;
use crate::ui_overlay::debug::{DebugSettings, draw_bounding_boxes};
use crate::ui_overlay::grid::update_grid_preview;
use crate::ui_overlay::selection::{SelectionState, update_selected_tile};
use crate::ui_overlay::spawn_ui_overlay;
use bevy::input_focus::tab_navigation::TabNavigationPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use game_map::map::{MapResource, TowerRangeMap, spawn_map};

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
        .init_resource::<TowerRangeMap>()
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
                move_enemies,
                update_towers_in_range,
                rotate_towers,
                tower_shooting,
                bullet_movement,
                calculate_collisions,
                bullet_collisions,
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
    spawn_map(&mut commands, &asset_server, &texture_pack_settings);
    spawn_ui_overlay(&mut commands, &asset_server, &map_resource, &texture_pack_settings);
    spawn_game_cli(&mut commands);
    spawn_map_visual_layer(&mut commands, &asset_server, &map_resource, &texture_pack_settings);
    commands.spawn((Camera2d, IsDefaultUiCamera));
}
