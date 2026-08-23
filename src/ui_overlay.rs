use crate::consts;
use crate::game_map::map::MapResource;
use crate::ui_overlay::grid::{spawn_contrast_overlay, spawn_grid, spawn_grid_positions};
use crate::ui_overlay::selection::spawn_tile_highlight;
use bevy::asset::AssetServer;
use bevy::prelude::{
    Commands, Res, Sprite, SpriteImageMode, SpriteScalingMode, Transform, Vec2, default,
};

pub mod debug;
pub mod grid;
pub mod selection;

pub fn spawn_ui_overlay(
    commands: &mut Commands, asset_server: &AssetServer, map_resource: &Res<MapResource>,
) {
    spawn_contrast_overlay(commands, map_resource);
    spawn_grid(commands, map_resource);
    spawn_grid_positions(commands, map_resource);
    spawn_tile_highlight(commands, asset_server);
    spawn_map_border(commands, asset_server);
}

pub fn spawn_map_border(commands: &mut Commands, asset_server: &AssetServer) {
    commands.spawn((
        Sprite {
            image: asset_server.load(consts::assets::sprites::BORDER_TEST),
            custom_size: Option::from(Vec2::splat((consts::TILE_SIZE * 2 + 2) as f32)),
            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
            ..default()
        },
        Transform::from_xyz(
            consts::MAP_SIZE_TILES[0] as f32 / 2.0,
            consts::MAP_SIZE_TILES[1] as f32 / 2.0,
            consts::rendering_layers::MAP,
        ),
    ));
}
