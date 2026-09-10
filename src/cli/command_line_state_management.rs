use crate::cli::auto_completion::Autocompletion;
use crate::coordinates::GridCoordinate;
use crate::ecs_elements::messages::CommandEvent;
use crate::ecs_elements::resources::{CommandHistory, CommandState, SelectionState};
use crate::entities::enemies::EnemyType;
use crate::entities::tower::TowerType;
use crate::ui_overlay::grid::get_number_from_letter;
use crate::ui_overlay::ui_state::UiState;
use crate::consts;
use bevy::input::ButtonInput;
use bevy::input_focus::InputFocus;
use bevy::prelude::{Color, KeyCode, MessageWriter, Query, Res, ResMut, TextColor};
use bevy::text::{EditableText, TextEdit};
use clap::error::ErrorKind::MissingRequiredArgument;
use clap::{Error, Parser, ValueEnum};
use std::str::FromStr;
use strum::{EnumString, VariantNames};

#[derive(Default, PartialEq, Clone, Eq)]
pub(crate) enum PreviewCommand {
    #[default]
    None,
    ShowGrid,
    SidebarState(UiState),
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

#[derive(Debug, VariantNames, EnumString, Copy, Clone, ValueEnum, PartialEq)]
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
        let show_error = determine_show_error(&command_state, &current_input);
        text_color.0 = if show_error { consts::ui::CONSOLE_ERROR_COLOR } else { Color::WHITE };
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

fn determine_show_error(command_state: &ResMut<CommandState>, current_input: &str) -> bool {
    let evaluated = &command_state.parse_output.evaluated;
    let autocompletion = &command_state.parse_output.autocompletion;
    evaluated.iter().rev().skip(1).any(|r| r.is_err())
        || evaluated.last().is_some_and(|l| {
            l.as_ref().is_err_and(|err| {
                current_input.trim().ends_with(";")
                    || (err.kind() != MissingRequiredArgument && autocompletion.is_empty())
            })
        })
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
            ["show", menu_path] => {
                let parts: Vec<&str> = menu_path.split('/').collect();

                match parts.as_slice() {
                    ["menu", "towers"] => {
                        return PreviewCommand::SidebarState(UiState::TowersList { filter: None });
                    },
                    ["menu", "towers", tower_type_string] => {
                        preview = match parse_tower_type(tower_type_string) {
                            Some(tower_type) => {
                                PreviewCommand::SidebarState(UiState::TowerInfo(tower_type))
                            },
                            None => PreviewCommand::SidebarState(UiState::TowersList {
                                filter: Some(tower_type_string.parse().unwrap()),
                            }),
                        }
                    },
                    ["menu", "enemies"] => {
                        return PreviewCommand::SidebarState(UiState::EnemiesList { filter: None });
                    },
                    ["show", "menu", "enemies", enemy_type_string] => {
                        preview = match parse_enemy_type(enemy_type_string) {
                            Some(enemy_type) => {
                                PreviewCommand::SidebarState(UiState::EnemyInfo(enemy_type))
                            },
                            None => PreviewCommand::SidebarState(UiState::EnemiesList {
                                filter: Some(enemy_type_string.parse().unwrap()),
                            }),
                        }
                    },
                    _ => {},
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

fn parse_tower_type(tower_type_string: &str) -> Option<TowerType> {
    <TowerType as clap::ValueEnum>::from_str(tower_type_string, true)
        .map_err(|_| println!("Unknown tower type: {:?}", tower_type_string))
        .ok()
}

fn parse_enemy_type(enemy_type_string: &str) -> Option<EnemyType> {
    EnemyType::from_str(enemy_type_string)
        .map_err(|_| println!("Unknown enemy type: {:?}", enemy_type_string))
        .ok()
}

pub(crate) fn parse_tile_position(position: &str) -> Option<GridCoordinate> {
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
