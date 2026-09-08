use crate::consts;
use crate::ecs_elements::resources::{MapResource, TexturePackSettings};
use crate::texture_packs::TexturePackAssets;
use bevy::asset::AssetServer;
use bevy::prelude::{
    Commands, Sprite, SpriteImageMode, SpriteScalingMode, Transform, Vec2, default,
};

pub(crate) fn spawn_map_visual_layer(
    commands: &mut Commands, asset_server: &AssetServer, map_resource: &MapResource,
    texture_pack_settings: &TexturePackSettings,
) {
    let map_size = map_resource.map_tiles().map_size();

    commands.spawn((
        Sprite {
            image: asset_server.load(
                texture_pack_settings.get_asset_path(TexturePackAssets::MapVisualLayers_ShipYard),
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
