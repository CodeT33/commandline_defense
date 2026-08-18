use crate::grid::{get_number_from_letter, SelectionState};
use bevy::input_focus::{AutoFocus, InputFocus};
use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle};

#[derive(Resource, Default)]
pub struct CommandState {
    pub preview: PreviewCommand,
    pub last_input: String,
}

#[derive(Message, Debug)]
pub enum CommandEvent {
    Select {tile: Vec2},
    Deselect,
    Path,
    ExitGame,
}

#[derive(Default)]
pub enum PreviewCommand {
    #[default]
    None,
    ShowGrid,
    HighlightTile {
        tile: Vec2,
    },
    HighlightPath,
}

pub enum Command {
    Select(Vec2),
    Deselect,
    Path,
    ExitGame
}

pub fn spawn_command_line(commands: &mut Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,

            left: px(16.0),
            right: px(16.0),
            bottom: px(10.0),

            height: px(32.0),

            padding: UiRect { left: px(8.0), right: px(8.0), bottom: px(4.0), top: px(4.0) },
            border: UiRect::all(px(2.0)),
            border_radius: BorderRadius::all(px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        EditableText {
            cursor_width: 0.25,
            allow_newlines: false,
            ..default()
        },
        TextCursorStyle::default(),
        AutoFocus,
    ));
}

pub fn handle_command_line_state(
    focus: Res<InputFocus>, keys: Res<ButtonInput<KeyCode>>, mut inputs: Query<&mut EditableText>, mut command_state: ResMut<CommandState>, mut command_events: MessageWriter<CommandEvent>,
) {
    let Some(entity) = focus.get() else {
        return;
    };
    let Ok(mut input) = inputs.get_mut(entity) else {
        return;
    };

    let current_input = input.value().to_string();

    if current_input != &*command_state.last_input {
        command_state.last_input = current_input.to_string();
        command_state.preview = parse_command_preview(&current_input);
        println!("Preview changed: {:?}", current_input)
    }

    if keys.just_pressed(KeyCode::Enter) {
        if let Some(command) = parse_command(&*current_input) {
            send_command_event(
                command,
                &mut command_events,
            );
        }

        input.clear();

        command_state.last_input.clear();
        command_state.preview = PreviewCommand::None;
    }
}

fn send_command_event(command: Command, events: &mut MessageWriter<CommandEvent>,) {
    match command {
        Command::Select {..} => {

        },
        Command::Deselect => {
            events.write(CommandEvent::Deselect);
        },
        Command::Path => {
            events.write(CommandEvent::Path);
        }
        Command::ExitGame => {
            events.write(CommandEvent::ExitGame);
        }
    }
}

fn send_select_event(tile: Vec2, events: &mut MessageWriter<CommandEvent>,) {
    events.write(CommandEvent::Select {tile});
}

pub fn handle_command_events(mut events: MessageReader<CommandEvent>, mut selection_state: ResMut<SelectionState>) {
    for event in events.read() {
        match event {
            CommandEvent::Select {tile} => {
                selection_state.selected_tile = Some(*tile);

                println!("Selected tile: {:?}", tile);
            },
            CommandEvent::Deselect => {
                selection_state.selected_tile = None;

                println!("Deselected tiles");
            }
            CommandEvent::Path => {
                println!("Show path");
            }
            CommandEvent::ExitGame => {
                println!("ExitGame");
            }
        }
    }
}

fn parse_command_preview(input: &str,) -> PreviewCommand {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    match tokens.as_slice() {
        ["select"] => {
            PreviewCommand::ShowGrid
        }
        ["select", position] => {
            match parse_tile_position(position) {
                Some(tile) => {
                    PreviewCommand::HighlightTile {
                        tile,
                    }
                }
                None => {
                    PreviewCommand::ShowGrid
                }
            }
        }
        _ => {
            PreviewCommand::None
        }
    }
}

fn parse_command(input: &str) -> Option<Command> {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    match tokens.as_slice() {
        ["select", position] => {
            let tile = parse_tile_position(position)?;
            println!("Selected tile: {:?}", tile);
            Some(Command::Select(tile))
        }
        ["deselect"] => {
            println!("Deselected");
            Some(Command::Deselect)
        }
        ["path"] => {
            println!("Show path");
            Some(Command::Path)
        }
        ["exit", "game"] => {
            println!("Exit game");
            Some(Command::ExitGame)
        }
        _ => {
            println!("Unknown command: {:?}", input);
            None
        }
    }
}

/*

select (temp show grid and grid positions)

select 4c (highlight c4)

deselect (reset highlighting)

path (temp highlights the path)

exit game (exit game)

 */

fn parse_tile_position(position: &str) -> Option<Vec2> {
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

    Some(Vec2::new(x as f32, y as f32))
}


