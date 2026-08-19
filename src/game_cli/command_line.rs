use bevy::input_focus::{AutoFocus, InputFocus};
use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle, TextEdit};

#[derive(Resource, Default)]
pub struct CommandHistory {
    pub(crate) entries: Vec<String>,
    pub(crate) idx: usize,
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
        EditableText { cursor_width: 0.25, allow_newlines: false, ..default() },
        TextCursorStyle::default(),
        AutoFocus,
    ));
}

pub fn navigate_command_history(
    focus: Res<InputFocus>, keys: Res<ButtonInput<KeyCode>>, mut inputs: Query<&mut EditableText>,
    mut history: ResMut<CommandHistory>,
) {
    let direction = match keys.just_pressed(KeyCode::ArrowUp) {
        true => -1,
        false if keys.just_pressed(KeyCode::ArrowDown) => 1,
        _ => return,
    };
    if history.entries.is_empty() {
        return;
    }
    let Some(entity) = focus.get() else {
        return;
    };
    let Ok(mut input) = inputs.get_mut(entity) else {
        return;
    };

    history.idx =
        (history.idx as isize + direction).clamp(0, history.entries.len() as isize) as usize;
    match history.entries.get(history.idx) {
        Some(command) => set_input_text(&mut input, command),
        None => input.clear(),
    }
}

fn set_input_text(input: &mut EditableText, text: &str) {
    input.editor_mut().set_text(text);
    input.queue_edit(TextEdit::TextEnd(false));
}

/*

select (temp show grid and grid positions)

select 4c (highlight c4)

deselect (reset highlighting)

path (temp highlights the path)

exit game (exit game)

 */
