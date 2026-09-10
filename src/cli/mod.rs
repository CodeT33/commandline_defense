use crate::cli::command_line::spawn_command_line;
use bevy::prelude::Commands;

pub mod auto_completion;
pub(crate) mod command_event_handling;
pub(crate) mod command_line;
pub(crate) mod command_line_state_management;

pub(crate) fn spawn_game_cli(commands: &mut Commands) {
    spawn_command_line(commands);
}
