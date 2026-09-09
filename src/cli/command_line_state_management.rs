use crate::coordinates::GridCoordinate;
use crate::ecs_elements::messages::CommandEvent;
use crate::ecs_elements::resources::{CommandHistory, CommandState, SelectionState};
use crate::entities::enemies::EnemyType;
use crate::entities::tower::TowerType;
use crate::ui_overlay::grid::get_number_from_letter;
use crate::ui_overlay::ui_state::UiState;
use bevy::input::ButtonInput;
use bevy::input_focus::InputFocus;
use bevy::prelude::{KeyCode, MessageWriter, Query, Res, ResMut};
use bevy::text::EditableText;
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

#[derive(Debug, VariantNames, EnumString)]
pub(crate) enum Settings {
    #[strum(serialize = "bounding-boxes")]
    BoundingBoxes,
    #[strum(serialize = "sim-speed")]
    SimSpeed,
    #[strum(serialize = "enemy-spawn-interval")]
    EnemySpawnInterval,
}

pub(crate) fn handle_command_line_state(
    focus: Res<InputFocus>, keys: Res<ButtonInput<KeyCode>>, mut inputs: Query<&mut EditableText>,
    mut command_state: ResMut<CommandState>, mut command_events: MessageWriter<CommandEvent>,
    mut history: ResMut<CommandHistory>, selection_state: ResMut<SelectionState>,
) {
    let Some(entity) = focus.get() else {
        return;
    };
    let Ok(mut input) = inputs.get_mut(entity) else {
        return;
    };

    let current_input = input.value().to_string();

    //Preview
    if current_input != command_state.last_input {
        command_state.last_input = current_input.clone();
        command_state.preview = parse_command_preview(&current_input);
    }

    //Submit
    if keys.just_pressed(KeyCode::Enter) {
        let output: Result<Vec<_>, _> =
            parse_commandline_input(&current_input, selection_state.selected_tile)
                .into_iter()
                .collect::<Result<_, _>>();

        let commands = match output {
            Ok(commands) => commands,
            Err(err) => {
                println!("Failed to parse commands: {}", err);
                return;
            },
        };

        for command in commands {
            command_events.write(command);
            history.entries.push(current_input.clone());
            history.idx = history.entries.len();
        }

        input.clear();

        command_state.last_input.clear();
        command_state.preview = PreviewCommand::None;
    }
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
            ["show", "towers"] => {
                return PreviewCommand::SidebarState(UiState::TowersList { filter: None });
            },
            ["show", "towers", tower_type_string] => {
                preview = match parse_tower_type(tower_type_string) {
                    Some(tower_type) => PreviewCommand::SidebarState(UiState::TowerInfo(tower_type)),
                    None => PreviewCommand::SidebarState(UiState::TowersList {filter: Some(tower_type_string.parse().unwrap())}),
                }
            },
            ["show", "enemies"] => {
                return PreviewCommand::SidebarState(UiState::EnemiesList { filter: None });
            },
            ["show", "enemies", enemy_type_string] => {
                preview = match parse_enemy_type(enemy_type_string) {
                    Some(enemy_type) => PreviewCommand::SidebarState(UiState::EnemyInfo(enemy_type)),
                    None => PreviewCommand::SidebarState(UiState::EnemiesList {filter: Some(enemy_type_string.parse().unwrap())})
                }
            },

            _ => {},
        }
    }
    preview
}

fn parse_commandline_input(
    input: &str, mut selected_tile: Option<GridCoordinate>,
) -> Vec<Result<CommandEvent, String>> {
    input
        .split(';')
        .filter_map(|command_text| {
            parse_single_command(command_text, &mut selected_tile).transpose()
        })
        .collect()
}

fn parse_single_command(
    input_str: &str, current_selected_tile: &mut Option<GridCoordinate>,
) -> Result<Option<CommandEvent>, String> {
    let command_text = input_str.trim();

    if command_text.is_empty() {
        return Ok(None);
    }

    let tokens: Vec<&str> = command_text.split_whitespace().collect();

    Ok(Some(match tokens.as_slice() {
        ["help"] => CommandEvent::Help,
        ["select", position] => {
            let tile = parse_tile_position(position)
                .ok_or_else(|| format!("Invalid tile position: {:?}", position))?;
            *current_selected_tile = Some(tile);
            CommandEvent::Select { tile }
        },
        ["place", tower_type] => {
            let tile = current_selected_tile
                .ok_or_else(|| "Cannot place tower: no tile selected".to_string())?;
            let tower_type = parse_tower_type(tower_type).ok_or_else(|| {
                format!("Cannot place tower: unknown tower type: {:?}", tower_type)
            })?;
            CommandEvent::Place { tower_type, tower_pos: tile }
        },
        ["clear"] => {
            *current_selected_tile = None;
            CommandEvent::Clear
        },
        ["show", "balance"] => CommandEvent::Balance,
        ["exit", "game"] => CommandEvent::ExitGame,
        ["set", setting, value] => {
            let value = value.parse::<f32>().map_err(|e| e.to_string())?;
            let setting = Settings::from_str(setting).map_err(|_| {
                format!(
                    "Unknown setting: \"{}\", possible options are: {}",
                    setting,
                    Settings::VARIANTS.join(", ")
                )
            })?;
            CommandEvent::Set { setting, value }
        },
        _ => Err(format!("Unknown command: \"{}\"", command_text))?,
    }))
}

fn parse_tower_type(tower_type_string: &str) -> Option<TowerType> {
    TowerType::from_str(tower_type_string)
        .map_err(|_| println!("Unknown tower type: {:?}", tower_type_string))
        .ok()
}

fn parse_enemy_type(enemy_type_string: &str) -> Option<EnemyType> {
    EnemyType::from_str(enemy_type_string)
        .map_err(|_| println!("Unknown enemy type: {:?}", enemy_type_string))
        .ok()
}

fn parse_tile_position(position: &str) -> Option<GridCoordinate> {
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
