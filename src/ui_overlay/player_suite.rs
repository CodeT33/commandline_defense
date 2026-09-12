use crate::consts::{BEVY_UI_BACKGROUND_COLOR, BEVY_UI_BORDER_RADIUS};
use crate::ecs_elements::resources::{DebugSettings, PlayerSuiteResource};
use bevy::input_focus::tab_navigation::TabGroup;
use bevy::prelude::{
    BackgroundColor, Commands, Component, FontSize, Node, Query, Res, Text, TextFont, With,
    default, px,
};

#[derive(Component)]
pub(crate) struct PlayerSuiteUi;

pub(crate) fn spawn_player_suite_ui(commands: &mut Commands) {
    commands.spawn((
        Node {
            padding: px(8.0).all(),
            margin: px(8.0).all(),
            row_gap: px(0),
            column_gap: px(0),
            border_radius: BEVY_UI_BORDER_RADIUS,
            ..default()
        },
        TabGroup::new(0),
        Text("...".parse().unwrap()),
        TextFont { font_size: FontSize::Px(20.0), ..default() },
        BackgroundColor(BEVY_UI_BACKGROUND_COLOR),
        PlayerSuiteUi,
    ));
}

pub(crate) fn update_player_suite_ui(
    mut query: Query<&mut Text, With<PlayerSuiteUi>>, player_suite: Res<PlayerSuiteResource>,
    debug_settings: Res<DebugSettings>,
) {
    let paused_text = if debug_settings.paused { "\n\nGame paused" } else { "" };

    let wave_text = if debug_settings.paused {
        format!("Next wave: {}", player_suite.next_wave)
    } else {
        format!("Current wave: {}", player_suite.next_wave)
    };

    for mut text in &mut query {
        **text = format!(
            "Balance: ${}\nHealth: {}\n{}{}",
            player_suite.money, player_suite.health, wave_text, paused_text
        );
    }
}
