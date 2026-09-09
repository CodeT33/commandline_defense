use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use egui::{LayerId, Ui, UiBuilder};

pub fn spawn_sidebar(mut contexts: EguiContexts) -> Result {
    let ctx = contexts.ctx_mut()?;
    let mut viewport_ui = Ui::new(
        ctx.clone(),
        "viewport".into(),
        UiBuilder::new().layer_id(LayerId::background()).max_rect(ctx.viewport_rect()),
    );
    let mut right_sidepanel = egui::Panel::right("right_panel")
        .resizable(true)
        .show(&mut viewport_ui, |ui| {
            ui.label("sidebar lol");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    Ok(())
}
