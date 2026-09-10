use clap::CommandFactory;
use clap::error::ErrorKind;
use clap_complete::CompletionCandidate;
use clap_complete::engine::complete;

pub trait Autocompletion: CommandFactory {
    fn get_autocompletion(input: &str) -> Vec<CompletionCandidate> {
        let mut command_object = Self::command();
        let mut words: Vec<_> = input.split_whitespace().map(|s| s.into()).collect();
        if words.is_empty() || input.ends_with(" ") {
            words.push("".into())
        }
        let last_word_idx = words.len() - 1;
        if last_word_idx != 0
            && let Err(err) = Self::command().try_get_matches_from(&words[..last_word_idx])
            && !matches!(
                err.kind(),
                ErrorKind::MissingRequiredArgument | ErrorKind::MissingSubcommand
            )
        {
            return vec![];
        }
        complete(&mut command_object, words, last_word_idx, None).unwrap_or_default()
    }
}

impl<T: CommandFactory> Autocompletion for T {}
