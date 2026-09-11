use crate::cli::command_input::{CommandInput, determine_show_error, parse_commandline_input};
use crate::cli::preview::{PreviewCommand, parse_command_preview};
use crate::consts;
use crate::coordinates::GridCoordinate;
use crate::ecs_elements::components::CommandAutoCompletion;
use crate::ecs_elements::messages::CommandEvent;
use crate::ecs_elements::resources::{CommandHistory, CommandState, SelectionState};
use bevy::input::ButtonInput;
use bevy::input_focus::InputFocus;
use bevy::prelude::{
    Color, KeyCode, MessageWriter, Node, Query, Res, ResMut, Text, TextColor, With,
};
use bevy::text::{EditableText, TextEdit};
use bevy::ui::Display;

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn handle_command_line_state(
    focus: Res<InputFocus>, keys: Res<ButtonInput<KeyCode>>,
    mut inputs: Query<(&mut EditableText, &mut TextColor)>,
    mut auto_completion_text: Query<(&mut Text, &mut Node), With<CommandAutoCompletion>>,
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

    // Preview
    if current_input != command_state.last_input {
        command_state.last_input = current_input.clone();
        command_state.preview = parse_command_preview(&current_input, &command_state);
        command_state.parse_output = parse_commandline_input(&current_input);
        if !command_state.parse_output.autocompletion.is_empty() {
            println!("{:?}", command_state.parse_output.autocompletion);
        }
        if let Ok((mut ui_text, mut ui_node)) = auto_completion_text.single_mut() {
            ui_text.0 = command_state.parse_output.autocompletion.join("\n");
            ui_node.display = if command_state.parse_output.autocompletion.is_empty() {
                Display::None
            } else {
                Display::Flex
            };
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

    // Submit
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
