// use crate::ecs_elements::messages::CommandEvent;
// use std::str::SplitWhitespace;
// use strum::VariantNames;
//
// #[allow(unused)]
// pub trait AutoCompletion: VariantNames {
//     fn trigger(input: &mut SplitWhitespace) -> AutoCompletionResult;
// }
//
// pub struct AutoCompletionResult {
//     /// Can be used to replace the input string
//     complete_to: Option<&'static str>,
//     /// Can be used to replace the input string
//     auto_completion_results: Vec<&'static str>,
// }
//
// impl AutoCompletionResult {
//     pub fn empty() -> Self {
//         Self { complete_to: None, auto_completion_results: vec![] }
//     }
// }
//
// impl CommandEvent {
//     fn trigger(input: &mut SplitWhitespace) -> AutoCompletionResult {
//         let Some(first_word) = input.next() else {
//             return AutoCompletionResult {
//                 complete_to: None,
//                 auto_completion_results: Self::VARIANTS.to_vec(),
//             };
//         };
//         if let Some(matching_variant) =
//             Self::VARIANTS.iter().find(|variant| **variant == first_word)
//         {
//             match *matching_variant {
//                 "help" => AutoCompletionResult::empty(),
//                 _ => {},
//             }
//         } else {
//             let possible_options = Self::VARIANTS
//                 .iter()
//                 .copied()
//                 .filter(|variant| variant.starts_with(first_word))
//                 .collect::<Vec<_>>();
//             AutoCompletionResult {
//                 complete_to: (possible_options.len() == 1).then_some(possible_options[0]),
//                 auto_completion_results: possible_options,
//             }
//         }
//     }
// }
//
// pub trait SingleWordIsEnough {}
//
// impl AutoCompletion for CommandEvent{
// 	fn trigger(input: &mut SplitWhitespace) -> AutoCompletionResult {
// 		// 1. Get first word else return autocompletion result empty
// 		// 2. if first word incomplete -> return matching options
// 		// 3. if word matches variant perfectly:
// 		// 			1. if variant has no members -> Autocomp::empty
// 		// 			2. if variant has one member -> run trigger on member
// 		// 			3. if variant has several members -> run trigger on all members one
// 		todo!()
// 	}
// }

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::engine::complete;
use std::ffi::OsString;

#[derive(Parser)]
#[command(name = "", disable_help_subcommand = true, disable_help_flag = true)]
struct Cli {
    #[command(subcommand)]
    cmd: RootCommand,
}

#[derive(Subcommand)]
enum RootCommand {
    Spawn {
        #[command(subcommand)]
        target: TargetCommand,
    },
    Quit,
}

#[derive(Subcommand)]
enum TargetCommand {
    Player,
    Enemy,
    SomethingElse { lol: i32, other: bool },
}

#[cfg(test)]
fn autocomplete(input: &str) -> Vec<String> {
    let mut cmd = Cli::command();

    // 1. Tokenize input with a dummy executable prefix
    let mut args: Vec<OsString> = std::iter::once(OsString::from("app"))
        .chain(input.split_whitespace().map(OsString::from))
        .collect();

    // 2. Trailing whitespace signals the start of a new argument
    if input.ends_with(' ') || input.is_empty() {
        args.push(OsString::new());
    }

    // 3. Complete at the final token index
    let active_index = args.len() - 1;

    complete(&mut cmd, args, active_index, None)
        .unwrap_or_default()
        .into_iter()
        .map(|candidate| candidate.get_value().to_string_lossy().into_owned())
        .collect()
}

#[test]
fn run_test() {
    println!("{:?}", autocomplete("")); // ["spawn"]
    println!("{:?}", autocomplete("sp")); // ["spawn"]
    println!("{:?}", autocomplete("spawn ")); // ["enemy", "help", "player"]
    println!("{:?}", autocomplete("spawn p")); // ["player"]
}
