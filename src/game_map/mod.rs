use macros::dir_structure_as_enum_absolute_paths;

pub mod map;
pub mod map_logic_parsing;
pub mod map_rendering;

dir_structure_as_enum_absolute_paths!(MapLogicLayers, "assets/map_logic_layers");
