use crate::game_cli::command_line::CommandHistory;
use crate::game_cli::send_command_event;
use crate::ui_overlay::grid::get_number_from_letter;
use avian2d::parry::glamx::Vec2;
use bevy::input::ButtonInput;
use bevy::input_focus::InputFocus;
use bevy::prelude::{KeyCode, Message, MessageWriter, Query, Res, ResMut, Resource};
use bevy::text::EditableText;

#[derive(Resource, Default)]
pub struct CommandState {
    pub preview: PreviewCommand,
    pub last_command: String,
}

#[derive(Default)]
pub enum PreviewCommand {
    #[default]
    None,
    ShowGrid,
    ShowPath,
    ShowRestricted,
    ShowWater,
    ShowTowers,
    ShowRanges,
    HighlightTile {
        tile: Vec2,
    },
}

pub enum Command {
    Help,
    Select { tile: Vec2 },
    Deselect,
    ExitGame,
}

#[derive(Message, Debug)]
pub enum CommandEvent {
    Help,
    Select { tile: Vec2 },
    Deselect,
    ExitGame,
}

pub fn handle_command_line_state(
    focus: Res<InputFocus>, keys: Res<ButtonInput<KeyCode>>, mut inputs: Query<&mut EditableText>,
    mut command_state: ResMut<CommandState>, mut command_events: MessageWriter<CommandEvent>,
    mut history: ResMut<CommandHistory>,
) {
    let Some(entity) = focus.get() else {
        return;
    };
    let Ok(mut input) = inputs.get_mut(entity) else {
        return;
    };

    let current_input = input.value().to_string();

    if current_input != command_state.last_command {
        command_state.last_command = current_input.clone();
        command_state.preview = parse_command_preview(&current_input);
        println!("Preview changed: {:?}", current_input)
    }

    if keys.just_pressed(KeyCode::Enter) {
        if let Some(command) = parse_command_event(&current_input) {
            send_command_event(command, &mut command_events);
            history.entries.push(current_input.clone());
            history.idx = history.entries.len();
        }

        input.clear();

        command_state.last_command.clear();
        command_state.preview = PreviewCommand::None;
    }
}

fn parse_command_preview(input: &str) -> PreviewCommand {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let trimmed_input = input.trim();

    if "select".starts_with(trimmed_input) && !"se".starts_with(trimmed_input) {
        return PreviewCommand::ShowGrid;
    }

    match tokens.as_slice() {
        ["select"] => PreviewCommand::ShowGrid,
        ["show grid"] => PreviewCommand::ShowGrid,
        ["show path"] => PreviewCommand::ShowPath,
        ["show restricted"] => PreviewCommand::ShowRestricted,
        ["show water"] => PreviewCommand::ShowWater,
        ["show towers"] => PreviewCommand::ShowTowers,
        ["show ranges"] => PreviewCommand::ShowRanges,
        ["select", position] => match parse_tile_position(position) {
            Some(tile) => PreviewCommand::HighlightTile { tile },
            None => PreviewCommand::ShowGrid,
        },
        _ => PreviewCommand::None,
    }
}

fn parse_command_event(input: &str) -> Option<Command> {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    match tokens.as_slice() {
        ["help"] => Some(Command::Help),
        ["select", position] => {
            let tile = parse_tile_position(position)?;
            Some(Command::Select { tile })
        },
        ["deselect"] => Some(Command::Deselect),
        ["exit", "game"] => Some(Command::ExitGame),
        _ => {
            println!("Unknown command: {:?}", input);
            Some(Command::Help)
        },
    }
}

fn parse_tile_position(position: &str) -> Option<Vec2> {
    let position = position.to_ascii_uppercase();

    let mut number = String::new();
    let mut letter = None;

    for character in position.chars() {
        if character.is_ascii_digit() {
            number.push(character);
        } else if character.is_ascii_alphabetic() {
            //Only one letter
            if letter.is_some() {
                return None;
            }

            letter = Some(character);
        } else {
            return None;
        }
    }

    let x: u16 = number.parse().ok()?;
    let y: u16 = get_number_from_letter(letter?)?;

    Some(Vec2::new(x as f32, y as f32))
}
