use crate::consts;
use crate::consts::{MAP_SIZE_TILES, TILE_SIZE};
use crate::game_cli::command_line_state_management::{CommandState, PreviewCommand};
use avian2d::parry::glamx::Vec2;
use bevy::asset::AssetServer;
use bevy::prelude::{
    Commands, Component, Query, Res, Resource, Sprite, SpriteImageMode, SpriteScalingMode,
    Transform, Visibility, With, default,
};

#[derive(Resource, Default)]
pub struct SelectionState {
    pub selected_tile: Option<Vec2>,
}

#[derive(Component)]
pub struct TileHighlight;

pub fn spawn_tile_highlight(commands: &mut Commands, asset_server: &AssetServer) {
    commands.spawn((
        Sprite {
            image: asset_server.load(consts::assets::sprites::SELECTION_SQUARE_ARROW_TOP),
            custom_size: Option::from(Vec2::splat(TILE_SIZE as f32 / 8.0)),
            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
            ..default()
        },
        Transform::from_xyz(default(), default(), consts::rendering_layers::HIGHLIGHT),
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
