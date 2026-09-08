use crate::consts;
use crate::ecs_elements::components::{ColliderShape, Map};
use crate::ecs_elements::resources::MapResource;
use crate::map::map_logic_parsing::GameMap;
use bevy::prelude::{Commands, Transform};
use macros::dir_structure_as_enum_absolute_paths;

pub(crate) mod map_logic_parsing;
pub(crate) mod map_rendering;

dir_structure_as_enum_absolute_paths!(MapLogicLayers, "assets/map_logic_layers");

impl Default for MapResource {
    fn default() -> Self {
        MapResource(
            GameMap::load(MapLogicLayers::ShipYard, consts::MAP_SIZE_TILES)
                .expect("Could not load game map"),
        )
    }
}

pub(crate) fn spawn_map_bounds(commands: &mut Commands, map: &MapResource) {
    let map_size = map.map_tiles().map_size().as_vec2();
    commands.spawn((
        Map,
        ColliderShape::rect(map_size),
        Transform::from_translation((map_size / 2.0).extend(0.0)),
    ));
}
