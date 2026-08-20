use crate::consts;
use crate::ui_overlay::grid::{spawn_grid, spawn_grid_positions};
use crate::ui_overlay::selection::spawn_tile_highlight;
use avian2d::parry::glamx::Vec2;
use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Sprite, SpriteImageMode, SpriteScalingMode, Transform, default, Res};
use crate::map::MapResource;

pub mod grid;
pub mod selection;

pub fn spawn_ui_overlay(commands: &mut Commands, asset_server: &AssetServer, map_resource: &Res<MapResource>) {
    spawn_grid(commands);
    spawn_grid_positions(commands, map_resource);
    spawn_tile_highlight(commands, asset_server);
    spawn_map_border(commands, asset_server)
}

pub fn spawn_map_border(commands: &mut Commands, asset_server: &AssetServer) {
    commands.spawn((
        Sprite {
            image: asset_server.load(consts::paths::map::MAP_BORDER),
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
