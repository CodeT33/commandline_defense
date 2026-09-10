use crate::cli::auto_completion::Autocompletion;
use crate::coordinates::GridCoordinate;
use crate::ecs_elements::messages::CommandEvent;
use crate::ecs_elements::resources::{CommandHistory, CommandState, SelectionState};
use crate::entities::tower::TowerType;
use crate::ui_overlay::grid::get_number_from_letter;
use bevy::input::ButtonInput;
use bevy::input_focus::InputFocus;
use bevy::prelude::{Color, KeyCode, MessageWriter, Query, Res, ResMut, TextColor};
use bevy::text::{EditableText, TextEdit};
use clap::{Error, Parser, ValueEnum};

#[derive(Default)]
pub(crate) enum PreviewCommand {
    #[default]
    None,
    ShowGrid,
    #[allow(unused)]
    ShowPath,
    #[allow(unused)]
    ShowRestricted,
    #[allow(unused)]
    ShowWater,
    #[allow(unused)]
    ShowTowers,
    #[allow(unused)]
    ShowRanges,
    HighlightTile {
        tile: GridCoordinate,
    },
}

#[derive(Debug, PartialEq, ValueEnum, Copy, Clone)]
pub(crate) enum Settings {
    BoundingBoxes,
    SimSpeed,
    EnemySpawnInterval,
}

pub(crate) fn handle_command_line_state(
    focus: Res<InputFocus>, keys: Res<ButtonInput<KeyCode>>,
    mut inputs: Query<(&mut EditableText, &mut TextColor)>,
    mut command_state: ResMut<CommandState>, mut command_events: MessageWriter<CommandEvent>,
    mut history: ResMut<CommandHistory>, mut selection_state: ResMut<SelectionState>,
) {
    let Some(entity) = focus.get() else {
        return;
    };
    let Ok((mut input, mut text_color)) = inputs.get_mut(entity) else {
        return;
    };

    let mut current_input = input.value().to_string();

    //Preview
    if current_input != command_state.last_input {
        command_state.last_input = current_input.clone();
        command_state.preview = parse_command_preview(&current_input);
        command_state.parse_output = parse_commandline_input(&current_input);
        if !command_state.parse_output.autocompletion.is_empty() {
            println!("{:?}", command_state.parse_output.autocompletion);
        }
        text_color.0 = if command_state.parse_output.evaluated.iter().any(|l| l.is_err())
            && (command_state.parse_output.autocompletion.is_empty()
                || current_input.split(";").last().is_some_and(|s| s.is_empty()))
        {
            Color::linear_rgb(1.0, 0.0, 0.0)
        } else {
            Color::WHITE
        };
    }

    if keys.just_pressed(KeyCode::Tab)
        && let [complete_to] = command_state.parse_output.autocompletion.as_slice()
        && let Some(last_command) = current_input.split(";").last()
        && let Some(last_word) = last_command.split_whitespace().last()
        && complete_to.len() > last_word.len()
    {
        let new_len = current_input.len() - last_word.len();
        current_input.truncate(new_len);
        current_input.push_str(complete_to);
        input.editor.set_text(&current_input);
        input.queue_edit(TextEdit::TextEnd(false));
    }

    //Submit
    if keys.just_pressed(KeyCode::Enter) {
        let command_inputs = match command_state
            .parse_output
            .evaluated
            .iter()
            .map(|r| r.as_ref().map_err(|e| e.to_string()).copied())
            .collect::<Result<Vec<_>, String>>()
        {
            Ok(commands) => commands,
            Err(err) => {
                println!("Failed to parse commands: {}", err);
                return;
            },
        };

        let mut local_selection_state = selection_state.selected_tile;
        let sendable_commands =
            match parse_to_sendable_commands(&command_inputs, &mut local_selection_state) {
                Ok(commands) => commands,
                Err(error) => {
                    println!("{}", error);
                    return;
                },
            };
        selection_state.selected_tile = local_selection_state;

        for command in sendable_commands {
            command_events.write(command);
            history.entries.push(current_input.clone());
            history.idx = history.entries.len();
        }

        input.clear();

        command_state.last_input.clear();
        command_state.preview = PreviewCommand::None;
    }
}

fn parse_to_sendable_commands(
    input_commands: &[CommandInput], selected_tile: &mut Option<GridCoordinate>,
) -> Result<Vec<CommandEvent>, &'static str> {
    input_commands
        .iter()
        .copied()
        .map(|ic| {
            Ok(match ic {
                CommandInput::Help => CommandEvent::Help,
                CommandInput::Select { tile } => {
                    *selected_tile = tile.into();
                    CommandEvent::Select { tile }
                },
                CommandInput::Place { tower_type } => selected_tile
                    .map(|p| CommandEvent::Place { tower_type, tower_pos: p })
                    .ok_or("No Tile selected")?,
                CommandInput::Clear => CommandEvent::Clear,
                CommandInput::Balance => CommandEvent::Balance,
                CommandInput::ExitGame => CommandEvent::ExitGame,
                CommandInput::Set { setting, value } => CommandEvent::Set { setting, value },
            })
        })
        .collect()
}

fn parse_command_preview(input: &str) -> PreviewCommand {
    let mut preview = PreviewCommand::None;

    for command_text in input.split(';') {
        let command_text = command_text.trim();

        if command_text.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = command_text.split_whitespace().collect();

        match tokens.as_slice() {
            ["show", "grid"] => return PreviewCommand::ShowGrid,
            ["select"] => {
                return PreviewCommand::ShowGrid;
            },
            ["select", position] => {
                preview = match parse_tile_position(position) {
                    Some(tile) => PreviewCommand::HighlightTile { tile },
                    None => PreviewCommand::ShowGrid,
                }
            },
            _ => {},
        }
    }
    preview
}

#[derive(Default)]
pub struct ParseOutput {
    autocompletion: Vec<String>,
    evaluated: Vec<Result<CommandInput, Error>>,
}

fn parse_commandline_input(input: &str) -> ParseOutput {
    let split = input.split(';').collect::<Vec<_>>();
    let evaluated = split
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(parse_single_command_new)
        .collect::<Vec<Result<_, Error>>>();

    let autocompletion: Vec<_> =
        split.last().map(get_auto_completion_single_line).unwrap_or_default();
    ParseOutput { evaluated, autocompletion }
}

#[derive(Parser, Debug, Clone, Copy)]
#[command(
    no_binary_name = true,
    disable_help_subcommand = true,
    disable_help_flag = true,
    override_usage = "<COMMAND>"
)]
pub(crate) enum CommandInput {
    Help,
    Select { tile: GridCoordinate },
    Place { tower_type: TowerType },
    Clear,
    Balance,
    ExitGame,

    Set { setting: Settings, value: f32 },
}

fn parse_single_command_new(input_str: impl AsRef<str>) -> Result<CommandInput, Error> {
    CommandInput::try_parse_from(input_str.as_ref().split_whitespace())
}

fn get_auto_completion_single_line(input_str: impl AsRef<str>) -> Vec<String> {
    CommandInput::get_autocompletion(input_str.as_ref())
        .iter()
        .map(|cc| cc.get_value().to_string_lossy().to_string())
        .collect()
}

#[test]
fn test_input() {
    let input = "set";
    println!(
        "{:?}",
        match parse_single_command_new(input) {
            Ok(val) => println!("Parsed: {:?}", val),
            Err(err) => println!("Err: {}", err),
        }
    );
    println!("{:?}", get_auto_completion_single_line(input));
}

pub fn parse_tile_position(position: &str) -> Option<GridCoordinate> {
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

    Some(GridCoordinate::new(x, y))
}
