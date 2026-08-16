use bevy::input_focus::tab_navigation::{TabGroup, TabIndex};
use bevy::input_focus::{AutoFocus, InputFocus};
use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle};

pub fn spawn_text_input(commands: &mut Commands) {
    commands.spawn((
        Text::new("click the box, type, Enter to print"),
        TextFont { font_size: FontSize::Px(16.0), ..default() },
        Node { ..default() },
    ));

    commands.spawn((
        Node {
            padding: px(8).all(),
            border: px(2).all(),
            align_items: AlignItems::Center,
            ..default()
        },
        EditableText { visible_width: Some(16.0), allow_newlines: false, ..default() },
        TextFont { font_size: FontSize::Px(24.0), ..default() },
        TextColor(Color::WHITE),
        TextCursorStyle::default(),
        BackgroundColor::default(),
        TabGroup::new(0),
        TabIndex(0),
        AutoFocus,
    ));
}

pub fn submit_text(
    focus: Res<InputFocus>, keys: Res<ButtonInput<KeyCode>>, mut inputs: Query<&mut EditableText>,
) {
    if !keys.just_pressed(KeyCode::Enter) {
        return;
    }
    let Some(entity) = focus.get() else {
        return;
    };
    let Ok(mut input) = inputs.get_mut(entity) else {
        return;
    };

    println!("{}", input.value());
    input.clear();
}
