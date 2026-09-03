use crate::consts;
use crate::game_map::map::MapResource;
use crate::texture_packs::TexturePackSettings;
use bevy::asset::AssetServer;
use bevy::prelude::{
    Commands, Sprite, SpriteImageMode, SpriteScalingMode, Transform, Vec2, default,
};
use consts::texture_paths;

pub fn spawn_map_visual_layer(
    commands: &mut Commands, asset_server: &AssetServer, map_resource: &MapResource,
    texture_pack_settings: &TexturePackSettings,
) {
    let map_size = map_resource.0.map_tiles.map_size;

    commands.spawn((
        Sprite {
            image: asset_server.load(
                texture_pack_settings
                    .get_asset_path(texture_paths::map_visual_layers::ONE_BIT_CASTLE),
            ),
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
