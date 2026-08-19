use crate::game_cli::command_line_state_management::{Command, CommandEvent};
use bevy::prelude::MessageWriter;

pub mod command_event_handling;
pub mod command_line;
pub mod command_line_state_management;

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
