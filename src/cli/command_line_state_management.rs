use crate::cli::auto_completion::Autocompletion;
use crate::cli::preview::{PreviewCommand, parse_command_preview};
use crate::consts;
use crate::coordinates::GridCoordinate;
use crate::ecs_elements::messages::CommandEvent;
use crate::ecs_elements::resources::{CommandHistory, CommandState, SelectionState};
use crate::entities::enemies::EnemyType;
use crate::entities::tower::TowerType;
use bevy::input::ButtonInput;
use bevy::input_focus::InputFocus;
use bevy::prelude::{Color, KeyCode, MessageWriter, Query, Res, ResMut, TextColor};
use bevy::text::{EditableText, TextEdit};
use clap::error::ErrorKind::MissingRequiredArgument;
use clap::{Error, Parser, Subcommand, ValueEnum};
use std::fmt::Debug;
use std::ops::RangeBounds;
use std::str::FromStr;

#[derive(Debug, PartialEq, Subcommand, Copy, Clone)]
pub(crate) enum Settings {
    BoundingBoxes {
        #[arg(action = clap::ArgAction::Set, value_parser = clap::value_parser!(bool))]
        value: bool,
    },
    SimSpeed {
        #[arg(value_parser = float_range(0.0..=consts::MAX_SIM_SPEED))]
        value: f32,
    },
    EnemySpawnInterval {
        #[arg(value_parser = clap::value_parser!(u16).range(1..))]
        value: u16,
    },
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
        command_state.preview = parse_command_preview(&current_input, &command_state);
        command_state.parse_output = parse_commandline_input(&current_input);
        if !command_state.parse_output.autocompletion.is_empty() {
            println!("{:?}", command_state.parse_output.autocompletion);
        }
        let show_error = determine_show_error(&command_state.parse_output, &current_input);
        text_color.0 = if show_error { consts::ui::CONSOLE_ERROR_COLOR } else { Color::WHITE };
    }

    if keys.just_pressed(KeyCode::Tab) {
        let current_input_cleaned_up =
            current_input.replace(consts::COMMAND_OPEN_SEPARATION_CHARACTER, " ");
        if let [complete_to] = command_state.parse_output.autocompletion.as_slice()
            && let Some(last_command) = current_input_cleaned_up.split(";").last()
            && let Some(last_word) = last_command.split_whitespace().last()
            && complete_to.len() >= last_word.len()
        {
            let mut new = complete_to.to_owned();
            println!("{}", last_command);
            if last_command.trim_start().starts_with("open ") {
                new.push(consts::COMMAND_OPEN_SEPARATION_CHARACTER);
            } else {
                new.push(' ');
            }
            let new_len = if current_input_cleaned_up.ends_with(" ") {
                current_input.len()
            } else {
                current_input.len() - last_word.len()
            };
            current_input.truncate(new_len);

            current_input.push_str(&new);
            input.editor.set_text(&current_input);
            input.queue_edit(TextEdit::TextEnd(false));
        }
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

fn determine_show_error(output: &ParseOutput, current_input: &str) -> bool {
    let evaluated = &output.evaluated;
    let autocompletion = &output.autocompletion;
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
        .map(|ic| {
            Ok(match ic {
                CommandInput::Help => CommandEvent::Help,
                CommandInput::Select { tile } => {
                    *selected_tile = (*tile).into();
                    CommandEvent::Select { tile: *tile }
                },
                CommandInput::Place { tower_type } => selected_tile
                    .map(|p| CommandEvent::Place { tower_type: *tower_type, tower_pos: p })
                    .ok_or("No Tile selected")?,
                CommandInput::Clear => CommandEvent::Clear,
                CommandInput::Balance => CommandEvent::Balance,
                CommandInput::ExitGame => CommandEvent::ExitGame,
                CommandInput::Set(setting) => CommandEvent::Set(*setting),
                _ => {
                    println!("juckt");
                    Err("Leck Eier")?
                },
            })
        })
        .collect()
}

#[derive(Default)]
pub struct ParseOutput {
    autocompletion: Vec<String>,
    evaluated: Vec<Result<CommandInput, Error>>,
}

fn parse_commandline_input(input: &str) -> ParseOutput {
    let split = input
        .split(';')
        .map(|s| s.replace(consts::COMMAND_OPEN_SEPARATION_CHARACTER, " "))
        .collect::<Vec<_>>();
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
    Select {
        #[arg(value_parser = parse_tile)]
        tile: GridCoordinate,
    },
    Place {
        tower_type: TowerType,
    },
    Clear,
    Balance,
    ExitGame,
    #[command(subcommand)]
    Set(Settings),
    #[command(subcommand)]
    Open(OpenCommand),
}

#[derive(Subcommand, Debug, Clone, Copy)]
pub(crate) enum OpenCommand {
    Info {
        #[command(subcommand)]
        further: Option<FurtherInfo>,
    },
}

#[derive(Subcommand, Debug, Clone, Copy)]
pub(crate) enum FurtherInfo {
    Enemies {
        enemy_type: Option<EnemyType>,
        #[arg(requires = "enemy_type")]
        further: Option<EnemyFurther>,
    },
    Towers {
        tower_type: Option<TowerType>,
        #[arg(requires = "tower_type")]
        further: Option<TowerFurther>,
    },
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub(crate) enum EnemyFurther {
    Description,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub(crate) enum TowerFurther {
    Description,
    Upgrades,
}

fn parse_tile(s: &str) -> Result<GridCoordinate, String> {
    let tile = GridCoordinate::from_str(s)?;
    tile.is_on_map(consts::MAP_SIZE_TILES)
        .then_some(tile)
        .ok_or_else(|| format!("tile {} is not on the map", s))
}

fn float_range<T: RangeBounds<f32> + Debug + Clone>(
    range: T,
) -> impl Fn(&str) -> Result<f32, String> + Clone {
    move |s: &str| {
        let value: f32 = s.parse().map_err(|_| "expected a number".to_string())?;
        range.contains(&value).then_some(value).ok_or_else(|| format!("must be in {:?}", range))
    }
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

#[test]
#[ignore]
fn probe_show_error() {
    let cases: &[(&str, bool)] = &[
        ("help", false),
        ("help;", false),
        ("badcmd", true),
        ("badcmd; help", true),
        ("help; badcmd", true),
        ("se", false),
        ("sh", true),
        ("exit", false),
        ("select", false),
        ("select ", false),
        ("select 3A", false),
        ("select 3", false),
        ("select A", false),
        ("select 3A; select 3", false),
        ("select AB", true),
        ("select 3!", true),
        ("select a3b", true),
        ("place", false),
        ("place ", false),
        ("place assault", false),
        ("place assault-bober", false),
        ("place b", false),
        ("place qwerty", true),
        ("set", false),
        ("set sim-speed ", false),
        ("set sim-speed 2", false),
        ("set nope 2", true),
        ("select;", true),
        ("place;", true),
        ("select 3A; select", false),
        ("select 3A; ; place assault", false),
        ("set sim-speed nan", true),
        ("set sim-speed -1", true),
        ("show balance", true),
        ("exit game", true),
    ];
    for (input, expected) in cases {
        let output = parse_commandline_input(input);
        let actual = determine_show_error(&output, input);
        assert_eq!(actual, *expected, "input {:?}", input);
    }
}
