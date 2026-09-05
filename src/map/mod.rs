use crate::consts;
use crate::ecs_elements::resources::MapResource;
use crate::map::map_logic_parsing::GameMap;
use macros::dir_structure_as_enum_absolute_paths;

pub mod map_logic_parsing;
pub mod map_rendering;

dir_structure_as_enum_absolute_paths!(MapLogicLayers, "assets/map_logic_layers");

impl Default for MapResource {
    fn default() -> Self {
        MapResource(
            GameMap::load(MapLogicLayers::OneBitCastle, consts::MAP_SIZE_TILES)
                .expect("Could not load game map"),
        )
    }
}
