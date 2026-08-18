use crate::bullets::BulletEmissionData;
use crate::consts;
use avian2d::prelude::*;
use bevy::asset::AssetServer;
use bevy::ecs::entity::EntityHashSet;
use bevy::math::U16Vec2;
use bevy::prelude::*;
use map_parsing::GameMap;

pub struct Map {
    enemies: usize,
    towers: Vec<[u16; 2]>,
}

impl Default for Map {
    fn default() -> Self {
        Self { enemies: 40, towers: (5..15).map(|x| [x, 3]).collect() }
    }
}

#[derive(Component)]
pub struct Enemy {
    pub path_offset: f32,
    pub path_progress: f32,
}

#[derive(Component, Default)]
pub struct Tower {
    pub enemies_in_range: EntityHashSet,
}

#[derive(Resource)]
pub struct TowerRangeMap {
    pub size: U16Vec2,
    towers_in_range: Vec<Vec<Entity>>,
}

impl Default for TowerRangeMap {
    fn default() -> Self {
        let size = U16Vec2::from_array(consts::MAP_SIZE_TILES);
        Self { size, towers_in_range: vec![Vec::new(); (size.x * size.y) as usize] }
    }
}

#[derive(Resource)]
pub struct MapResource(pub GameMap);

pub fn spawn_map(
    commands: &mut Commands, asset_server: Res<AssetServer>,
    mut tower_range_map: ResMut<TowerRangeMap>,
) {
    let game_map =
        GameMap::load(r"assets\maps\backrooms\logic_layer.png", consts::MAP_SIZE_TILES.into())
            .expect("Could not load game map");

    commands.insert_resource(MapResource(game_map));

    let map = Map::default();
    let enemy_count = map.enemies;
    for index in 0..enemy_count {
        commands.spawn((
            Enemy { path_offset: index as f32 / enemy_count as f32, path_progress: 0.0 },
            Collider::circle(consts::ENEMY_RADIUS),
            Sprite {
                image: asset_server.load(consts::paths::sprite::ENEMY),
                custom_size: consts::ENEMY_SIZE_TILES.into(),
                ..default()
            },
            Transform::default(),
        ));
    }

    for &tower_pos in &map.towers {
        let mut data = BulletEmissionData::default();
        data.direction = Rot2::degrees(180.0);
        Tower::spawn(commands, &asset_server, tower_pos, data, &mut tower_range_map);
    }
}

impl Tower {
    fn spawn(
        commands: &mut Commands, asset_server: &Res<AssetServer>, tower_pos: [u16; 2],
        data: BulletEmissionData, tower_range_map: &mut ResMut<TowerRangeMap>,
    ) {
        let entity = commands
            .spawn((
                Tower::default(),
                Sprite {
                    image: asset_server.load(consts::paths::sprite::TURRET),
                    custom_size: consts::TOWER_SIZE_TILES.into(),
                    image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
                    ..default()
                },
                data,
                Transform::from_xyz(tower_pos[0] as f32 + 0.5, tower_pos[1] as f32 + 0.5, 0.0),
            ))
            .id();
        tower_range_map.add_range_rect(tower_pos, consts::TOWER_RANGE_TILES, entity);
    }
}

impl TowerRangeMap {
    pub fn clear(&mut self) {
        for x in &mut self.towers_in_range {
            x.clear();
        }
    }

    pub fn add_range_rect(&mut self, pos_tiles: [u16; 2], range_tiles: u16, entity: Entity) {
        let center = U16Vec2::from_array(pos_tiles);
        let range = U16Vec2::from_array([range_tiles; 2]);
        let min = center.saturating_sub(range);
        let max = center.saturating_add(range).min(self.size.saturating_sub(U16Vec2::ONE));
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                self.towers_in_range[(y * self.size.x + x) as usize].push(entity);
            }
        }
    }

    pub fn towers_in_range_at(&self, tile: U16Vec2) -> &[Entity] {
        let index = tile.y as usize * self.size.x as usize + tile.x as usize;
        &self.towers_in_range[index]
    }
}
