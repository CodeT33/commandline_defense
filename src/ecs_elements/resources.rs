use crate::cli::command_line_state_management::PreviewCommand;
use crate::consts;
use crate::coordinates::GridCoordinate;
use crate::map::map_logic_parsing::GameMap;
use bevy::prelude::{Deref, DerefMut, Resource};

#[derive(Resource, Deref, DerefMut)]
pub(crate) struct MapResource(pub(crate) GameMap);

#[derive(Resource, Default)]
pub(crate) struct CommandHistory {
    pub(crate) entries: Vec<String>,
    pub(crate) idx: usize,
}

#[derive(Resource, Default)]
pub(crate) struct CommandState {
    pub(crate) preview: PreviewCommand,
    pub(crate) last_input: String,
}

#[derive(Resource)]
pub(crate) struct PlayerSuiteResource {
    pub(crate) health: u16,
    pub(crate) shield: u16,
    pub(crate) points: u16,
    pub(crate) money: u16,
}

#[derive(Resource)]
pub(crate) struct TexturePackSettings {
    pub(crate) base_path: String,
}

#[derive(Resource, Default)]
pub(crate) struct SelectionState {
    pub(crate) selected_tile: Option<GridCoordinate>,
}

#[derive(Resource)]
pub(crate) struct DebugSettings {
    pub(crate) enable_bounding_boxes: bool,
    pub(crate) enemy_spawn_interval_ms: u64,
    pub(crate) sim_speed: f32,
}

impl Default for DebugSettings {
    fn default() -> Self {
        Self {
            enable_bounding_boxes: false,
            enemy_spawn_interval_ms: consts::ENEMY_SPAWN_INTERVAL_MS,
            sim_speed: 1.0,
        }
    }
}
