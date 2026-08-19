use bevy::asset::AssetServer;
use bevy::prelude::{default, Commands, Res, Sprite, SpriteImageMode, SpriteScalingMode, Transform, Vec2};
use map_parsing;
use crate::consts;

pub fn spawn_map_visual_layer(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load(consts::paths::map::MAP_VISUAL_LAYER),
            custom_size: Option::from(Vec2::splat((consts::TILE_SIZE*2) as f32)),
            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
            ..default()
        },
        Transform::from_xyz(consts::MAP_SIZE_TILES[0] as f32 / 2.0, consts::MAP_SIZE_TILES[1] as f32 / 2.0, 0.0),
        ));
}