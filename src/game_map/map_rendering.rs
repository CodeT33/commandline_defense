use crate::consts;
use crate::game_map::map::MapResource;
use bevy::asset::AssetServer;
use bevy::prelude::{
    Commands, Res, Sprite, SpriteImageMode, SpriteScalingMode, Transform, Vec2, default,
};
use consts::{BASE_TEXTURE_PACK_PATH, asset_path, texture_paths};

pub fn spawn_map_visual_layer(
    commands: &mut Commands, asset_server: &Res<AssetServer>, map_resource: &Res<MapResource>,
) {
    let map_size = map_resource.0.map_tiles.map_size;

    commands.spawn((
        Sprite {
            image: asset_server.load(asset_path(
                BASE_TEXTURE_PACK_PATH,
                texture_paths::maps::one_bit_castle::VISUAL_LAYER,
            )),
            custom_size: Option::from(Vec2::splat((consts::TILE_SIZE * 2) as f32)),
            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
            ..default()
        },
        Transform::from_xyz(
            map_size.x as f32 / 2.0,
            map_size.y as f32 / 2.0,
            consts::rendering_layers::MAP,
        ),
    ));
}
