use crate::collision::{ColliderShape, ColliderTypeA};
use crate::consts;
pub(crate) use crate::tower::{Tower, TowerRangeMap};
use bevy::asset::AssetServer;
use bevy::math::U16Vec2;
use bevy::prelude::*;
use map_parsing::GameMap;

#[derive(Resource, Clone)]
pub struct Map {
    pub enemies: usize,
    pub towers: Vec<U16Vec2>,
}

impl Default for Map {
    fn default() -> Self {
        Self { enemies: consts::ENEMY_COUNT, towers: vec![] }
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
            GameMap::load(
                consts::assets::maps::one_bit_castle::LOGIC_LAYER,
                consts::MAP_SIZE_TILES.into(),
            )
            .expect("Could not load game map"),
        )
    }
}

pub fn spawn_map(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let map = Map::default();
    commands.insert_resource(map.clone());
    let enemy_count = map.enemies;
    for index in 0..enemy_count {
        commands.spawn((
            Enemy { path_offset: index as f32 / enemy_count as f32, path_progress: 0.0 },
            ColliderTypeA(ColliderShape::circle(consts::ENEMY_RADIUS)),
            Sprite {
                image: asset_server
                    .load(consts::assets::resource_packs::base_pack::towers::gatling_tower::S0_0_0),
                custom_size: consts::ENEMY_SIZE_TILES.into(),
                image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, consts::rendering_layers::ENTITY),
        ));
    }
}
