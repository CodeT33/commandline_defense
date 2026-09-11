use crate::consts;
use crate::coordinates::GridCoordinate;
use crate::ecs_elements::resources::CommandState;
use crate::entities::enemies::EnemyType;
use crate::entities::tower::TowerType;
use crate::ui_overlay::grid::get_number_from_letter;
use crate::ui_overlay::ui_state::{EnemyPage, TowerPage, UiState};
use bevy::prelude::ResMut;
use clap::ValueEnum;

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

pub(crate) fn parse_command_preview(
    input: &str, current_preview: &ResMut<CommandState>,
) -> PreviewCommand {
    let mut preview = current_preview.preview.clone();

    if input.is_empty() {
        return PreviewCommand::None;
    }

    for command_text in input.split(';') {
        let command_text = command_text.trim();

        if input.is_empty() {
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
            ["open", menu_path] => {
                let parts: Vec<&str> = menu_path
                    .split(consts::COMMAND_OPEN_SEPARATION_CHARACTER)
                    .filter(|s| !s.is_empty())
                    .collect();

                match parts.as_slice() {
                    ["info"] => return PreviewCommand::SidebarState(UiState::Menus),
                    ["info", "towers"] => {
                        return PreviewCommand::SidebarState(UiState::TowersList {
                            selected: None,
                            further_details: None,
                        });
                    },
                    ["info", "towers", tower_type_string] => {
                        preview = match parse_tower_type(tower_type_string) {
                            Some(tower_type) => PreviewCommand::SidebarState(UiState::TowersList {
                                selected: Option::from(tower_type),
                                further_details: None,
                            }),
                            None => preview,
                        }
                    },
                    ["info", "towers", tower_type_string, "upgrades"] => {
                        preview = match parse_tower_type(tower_type_string) {
                            Some(tower_type) => PreviewCommand::SidebarState(UiState::TowersList {
                                selected: Option::from(tower_type),
                                further_details: Some(TowerPage::Upgrades),
                            }),
                            None => preview,
                        }
                    },
                    ["info", "towers", tower_type_string, "description"] => {
                        preview = match parse_tower_type(tower_type_string) {
                            Some(tower_type) => PreviewCommand::SidebarState(UiState::TowersList {
                                selected: Option::from(tower_type),
                                further_details: Some(TowerPage::Description),
                            }),
                            None => preview,
                        }
                    },
                    ["info", "enemies"] => {
                        return PreviewCommand::SidebarState(UiState::EnemiesList {
                            selected: None,
                            further_details: None,
                        });
                    },
                    ["info", "enemies", enemy_type_string] => {
                        preview = match parse_enemy_type(enemy_type_string) {
                            Some(enemy_type) => {
                                PreviewCommand::SidebarState(UiState::EnemiesList {
                                    selected: Option::from(enemy_type),
                                    further_details: None,
                                })
                            },
                            None => preview,
                        }
                    },
                    ["info", "enemies", enemy_type_string, "description"] => {
                        preview = match parse_enemy_type(enemy_type_string) {
                            Some(enemy_type) => {
                                PreviewCommand::SidebarState(UiState::EnemiesList {
                                    selected: Option::from(enemy_type),
                                    further_details: Some(EnemyPage::Description),
                                })
                            },
                            None => preview,
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

fn parse_tower_type(tower_type_string: &str) -> Option<TowerType> {
    <TowerType as clap::ValueEnum>::from_str(tower_type_string, true)
        .map_err(|_| println!("Unknown tower type: {:?}", tower_type_string))
        .ok()
}

fn parse_enemy_type(enemy_type_string: &str) -> Option<EnemyType> {
    EnemyType::from_str(enemy_type_string, false)
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
