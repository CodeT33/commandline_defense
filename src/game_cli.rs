use crate::game_cli::command_line_state_management::{Command, CommandEvent};
use bevy::prelude::{Commands, MessageWriter};
use crate::game_cli::command_line::spawn_command_line;

pub mod command_event_handling;
pub mod command_line;
pub mod command_line_state_management;

pub fn spawn_game_cli(commands: &mut Commands) {
    spawn_command_line(commands);
}

/// API to send command_events into the system (for example a "select" or "exit game")
pub fn send_command_event(command: Command, events: &mut MessageWriter<CommandEvent>) {
    match command {
        Command::Help => {
            events.write(CommandEvent::Help);
        }
        Command::Select { tile } => {
            events.write(CommandEvent::Select { tile });
        },
        Command::Deselect => {
            events.write(CommandEvent::Deselect);
        },
        Command::ExitGame => {
            events.write(CommandEvent::ExitGame);
        },
    }
}
