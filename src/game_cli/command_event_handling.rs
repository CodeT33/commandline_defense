use crate::game_cli::command_line_state_management::CommandEvent;
use crate::tower::TowerType;
use crate::ui_overlay::selection::SelectionState;
use avian2d::parry::glamx::Vec2;
use bevy::asset::AsyncWriteExt;
use bevy::math::U16Vec2;
use bevy::prelude::{Message, MessageReader, MessageWriter, ResMut};

#[derive(Message)]
pub struct PlaceTowerMessage {
    pub tower_type: TowerType,
    pub tower_pos: U16Vec2,
}

pub fn handle_command_events(
    mut messages: MessageWriter<PlaceTowerMessage>, mut events: MessageReader<CommandEvent>,
    mut selection_state: ResMut<SelectionState>,
) {
    for event in events.read() {
        match event {
            CommandEvent::Help => print_help(),
            CommandEvent::Select { tile } => select_tile(&mut selection_state, *tile),
            CommandEvent::Place { tower_type, tower_pos } => place_tower(&mut messages, *tower_type, tower_pos),
            CommandEvent::Deselect => deselect_tile(&mut selection_state),
            CommandEvent::ExitGame => exit_game(),
        }
    }
}

fn print_help() {
    println!("help");
}

fn select_tile(selection_state: &mut SelectionState, tile: U16Vec2) {
    selection_state.selected_tile = Some(tile);
    println!("Selecting tile: {:?}", tile);
}

fn place_tower(
    messages: &mut MessageWriter<PlaceTowerMessage>, tower_type: TowerType, tower_pos: &U16Vec2,
) {
    println!("Placing tower {:?} at {:?}", tower_type, tower_pos);
    messages.write(PlaceTowerMessage{ tower_type, tower_pos: *tower_pos });
}

fn deselect_tile(selection_state: &mut SelectionState) {
    selection_state.selected_tile = None;
    println!("Deselect everything");
}

fn exit_game() {
    println!("Exiting game");
}
