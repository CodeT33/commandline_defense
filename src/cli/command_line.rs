use crate::consts::BEVY_UI_BACKGROUND_COLOR;
use crate::consts::BEVY_UI_BORDER_RADIUS;
use crate::ecs_elements::components::CommandAutoCompletion;
use crate::ecs_elements::resources::CommandHistory;
use bevy::input_focus::tab_navigation::{TabGroup, TabIndex};
use bevy::input_focus::{AutoFocus, InputFocus};
use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle, TextEdit};
use bevy::ui::{ComputedNode, UiGlobalTransform, widget::TextScroll};
use bevy_egui::EguiGlobalSettings;
use parley::{Affinity, Cursor};

pub(crate) fn spawn_command_line(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::End,
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Start,
                padding: px(8.0).all(),
                row_gap: px(0),
                column_gap: px(0),
                ..default()
            },
            TabGroup::new(0),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    padding: px(8).all(),
                    border: px(0).all(),
                    align_items: AlignItems::Center,
                    border_radius: BEVY_UI_BORDER_RADIUS,
                    ..default()
                },
                BackgroundColor(BEVY_UI_BACKGROUND_COLOR),
                EditableText { visible_width: Some(16.0), allow_newlines: false, ..default() },
                TextFont { font_size: FontSize::Px(20.0), ..default() },
                TextColor(Color::WHITE),
                TextCursorStyle::default(),
                TabIndex(0),
                AutoFocus,
            ));
            parent.spawn((
                CommandAutoCompletion,
                Node {
                    padding: px(8).all(),
                    border: px(0).all(),
                    align_items: AlignItems::Center,
                    display: Display::None,
                    border_radius: BEVY_UI_BORDER_RADIUS,
                    ..default()
                },
                BackgroundColor(BEVY_UI_BACKGROUND_COLOR),
                Text("Hello\nidk".to_owned()),
                TextFont { font_size: FontSize::Px(20.0), ..default() },
                TextColor(Color::WHITE),
                TextCursorStyle::default(),
            ));
        });
}

pub(crate) fn block_egui_keyboard_input_when_console_focused(
    focus: Res<InputFocus>, inputs: Query<(), With<EditableText>>,
    mut egui_settings: ResMut<EguiGlobalSettings>,
) {
    let console_focused = focus.get().is_some_and(|entity| inputs.contains(entity));
    egui_settings.input_system_settings.run_write_keyboard_input_messages_system = !console_focused;
}

pub(crate) fn navigate_command_history(
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

/// Written using AI
/// Logical window-space position of the caret sitting at `char_index` in `input`,
/// in the same units as [`Val::Px`](Val). Multiply-based UI positions
/// (e.g. `Node.left`) take these directly; UI layout itself is physical, hence
/// the [`ComputedNode::inverse_scale_factor`] at the end.
///
/// `char_index` counts characters (not bytes) into the input's text buffer.
/// The entity's [`ComputedNode`], [`UiGlobalTransform`] and optional [`TextScroll`]
/// are needed to lift the glyph-local caret position into window coordinates.
/// Returns `None` until the text has been laid out at least once.
pub(crate) fn cursor_screen_position(
    char_index: usize, input: &EditableText, node: &ComputedNode, transform: &UiGlobalTransform,
    scroll: Option<&TextScroll>,
) -> Option<Vec2> {
    let editor = input.editor();
    let layout = editor.try_layout()?;
    let text = editor.raw_text();
    let byte_index = text.char_indices().nth(char_index).map_or(text.len(), |(index, _)| index);

    let bounds =
        Cursor::from_byte_index(layout, byte_index, Affinity::Downstream).geometry(layout, 0.0);
    let local = Vec2::new(bounds.x0 as f32, bounds.y0 as f32);

    let content_origin = node.content_box().min - scroll.map_or(Vec2::ZERO, |scroll| scroll.0);
    let physical = transform.affine().transform_point2(content_origin + local);
    Some(physical * node.inverse_scale_factor())
}
