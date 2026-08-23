use bevy::input_focus::tab_navigation::{TabGroup, TabIndex};
use bevy::input_focus::{AutoFocus, InputFocus};
use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle, TextEdit};

#[derive(Resource, Default)]
pub struct CommandHistory {
    pub(crate) entries: Vec<String>,
    pub(crate) idx: usize,
}

pub fn spawn_command_line(commands: &mut Commands) {
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
