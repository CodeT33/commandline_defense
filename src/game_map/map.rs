use crate::collision::{ColliderShape, ColliderTypeA};
use crate::consts;
use crate::game_map::MapLogicLayers;
use crate::game_map::map_logic_parsing::GameMap;
use crate::texture_packs::{TexturePackAssets, TexturePackSettings};
pub(crate) use crate::tower::{Tower, TowerRangeMap};
use bevy::asset::AssetServer;
use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct Map {
    pub enemies: usize,
}

impl Default for Map {
    fn default() -> Self {
        Self { enemies: consts::ENEMY_COUNT }
    }
}

#[derive(Component)]
pub struct Enemy {
    pub path_offset: f32,
    pub path_progress: f32,
}

#[derive(Resource)]
pub struct MapResource(pub GameMap);

impl Default for MapResource {
    fn default() -> Self {
        MapResource(
            GameMap::load(MapLogicLayers::OneBitCastle, consts::MAP_SIZE_TILES)
                .expect("Could not load game map"),
        )
    }
}
pub fn spawn_map(
    commands: &mut Commands, asset_server: &Res<AssetServer>,
    texture_pack_settings: &TexturePackSettings,
) {
    let map = Map::default();
    commands.insert_resource(map.clone());
    let enemy_count = map.enemies;
    for index in 0..enemy_count {
        commands.spawn((
            Enemy { path_offset: index as f32 / enemy_count as f32, path_progress: 0.0 },
            ColliderTypeA,
            ColliderShape::circle(consts::ENEMY_RADIUS),
            Sprite {
                image: asset_server.load(
                    texture_pack_settings
                        .get_asset_path(&TexturePackAssets::Towers_GatlingTower_000),
                ),
                custom_size: consts::ENEMY_SIZE_TILES.into(),
                image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, consts::rendering_layers::ENTITY),
        ));
    }
}
