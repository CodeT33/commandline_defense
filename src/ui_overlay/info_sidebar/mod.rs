use crate::cli::command_line_state_management::PreviewCommand;
use crate::ecs_elements::resources::CommandState;
use crate::ui_overlay::ui_state::UiState;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use egui::{LayerId, Ui, UiBuilder};

pub fn draw_sidebar(mut contexts: EguiContexts, command_state: Res<CommandState>) -> Result {
    let ctx = contexts.ctx_mut()?;
    let mut viewport_ui = Ui::new(
        ctx.clone(),
        "viewport".into(),
        UiBuilder::new().layer_id(LayerId::background()).max_rect(ctx.viewport_rect()),
    );

    if command_state.preview == PreviewCommand::SidebarState(UiState::TowersList { filter: None }) {
        egui::Panel::right("right_panel_towers_list")
            .resizable(true)
            .show(&mut viewport_ui, |ui| {
                ui.heading("Towers");
                ui.separator();
                ui.label("tower");
            })
            .response
            .rect
            .width();
    }

    if command_state.preview == PreviewCommand::SidebarState(UiState::EnemiesList { filter: None })
    {
        egui::Panel::right("right_panel_enemies_list")
            .resizable(true)
            .show(&mut viewport_ui, |ui| {
                ui.heading("Enemies");
                ui.separator();
                ui.label("enemie");
            })
            .response
            .rect
            .width();
    }

    Ok(())
}
