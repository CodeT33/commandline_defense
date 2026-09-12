use crate::consts;
use crate::ecs_elements::resources::{DebugSettings, GameState, PlayerSuiteResource};
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
            border_radius: consts::ui::BEVY_UI_BORDER_RADIUS,
            ..default()
        },
        TabGroup::new(0),
        Text("...".parse().unwrap()),
        TextFont { font_size: FontSize::Px(20.0), ..default() },
        BackgroundColor(consts::ui::BEVY_UI_BACKGROUND_COLOR),
        PlayerSuiteUi,
    ));
}

pub(crate) fn update_player_suite_ui(
    mut query: Query<&mut Text, With<PlayerSuiteUi>>, player_suite: Res<PlayerSuiteResource>,
    debug_settings: Res<DebugSettings>, game_state: Res<GameState>,
) {
    let paused = game_state.waves.is_waiting_for_resume() || debug_settings.paused;

    let paused_text = if paused { "\nGame paused" } else { "" };

    let wave_text = if paused {
        format!("Next wave: {}", player_suite.next_wave)
    } else {
        format!("Current wave: {}", player_suite.next_wave)
    };

    for mut text in &mut query {
        **text = format!(
            "Balance: ${}\nHealth: {}\n\n{}{}",
            player_suite.money, player_suite.health, wave_text, paused_text
        );
    }
}
