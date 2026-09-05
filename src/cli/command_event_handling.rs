use crate::cli::command_line_state_management::TogglableSettings;
use crate::coordinates::GridCoordinate;
use crate::ecs_elements::messages::{CommandEvent, PlaceTowerMessage};
use crate::ecs_elements::resources::{
    DebugSettings, MapResource, PlayerSuiteResource, SelectionState,
};
use crate::entities::tower::TowerType;
use crate::map::map_logic_parsing::TileType;
use bevy::prelude::{MessageReader, MessageWriter, Res, ResMut};

pub fn handle_command_events(
    mut messages: MessageWriter<PlaceTowerMessage>, mut events: MessageReader<CommandEvent>,
    mut selection_state: ResMut<SelectionState>, game_map: Res<MapResource>,
    player_suite: Res<PlayerSuiteResource>, mut debug_settings: ResMut<DebugSettings>,
) {
    for event in events.read() {
        match event {
            CommandEvent::Help => print_help(),
            CommandEvent::Select { tile } => select_tile(&mut selection_state, *tile),
            CommandEvent::Place { tower_type, tower_pos } => {
                place_tower(&mut messages, tower_type, tower_pos, &game_map)
            },
            CommandEvent::Clear => deselect_tile(&mut selection_state),
            CommandEvent::Balance => show_balance(&player_suite),
            CommandEvent::ExitGame => exit_game(),
            CommandEvent::Toggle(togglable) => match togglable {
                TogglableSettings::Balls => {
                    debug_settings.enable_bounding_boxes ^= true;
                },
            },
        }
    }
}

fn print_help() {
    println!("help");
}

fn select_tile(selection_state: &mut SelectionState, tile: GridCoordinate) {
    selection_state.selected_tile = Some(tile);
    println!("Selecting tile: {:?}", tile);
}

fn place_tower(
    messages: &mut MessageWriter<PlaceTowerMessage>, tower_type: &TowerType,
    tower_pos: &GridCoordinate, game_map: &Res<MapResource>,
) {
    let tile_type = game_map.0.return_tile_type(*tower_pos);

    println!("Trying to place {:?} at {:?} -> {:?}", tower_type, tower_pos, tile_type);

    if tile_type == TileType::Placeable {
        println!("Placing tower {:?} at {:?}", tower_type, tower_pos);
        messages.write(PlaceTowerMessage { tower_type: *tower_type, tower_pos: *tower_pos });
    } else {
        println!("Can't place tower {:?} at {:?} -> {:?}", tower_type, tower_pos, tile_type);
    }
}

fn deselect_tile(selection_state: &mut SelectionState) {
    selection_state.selected_tile = None;
    println!("Deselect everything");
}

fn show_balance(player_suite: &Res<PlayerSuiteResource>) {
    println!("Current balance: {:?}", player_suite.money);
}

fn exit_game() {
    println!("Exiting game");
}
