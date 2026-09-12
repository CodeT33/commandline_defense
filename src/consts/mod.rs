mod bullets;
mod enemies;
mod game_waves;
mod towers;
pub(crate) mod ui;
mod value_tiers;

use bevy::math::U16Vec2;
use bevy::prelude::{BorderRadius, Color, Val, Vec2};

pub(crate) const WINDOW_TITLE: &str = "Commandline Defense";
pub(crate) const WINDOW_RESOLUTION: [u32; 2] = [800, 450];

pub(crate) const PHYSICS_FRAME_RATE: u16 = 144 * 2;
pub(crate) const MAX_SIM_SPEED: f32 = 20.0;

pub(crate) const MAP_SIZE_TILES: U16Vec2 = U16Vec2 { x: 32, y: 16 };
pub(crate) const TILE_SIZE: u16 = 16;

pub(crate) const TOWER_COOLDOWN_MS: u32 = 1000;

pub(crate) const PROJECTILE_SIZE_TILES: Vec2 = Vec2::splat(1.0);

pub(crate) const BULLET_ROTATION_DURATION_MS: u64 = 234;

pub(crate) const COMMAND_OPEN_SEPARATION_CHARACTER: char = '/';

/// Number of simulated seconds captured before the log is saved and the program is terminated.
#[cfg(feature = "determinism")]
pub(crate) const LOG_DURATION_SECS: u64 = 15;

pub(crate) mod viewports {
    use crate::camera::Viewport;

    pub(crate) const BASIC_CAMERA: Viewport =
        Viewport { min_zoom: 0.01, max_zoom: 0.5, zoom_speed: 0.1 };
}

pub(crate) mod map_logic_parsing {
    pub(crate) const PATH_START: u32 = 0xff00ff;
    pub(crate) const PATH: u32 = 0xffff00;
    pub(crate) const RESTRICTED: u32 = 0xff0000;
    pub(crate) const PLACEABLE: u32 = 0x00ff00;
    pub(crate) const WATER: u32 = 0x0000ff;
}

pub(crate) mod rendering_layers {
    pub(crate) const MAP: f32 = 0.0;
    pub(crate) const CONTRAST: f32 = 1.0;
    pub(crate) const ENTITY: f32 = 5.0;
    pub(crate) const GRID: f32 = 10.0;
    pub(crate) const GRID_LABEL: f32 = 11.0;
    pub(crate) const HEALTH_BARS: f32 = 15.0;
    pub(crate) const HIGHLIGHT: f32 = 20.0;
}

pub(crate) const BASE_TEXTURE_PACK_PATH: &str = "assets/texture_packs/default";
