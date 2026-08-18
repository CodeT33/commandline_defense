use bevy::input_focus::tab_navigation::{TabGroup, TabIndex};
use bevy::input_focus::{AutoFocus, InputFocus};
use bevy::math::U16Vec2;
use bevy::prelude::*;
use bevy::text::{EditableText, TextCursorStyle};

use crate::consts;
use crate::map::{Map, TowerRangeMap};

#[derive(Resource, Default)]
pub struct HighlightState {
    spawned: Vec<Entity>,
    last_command: Option<String>,
    requested: Option<[u16; 2]>,
    highlighted: Option<[u16; 2]>,
}

#[derive(Component)]
struct RangeHighlight;

fn parse_tower_index(input: &str) -> Option<usize> {
    if !input.trim_start().starts_with("show") {
        return None;
    }
    input
        .split_whitespace()
        .find_map(|w| w.strip_prefix('t')?.parse::<usize>().ok())
        .map(|n| n.saturating_sub(1))
}

pub fn parse_commandline_input(
    inputs: Query<&EditableText>, map: Res<Map>, mut state: ResMut<HighlightState>,
) {
    let Some(input) = inputs.iter().next() else { return };
    let text = input.value().to_string();

    if state.last_command.as_deref() == Some(text.as_str()) {
        return;
    }
    state.last_command = Some(text.clone());

    state.requested = parse_tower_index(&text).and_then(|x| map.towers.get(x)).copied();
}

pub fn highlight_tower_range(
    mut commands: Commands, tower_range_map: Res<TowerRangeMap>, mut state: ResMut<HighlightState>,
) {
    if state.highlighted == state.requested {
        return;
    }

    for e in state.spawned.drain(..) {
        commands.entity(e).despawn();
    }

    let Some(tower_pos) = state.requested else {
        state.highlighted = None;
        return;
    };
    state.highlighted = Some(tower_pos);

    let (min, max) = tower_range_map.range_bounds(tower_pos, consts::TOWER_RANGE_TILES);
    let center = U16Vec2::from_array(tower_pos);

    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let color = if (x, y) == (center.x, center.y) {
                Color::srgba(1.0, 0.2, 0.2, 0.45)
            } else {
                Color::srgba(1.0, 1.0, 0.2, 0.45)
            };
            let e = commands
                .spawn((
                    Sprite { color, custom_size: Some(Vec2::splat(1.0)), ..default() },
                    Transform::from_xyz(x as f32 + 0.5, y as f32 + 0.5, -1.0),
                    RangeHighlight,
                ))
                .id();
            state.spawned.push(e);
        }
    }
}

#[allow(unused)]
pub fn spawn_text_input(commands: &mut Commands) {
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
