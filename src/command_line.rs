use crate::consts;
use crate::map::{Map, TowerRangeMap};
use bevy::input_focus::tab_navigation::{TabGroup, TabIndex};
use bevy::input_focus::{AutoFocus, InputFocus};
use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle};
use clap::Parser;

#[derive(Resource, Default)]
pub struct DebugVisualizationState {
    spawned: Vec<Entity>,
    current_state: commands::ShowTowerRangeSetting,
}

mod commands {
    use clap::{Parser, Subcommand};

    #[derive(Parser, Debug)]
    #[command(no_binary_name = true)]
    pub enum Command {
        #[clap(subcommand)]
        #[command(name = "set")]
        AdjustSettings(Setting),
    }

    #[derive(Subcommand, Debug)]
    pub enum Setting {
        #[clap(subcommand)]
        #[command(name = "show-tower-range")]
        TowerRange(ShowTowerRangeSetting),
    }

    #[derive(Subcommand, Debug, Default, PartialEq, Clone, Copy)]
    pub enum ShowTowerRangeSetting {
        #[default]
        None,
        Idx {
            tower_idx: usize,
        },
        All,
    }
}

#[derive(Message)]
pub struct CommandSubmitted(pub String);

pub fn spawn_text_input(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::End,
                padding: px(8).all(),
                ..default()
            },
            TabGroup::new(0),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    padding: px(8).all(),
                    border: px(2).all(),
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.1, 0.1, 0.12, 0.9)),
                EditableText { visible_width: Some(16.0), allow_newlines: false, ..default() },
                TextFont { font_size: FontSize::Px(20.0), ..default() },
                TextColor(Color::WHITE),
                TextCursorStyle::default(),
                TabIndex(0),
                AutoFocus,
            ));
        });
}

pub fn handle_text_submission(
    focus: Res<InputFocus>, keys: Res<ButtonInput<KeyCode>>, mut inputs: Query<&mut EditableText>,
    mut messages: MessageWriter<CommandSubmitted>,
) {
    if !keys.just_pressed(KeyCode::Enter) {
        return;
    }
    let Some(entity) = focus.get() else {
        return;
    };
    let Ok(mut input) = inputs.get_mut(entity) else {
        return;
    };

    messages.write(CommandSubmitted(input.value().to_string()));
    input.clear();
}

pub fn parse_commandline_input(
    mut commands: Commands, mut messages: MessageReader<CommandSubmitted>, map: Res<Map>,
    tower_range_map: Res<TowerRangeMap>, mut state: ResMut<DebugVisualizationState>,
) {
    let Some(CommandSubmitted(input)) = messages.read().next() else {
        return;
    };
    let command = match commands::Command::try_parse_from(input.split_whitespace()) {
        Ok(com) => com,
        Err(e) => {
            eprintln!("Error parsing command: {}", e);
            return;
        },
    };
    println!("{:?}", command);

    match command {
        commands::Command::AdjustSettings(setting) => {
            state.apply_setting(setting, &mut commands, &tower_range_map, &map);
        },
    }
}

impl DebugVisualizationState {
    fn apply_tower_range_setting(
        &mut self, commands: &mut Commands, tower_range_map: &TowerRangeMap, map: &Map,
        setting: commands::ShowTowerRangeSetting,
    ) {
        if self.current_state == setting {
            return;
        }
        self.current_state = setting;

        for e in self.spawned.drain(..) {
            commands.entity(e).despawn();
        }

        let tower_positions = match setting {
            commands::ShowTowerRangeSetting::None => Vec::new(),
            commands::ShowTowerRangeSetting::Idx { tower_idx } => {
                map.towers.get(tower_idx).into_iter().copied().collect()
            },
            commands::ShowTowerRangeSetting::All => map.towers.clone(),
        };

        for tower_pos in tower_positions {
            let (min, max) = tower_range_map.range_bounds(tower_pos, consts::TOWER_RANGE_TILES);

            for y in min.y..=max.y {
                for x in min.x..=max.x {
                    let color = if (x, y) == (tower_pos.x, tower_pos.y) {
                        consts::TOWER_RANGE_TILE_CENTER_COLOR
                    } else {
                        consts::TOWER_RANGE_TILE_COLOR
                    };
                    let e = commands
                        .spawn((
                            Sprite { color, custom_size: Some(Vec2::splat(1.0)), ..default() },
                            Transform::from_xyz(x as f32 + 0.5, y as f32 + 0.5, -1.0),
                        ))
                        .id();
                    self.spawned.push(e);
                }
            }
        }
    }

    fn apply_setting(
        &mut self, setting: commands::Setting, commands: &mut Commands,
        tower_range_map: &TowerRangeMap, map: &Map,
    ) {
        match setting {
            commands::Setting::TowerRange(setting) => {
                self.apply_tower_range_setting(commands, tower_range_map, map, setting)
            },
        }
    }
}
