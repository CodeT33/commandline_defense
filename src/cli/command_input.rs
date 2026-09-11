use crate::cli::auto_completion::Autocompletion;
use crate::consts;
use crate::coordinates::GridCoordinate;
use crate::entities::enemies::EnemyType;
use crate::entities::tower::TowerType;
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
    SpawnInterval {
        #[arg(value_parser = clap::value_parser!(u16).range(1..))]
        value: u16,
    },
    EnemyType {
        enemy_type: EnemyType,
    },
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

#[derive(Default)]
pub(crate) struct ParseOutput {
    pub(crate) autocompletion: Vec<String>,
    pub(crate) evaluated: Vec<Result<CommandInput, Error>>,
}

pub(crate) fn parse_commandline_input(input: &str) -> ParseOutput {
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

pub(crate) fn determine_show_error(output: &ParseOutput, current_input: &str) -> bool {
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
