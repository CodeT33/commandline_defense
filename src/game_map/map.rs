use crate::consts;
use crate::ecs_elements::resources::{Map, MapResource};
use crate::game_map::MapLogicLayers;
use crate::game_map::map_logic_parsing::GameMap;

impl Default for Map {
    fn default() -> Self {
        Self { enemies: consts::ENEMY_COUNT }
    }
}

impl Default for MapResource {
    fn default() -> Self {
        MapResource(
            GameMap::load(MapLogicLayers::OneBitCastle, consts::MAP_SIZE_TILES)
                .expect("Could not load game map"),
        )
    }
}
