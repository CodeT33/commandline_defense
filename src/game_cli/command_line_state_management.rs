use crate::coordinates::GridCoordinate;
use crate::ecs_elements::messages::CommandEvent;
use crate::ecs_elements::resources::{CommandHistory, CommandState, SelectionState};
use crate::game_cli::send_command_event;
use crate::tower::TowerType;
use crate::ui_overlay::grid::get_number_from_letter;
use bevy::input::ButtonInput;
use bevy::input_focus::InputFocus;
use bevy::prelude::{KeyCode, MessageWriter, Query, Res, ResMut};
use bevy::text::EditableText;

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
        tile: GridCoordinate,
    },
}

pub enum Command {
    Help,
    Select { tile: GridCoordinate },
    Place { tower_type: TowerType, tower_pos: GridCoordinate },
    Clear,
    Balance,
    ExitGame,
}

pub fn handle_command_line_state(
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
        let commands = parse_command_event(&current_input, selection_state.selected_tile);

        for command in commands {
            send_command_event(command, &mut command_events);
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
            _ => {},
        }
    }
    preview
}

fn parse_command_event(input: &str, selected_tile: Option<GridCoordinate>) -> Vec<Command> {
    let mut commands = Vec::new();

    let mut current_selected_tile = selected_tile;

    for command_text in input.split(';') {
        let command_text = command_text.trim();

        if command_text.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = command_text.split_whitespace().collect();

        match tokens.as_slice() {
            ["help"] => {
                commands.push(Command::Help);
            },
            ["select", position] => {
                let Some(tile) = parse_tile_position(position) else {
                    println!("Invalid tile position: {:?}", position);
                    continue;
                };

                current_selected_tile = Some(tile);

                commands.push(Command::Select { tile });
            },
            ["place", tower_type] => {
                let Some(tile) = current_selected_tile else {
                    println!("Cannot place tower: no tile selected");
                    continue;
                };
                let Some(tower_type) = parse_tower_type(tower_type) else {
                    println!("Cannot place tower: unknown tower type: {:?}", tower_type);
                    continue;
                };

                commands.push(Command::Place { tower_type, tower_pos: tile });
            },
            ["clear"] => {
                current_selected_tile = None;
                commands.push(Command::Clear);
            },
            ["show", "balance"] => {
                commands.push(Command::Balance);
            },
            ["exit", "game"] => {
                commands.push(Command::ExitGame);
            },
            _ => {
                println!("Unknown command: {:?}", command_text);
                commands.push(Command::Help);
            },
        }
    }
    commands
}

fn parse_tower_type(tower_type_string: &str) -> Option<TowerType> {
    match tower_type_string {
        "assault-troop" => Some(TowerType::AssaultTower),
        "boom-troop" => Some(TowerType::BoomTower),
        "gatling-troop" => Some(TowerType::GatlingTower),
        "sniper-troop" => Some(TowerType::SniperTower),
        "eitshtu" => Some(TowerType::Eitshtu),
        "acitonion" => Some(TowerType::Acitonion),
        "strorm" => Some(TowerType::Strorm),
        "infernon" => Some(TowerType::Infernon),
        "icebyte" => Some(TowerType::Icebyte),
        "goldt" => Some(TowerType::Goldt),
        "copprina" => Some(TowerType::Copprina),
        _ => {
            println!("Unknown tower type: {:?}", tower_type_string);
            None
        },
    }
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
