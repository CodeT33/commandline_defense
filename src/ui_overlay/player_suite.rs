use crate::consts::COMMANDLINE_BACKGROUND_COLOR;
use crate::ecs_elements::resources::PlayerSuiteResource;
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
            ..default()
        },
        TabGroup::new(0),
        Text("...".parse().unwrap()),
        TextFont { font_size: FontSize::Px(20.0), ..default() },
        BackgroundColor(COMMANDLINE_BACKGROUND_COLOR),
        PlayerSuiteUi,
    ));
}

pub(crate) fn update_player_suite_ui(
    mut query: Query<&mut Text, With<PlayerSuiteUi>>, player_suite: Res<PlayerSuiteResource>,
) {
    for mut text in &mut query {
        **text = format!("Balance: ${}\nHealth: {}", player_suite.money, player_suite.health);
    }
}
