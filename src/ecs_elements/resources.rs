use crate::cli::command_input::ParseOutput;
use crate::cli::preview::PreviewCommand;
use crate::coordinates::GridCoordinate;
use crate::map::map_logic_parsing::GameMap;
use crate::ui_overlay::ui_state::UiState;
use crate::waves::GameWaves;
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
    pub(crate) persistent_preview: Option<UiState>,
    pub(crate) last_input: String,
    pub(crate) parse_output: ParseOutput,
}

#[derive(Resource)]
pub(crate) struct PlayerSuiteResource {
    pub(crate) health: u16,
    pub(crate) shield: u16,
    pub(crate) points: u16,
    pub(crate) money: u16,
    pub(crate) next_wave: u16,
}

#[derive(Resource)]
pub(crate) struct TexturePackSettings {
    pub(crate) base_path: String,
}

#[derive(Resource, Default)]
pub(crate) struct SelectionState {
    pub(crate) selected_tile: Option<GridCoordinate>,
}

#[derive(Resource, Default)]
pub(crate) struct UiHover(pub(crate) bool);

#[derive(Resource)]
pub(crate) struct DebugSettings {
    pub(crate) enable_bounding_boxes: bool,
    pub(crate) sim_speed: f32,
    pub(crate) paused: bool,
}

#[allow(unused)]
#[derive(Resource)]
pub(crate) struct GameState {
    pub(crate) waves: GameWaves,
}
