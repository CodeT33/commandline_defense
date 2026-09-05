use crate::cli::command_line_state_management::PreviewCommand;
use crate::consts;
use crate::coordinates::GridCoordinate;
use crate::map::map_logic_parsing::GameMap;
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct MapResource(pub GameMap);

#[derive(Resource, Default)]
pub struct CommandHistory {
    pub(crate) entries: Vec<String>,
    pub(crate) idx: usize,
}

#[derive(Resource, Default)]
pub struct CommandState {
    pub preview: PreviewCommand,
    pub last_input: String,
}

#[derive(Resource)]
pub struct PlayerSuiteResource {
    pub health: u16,
    pub shield: u16,
    pub points: u16,
    pub money: u16,
}

#[derive(Resource)]
pub struct TexturePackSettings {
    pub base_path: String,
}

#[derive(Resource, Default)]
pub struct SelectionState {
    pub selected_tile: Option<GridCoordinate>,
}

#[derive(Resource)]
pub struct DebugSettings {
    pub enable_bounding_boxes: bool,
    pub enemy_spawn_interval_ms: u64,
}

impl Default for DebugSettings {
    fn default() -> Self {
        Self {
            enable_bounding_boxes: false,
            enemy_spawn_interval_ms: consts::ENEMY_SPAWN_INTERVAL_MS,
        }
    }
}
