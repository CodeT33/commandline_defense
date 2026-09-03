use crate::game_cli::command_line::spawn_command_line;
use crate::game_cli::command_line_state_management::Command;
use crate::messages::CommandEvent;
use bevy::prelude::{Commands, MessageWriter};

pub mod command_event_handling;
pub mod command_line;
pub mod command_line_state_management;

pub fn spawn_game_cli(commands: &mut Commands) {
    spawn_command_line(commands);
}

/// API to send command_events into the system (for example a "select" or "exit game")
pub fn send_command_event(command: Command, events: &mut MessageWriter<CommandEvent>) {
    events.write(match command {
        Command::Help => CommandEvent::Help,
        Command::Select { tile } => CommandEvent::Select { tile },
        Command::Place { tower_type, tower_pos } => CommandEvent::Place { tower_type, tower_pos },
        Command::Deselect => CommandEvent::Deselect,
        Command::Balance => CommandEvent::Balance,
        Command::ExitGame => CommandEvent::ExitGame,
    });
}
