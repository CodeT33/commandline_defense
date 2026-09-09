use crate::cli::command_line_state_management::PreviewCommand;
use crate::ecs_elements::resources::CommandState;
use crate::ui_overlay::ui_state::UiState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui::{LayerId, Ui, UiBuilder};

pub fn draw_sidebar(mut contexts: EguiContexts, command_state: Res<CommandState>) -> Result {
    let ctx = contexts.ctx_mut()?;
    let mut viewport_ui = Ui::new(
        ctx.clone(),
        "viewport".into(),
        UiBuilder::new().layer_id(LayerId::background()).max_rect(ctx.viewport_rect()),
    );

    if matches!(command_state.preview, PreviewCommand::SidebarState(UiState::TowersList { .. })) {
        egui::Panel::right("right_panel_towers_list")
            .resizable(false)
            .default_size(300.0)
            .show(&mut viewport_ui, |ui| {
                ui.heading("Towers");
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for _i in 0..3 {
                        tower_entry(ui);
                    }
                });
            });
    }

    if matches!(command_state.preview, PreviewCommand::SidebarState(UiState::EnemiesList { .. }))
    {
        egui::Panel::right("right_panel_enemies_list")
            .resizable(false)
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

fn tower_entry(ui: &mut Ui) {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("assault-bober").strong());

                ui.label("300");
            });
        });
    });

    ui.add_space(5.0);
}