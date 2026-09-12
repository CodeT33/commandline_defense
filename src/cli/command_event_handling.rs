use crate::cli::command_input::{FurtherInfo, OpenCommand, Settings};
use crate::coordinates::GridCoordinate;
use crate::ecs_elements::messages::{CommandEvent, PlaceTowerMessage};
use crate::ecs_elements::resources::{
    CommandState, DebugSettings, GameState, MapResource, SelectionState,
};
use crate::entities::tower::TowerType;
use crate::map::map_logic_parsing::TileType;
use crate::ui_overlay::ui_state::UiState;
use bevy::prelude::{MessageReader, MessageWriter, Res, ResMut};

pub(crate) fn handle_command_events(
    mut messages: MessageWriter<PlaceTowerMessage>, mut events: MessageReader<CommandEvent>,
    mut selection_state: ResMut<SelectionState>, game_map: Res<MapResource>,
    mut debug_settings: ResMut<DebugSettings>, mut command_state: ResMut<CommandState>,
    mut game_state: ResMut<GameState>,
) {
    for event in events.read().copied() {
        match event {
            CommandEvent::Select { tile } => select_tile(&mut selection_state, tile),
            CommandEvent::Place { tower_type, tower_pos } => {
                place_tower(&mut messages, tower_type, tower_pos, &game_map)
            },
            CommandEvent::Clear => {
                deselect_tile(&mut selection_state);
                command_state.persistent_preview = None;
            },
            CommandEvent::Pause => {
                if !game_state.waves.is_paused() {
                    debug_settings.paused = true;
                }
            },
            CommandEvent::Resume => {
                if game_state.waves.is_paused() {
                    game_state.waves.resume()
                }
                debug_settings.paused = false;
            },
            CommandEvent::ExitGame => exit_game(),
            CommandEvent::Set(setting) => match setting {
                Settings::BoundingBoxes { value } => {
                    debug_settings.enable_bounding_boxes = value;
                },
                Settings::SimSpeed { value } => {
                    debug_settings.sim_speed = value;
                },
                Settings::SpawnInterval { value } => {
                    debug_settings.enemy_spawn_interval_ms = value as u64;
                },
                Settings::EnemyType { enemy_type } => debug_settings.enemy_type = enemy_type,
            },
            CommandEvent::Open(open_thing) => {
                command_state.persistent_preview = match open_thing {
                    OpenCommand::Info { further: None } => UiState::Menus,
                    OpenCommand::Info { further: Some(further) } => match further {
                        FurtherInfo::Enemies { enemy_type, further } => {
                            UiState::EnemiesList { selected: enemy_type, further_details: further }
                        },
                        FurtherInfo::Towers { tower_type, further } => {
                            UiState::TowersList { selected: tower_type, further_details: further }
                        },
                    },
                }
                .into()
            },
        }
    }
}

fn select_tile(selection_state: &mut SelectionState, tile: GridCoordinate) {
    selection_state.selected_tile = Some(tile);
    println!("Selecting tile: {:?}", tile);
}

fn place_tower(
    messages: &mut MessageWriter<PlaceTowerMessage>, tower_type: TowerType,
    tower_pos: GridCoordinate, game_map: &Res<MapResource>,
) {
    let tile_type = game_map.return_tile_type(tower_pos);

    println!("Trying to place {:?} at {:?} -> {:?}", tower_type, tower_pos, tile_type);

    if tile_type == TileType::Placeable {
        println!("Placing tower {:?} at {:?}", tower_type, tower_pos);
        messages.write(PlaceTowerMessage { tower_type, tower_pos });
    } else {
        println!("Can't place tower {:?} at {:?} -> {:?}", tower_type, tower_pos, tile_type);
    }
}

fn deselect_tile(selection_state: &mut SelectionState) {
    selection_state.selected_tile = None;
    println!("Deselect everything");
}

fn exit_game() {
    println!("Exiting game");
}
