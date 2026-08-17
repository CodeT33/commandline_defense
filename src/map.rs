use crate::consts;
use bevy::asset::AssetServer;
use bevy::prelude::*;

pub struct Map {
    enemies: Vec<[u16; 2]>,
    towers: Vec<[u16; 2]>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            enemies: vec![[0, 0], [1, 2], [5, 5], [30, 15]],
            towers: vec![[7, 2], [6, 5], [5, 3], [31, 15]],
        }
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Tower;

#[derive(Component)]
pub struct MapVisualLayer;

pub fn spawn_map(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let map = Map::default();

    let map_visual_layer_size = Vec2{x: consts::MAP_SIZE_TILES[0] as f32, y: consts::MAP_SIZE_TILES[1] as f32};

    commands.spawn((MapVisualLayer, Sprite {
        image: asset_server.load(consts::paths::map::MAP_VISUAL_LAYER), custom_size: map_visual_layer_size.into(), ..default()
    },
        Transform::from_xyz((consts::MAP_SIZE_TILES[0]/2) as f32, (consts::MAP_SIZE_TILES[1]/2) as f32, 0.0)
    ));

    for &enemy_pos in &map.enemies {
        commands.spawn((
            Enemy,
            Sprite {
                image: asset_server.load(consts::paths::resources::enemies::ENEMY_1),
                custom_size: consts::ENEMY_SIZE_TILES.into(),
                image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
                ..default()
            },
            Transform::from_xyz(enemy_pos[0] as f32 + 0.5, enemy_pos[1] as f32 + 0.5, 0.0),
        ));
    }
    for &tower_pos in &map.towers {
        commands.spawn((
            Tower,
            Sprite {
                image: asset_server.load(consts::paths::resources::towers::assault_tower::ASSAULT_TOWER_0_0_0),
                custom_size: consts::TOWER_SIZE_TILES.into(),
                image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
                ..default()
            },
            Transform::from_xyz(tower_pos[0] as f32 + 0.5, tower_pos[1] as f32 + 0.5, 0.0),
        ));
    }
}
