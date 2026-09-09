#[cfg(test)]
mod tests {
    use crate::ecs_elements::messages::CommandEvent;
    use clap::{CommandFactory, Parser};
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
            complete(&mut command_object, words, last_word_idx, None).unwrap_or_default()
        }
    }

    impl<T: CommandFactory> Autocompletion for T {}

    #[test]
    fn sth() {
        let inputs = ["hel", "help", "sel", "select", "select ", "select c", "select c1", "place "];
        for input in inputs {
            let (auto, res) = quick_test(input);
            println!(
                "input: \"{}\", autocompletion: {:?}, Result:\n{}\n",
                input,
                auto,
                match res {
                    Ok(event) => format!("Parsed Successfully: {:?}", event),
                    Err(err) => err,
                }
            );
        }
    }

    fn quick_test(input: &str) -> (Vec<String>, Result<CommandEvent, String>) {
        let result = CommandEvent::get_autocompletion(input);
        let autocompletion: Vec<_> =
            result.iter().map(|cc| cc.get_value().to_string_lossy().to_string()).collect();
        let parsed =
            CommandEvent::try_parse_from(input.split_whitespace()).map_err(|e| e.to_string());
        (autocompletion, parsed)
    }
}
