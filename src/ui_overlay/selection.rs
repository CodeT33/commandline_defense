use crate::consts;
use crate::consts::{MAP_SIZE_TILES, TILE_SIZE};
use crate::game_cli::command_line_state_management::{CommandState, PreviewCommand};
use avian2d::parry::glamx::{Vec2, Vec3};
use bevy::prelude::{
    Commands, Component, Query, Res, Resource, Sprite, Transform, Visibility, With, default,
};

#[derive(Resource, Default)]
pub struct SelectionState {
    pub selected_tile: Option<Vec2>,
}

#[derive(Component)]
pub struct TileHighlight;

pub fn spawn_tile_highlight(commands: &mut Commands) {
    commands.spawn((
        Sprite {
            color: consts::ui::grid::TILE_HIGHLIGHT_COLOR,
            custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
            ..default()
        },
        Transform::from_xyz(8.0, 8.0, 20.0).with_scale(Vec3::splat(0.08)),
        TileHighlight,
    ));
}

pub fn update_selected_tile(
    command_state: Res<CommandState>, selection_state: Res<SelectionState>,
    mut highlight: Query<(&mut Transform, &mut Visibility), With<TileHighlight>>,
) {
    let Ok((mut transform, mut visibility)) = highlight.single_mut() else {
        return;
    };

    let tile = match &command_state.preview {
        PreviewCommand::HighlightTile { tile } => Some(*tile),
        _ => selection_state.selected_tile,
    };

    match tile {
        Some(tile) => {
            transform.translation.x = tile.x + 0.5;
            transform.translation.y = (MAP_SIZE_TILES[1] as f32) - tile.y - 0.5;

            *visibility = Visibility::Visible
        },
        None => *visibility = Visibility::Hidden,
    }
}
