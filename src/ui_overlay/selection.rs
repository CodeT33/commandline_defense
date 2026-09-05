use crate::consts::{self, TILE_SIZE};

use crate::ecs_elements::components::TileHighlight;
use crate::ecs_elements::resources::{CommandState, SelectionState, TexturePackSettings};
use crate::game_cli::command_line_state_management::PreviewCommand;
use crate::texture_packs::TexturePackAssets;
use bevy::asset::AssetServer;
use bevy::math::Vec2;
use bevy::prelude::{
    Commands, Query, Res, Sprite, SpriteImageMode, SpriteScalingMode, Transform, Visibility, With,
    default,
};

pub fn spawn_tile_highlight(
    commands: &mut Commands, asset_server: &AssetServer,
    texture_pack_settings: &TexturePackSettings,
) {
    commands.spawn((
        Sprite {
            image: asset_server.load(
                texture_pack_settings
                    .get_asset_path(TexturePackAssets::WipSprites_SelectionSquareArrowTop),
            ),
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
            transform.translation.x = tile.position.x as f32 + 0.5;
            transform.translation.y = tile.position.y as f32 + 0.5;

            *visibility = Visibility::Visible
        },
        None => *visibility = Visibility::Hidden,
    }
}
