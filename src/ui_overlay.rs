use bevy::prelude::Commands;
use crate::ui_overlay::grid::{spawn_grid, spawn_grid_positions};
use crate::ui_overlay::selection::spawn_tile_highlight;

pub mod grid;
pub mod selection;

pub fn spawn_ui_overlay(commands: &mut Commands) {
    spawn_grid(commands);
    spawn_grid_positions(commands);
    spawn_tile_highlight(commands);
}