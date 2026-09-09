use crate::consts;
use crate::ecs_elements::components::{ColliderShape, Map};
use crate::ecs_elements::resources::MapResource;
use crate::map::map_logic_parsing::GameMap;
use bevy::prelude::{Commands, Transform};
use macros::dir_structure_as_enum_absolute_paths;

pub mod map_logic_parsing;
pub mod map_rendering;

dir_structure_as_enum_absolute_paths!(MapLogicLayers, "assets/map_logic_layers");

impl Default for MapResource {
    fn default() -> Self {
        MapResource(
            GameMap::load(MapLogicLayers::MagicOfLeavesMap1, consts::MAP_SIZE_TILES)
                .expect("Could not load game map"),
        )
    }
}

pub fn spawn_map_bounds(commands: &mut Commands, map: &MapResource) {
    let map_size = map.0.map_tiles().map_size.as_vec2();
    commands.spawn((
        Map,
        ColliderShape::rect(map_size),
        Transform::from_translation((map_size / 2.0).extend(0.0)),
    ));
}
