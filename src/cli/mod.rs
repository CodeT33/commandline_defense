use crate::cli::command_line::spawn_command_line;
use bevy::prelude::Commands;

pub mod command_event_handling;
pub mod command_line;
pub mod command_line_state_management;

pub fn spawn_game_cli(commands: &mut Commands) {
    spawn_command_line(commands);
}
