use crate::grid::get_number_from_letter;
use bevy::input_focus::tab_navigation::{TabGroup, TabIndex};
use bevy::input_focus::{AutoFocus, InputFocus};
use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle};

#[derive(Resource, Default)]
pub struct CommandPreview {
    pub command: PreviewCommand,
}

#[derive(Resource, Default)]
pub struct CommandState {
    pub preview: PreviewCommand,
    pub last_input: String,
    pub selected_tile: Option<Vec2>,
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
            //visible_width: Some(16.0),
            allow_newlines: false,
            ..default()
        },
        TextCursorStyle::default(),
        AutoFocus,
    ));
}

pub fn handle_command_line_state(
    focus: Res<InputFocus>, keys: Res<ButtonInput<KeyCode>>, mut inputs: Query<&mut EditableText>, mut preview: ResMut<CommandPreview>,
) {
    let Some(entity) = focus.get() else {
        return;
    };
    let Ok(mut input) = inputs.get_mut(entity) else {
        return;
    };

    let command_text = input.value().to_string();

    preview.command = parse_command_preview(&command_text);

    if keys.just_pressed(KeyCode::Enter) {
        parse_command_submissions(&command_text);
        input.clear();
    }
}

/*

select (temp show grid and grid positions)

select 4c (highlight c4)

deselect (reset highlighting)

path (temp highlights the path)

exit game (exit game)

 */

fn parse_command_preview(input: &str) -> PreviewCommand {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    match tokens.as_slice() {
        ["select"] => {
            println!("Showing grid");
            PreviewCommand::ShowGrid
        },
        ["select", position] => {
            println!("Highlighting tile: {:?}", position);
            match parse_tile_position(position) {
                Some(tile) => PreviewCommand::HighlightTile { tile },
                None => PreviewCommand::ShowGrid,
            }
        },
        ["path"] => {
            println!("Highlighting path");
            PreviewCommand::HighlightPath
        },
        _ => PreviewCommand::None,
    }
}

fn parse_tile_position(position: &str) -> Option<Vec2> {
    let position = position.to_ascii_uppercase();

    let mut number = String::new();
    let mut letter = None;

    for character in position.chars() {
        if character.is_ascii_digit() {
            number.push(character);
        } else if character.is_ascii_alphabetic() {
            letter = Some(character);
        } else {
            return None;
        }
    }

    let x: u16 = number.parse().ok()?;
    let y: u16 = get_number_from_letter(letter?)?;

    Some(Vec2::new(x as f32, y as f32))
}

fn parse_command_submissions(command_submission: &str) {
    let tokens: Vec<&str> = command_submission.split_whitespace().collect();

    match tokens.as_slice() {
        ["select", position] => {
            if let Some(tile) = parse_tile_position(position) {
                println!("Selected tile: {:?}", tile);
            }
        },
        ["deselect"] => {
            println!("Deselected");
        },
        ["path"] => {
            println!("Show path");
        },
        ["exit", "game"] => {
            println!("Exit game");
        },
        _ => {
            println!("Unknown command: {:?}", command_submission);
        },
    }
}
