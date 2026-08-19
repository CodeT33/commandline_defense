use crate::game_cli::command_line_state_management::CommandEvent;
use crate::ui_overlay::selection::SelectionState;
use avian2d::parry::glamx::Vec2;
use bevy::prelude::{MessageReader, ResMut};

pub fn handle_command_events(
    mut events: MessageReader<CommandEvent>, mut selection_state: ResMut<SelectionState>,
) {
    for event in events.read() {
        match event {
            CommandEvent::Help => print_help(),
            CommandEvent::Select { tile } => select_tile(&mut selection_state, *tile),
            CommandEvent::Deselect => deselect_tile(&mut selection_state),
            CommandEvent::ExitGame => exit_game(),
        }
    }
}

fn print_help() {
    println!("help");
}

fn select_tile(selection_state: &mut SelectionState, tile: Vec2) {
    selection_state.selected_tile = Some(tile);
    println!("Selected tile: {:?}", tile);
}

fn deselect_tile(selection_state: &mut SelectionState) {
    selection_state.selected_tile = None;
    println!("Deselected everything");
}

fn exit_game() {
    println!("Exit game");
}
