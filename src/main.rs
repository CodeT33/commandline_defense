mod camera;
pub mod cli;
pub mod collision;
pub mod consts;
pub mod coordinates;
#[cfg(feature = "determinism")]
pub mod determinism_harness;
pub mod ecs_elements;
pub mod entities;
pub mod map;
pub mod movement;
pub mod player_suite;
pub mod scheduling;
pub mod texture_packs;
mod ui_overlay;

use crate::camera::{camera_zoom_and_pan, set_camera_position};
use crate::cli::command_event_handling::handle_command_events;
use crate::cli::command_line::navigate_command_history;
use crate::cli::command_line_state_management::{Settings, handle_command_line_state};
use crate::cli::spawn_game_cli;
use crate::collision::calculate_collisions;
use crate::coordinates::GridCoordinate;
use crate::entities::enemies::handle_enemy_spawns;
use crate::entities::tower::TowerType;
use crate::map::map_rendering::spawn_map_visual_layer;
use crate::map::spawn_map_bounds;
use crate::movement::delete_out_of_map_entities;
use crate::ui_overlay::debug::draw_bounding_boxes;
use crate::ui_overlay::grid::update_grid_preview;
use crate::ui_overlay::health_bars::draw_health_bars;
use crate::ui_overlay::selection::update_selected_tile;
use crate::ui_overlay::spawn_ui_overlay;
use bevy::input_focus::tab_navigation::TabNavigationPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_vector_shapes::prelude::*;
use ecs_elements::messages::{
    CollisionEnded, CollisionStarted, CollisionSustained, CommandEvent, PlaceTowerMessage,
    SpawnBullet, SpawnEnemy,
};
use ecs_elements::resources::{
    CommandHistory, CommandState, DebugSettings, MapResource, PlayerSuiteResource, SelectionState,
    TexturePackSettings,
};
use entities::bullets::{
    handle_bullet_enemy_collisions, handle_bullet_spawns, move_bullets, request_bullet_spawns,
    update_towers_in_range_and_rotate,
};
use entities::enemies::{move_enemies, request_enemy_spawns};
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
        Shape2dPlugin::default(),
    ));
    #[cfg(feature = "determinism")]
    app.add_plugins(crate::determinism_harness::DeterminismHarnessPlugin);
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
        .add_message::<SpawnEnemy>()
        .add_message::<SpawnBullet>()
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
                delete_out_of_map_entities,
                calculate_collisions,
                handle_bullet_enemy_collisions,
                update_towers_in_range_and_rotate,
                (
                    (request_bullet_spawns, handle_bullet_spawns).chain(),
                    (request_enemy_spawns, handle_enemy_spawns).chain(),
                ),
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
                draw_health_bars,
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
    mut command_messages: MessageWriter<CommandEvent>,
) {
    spawn_ui_overlay(&mut commands, &asset_server, &map_resource, &texture_pack_settings);
    spawn_game_cli(&mut commands);
    spawn_map_visual_layer(&mut commands, &asset_server, &map_resource, &texture_pack_settings);
    commands.spawn((
        Camera2d,
        IsDefaultUiCamera,
        Projection::Orthographic(OrthographicProjection::default_2d()),
    ));
    spawn_map_bounds(&mut commands, &map_resource);
    command_messages.write(CommandEvent::Set { setting: Settings::BoundingBoxes, value: 1.0 });
    command_messages.write(CommandEvent::Place {
        tower_type: TowerType::BoomTower,
        tower_pos: GridCoordinate::new(9, 3),
    });
}
