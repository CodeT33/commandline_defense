use crate::cli::command_line_state_management::PreviewCommand;
use crate::ecs_elements::resources::{CommandState, TexturePackSettings};
use crate::entities::tower::TowerType;
use crate::ui_overlay::ui_state::UiState;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiTextureHandle, egui};
use egui::{LayerId, Ui, UiBuilder};
use strum::IntoEnumIterator;

pub fn draw_sidebar(
    mut contexts: EguiContexts, command_state: Res<CommandState>, asset_server: Res<AssetServer>,
    texture_pack_settings: Res<TexturePackSettings>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let mut viewport_ui = Ui::new(
        ctx.clone(),
        "viewport".into(),
        UiBuilder::new().layer_id(LayerId::background()).max_rect(ctx.viewport_rect()),
    );

    if matches!(command_state.preview, PreviewCommand::SidebarState(UiState::TowersList { .. })) {
        egui::Panel::right("right_panel_towers_list").resizable(false).show(
            &mut viewport_ui,
            |ui| {
                ui.heading("Towers");
                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for tower in TowerType::iter() {
                        tower_entry(
                            ui,
                            &mut contexts,
                            &asset_server,
                            &texture_pack_settings,
                            tower,
                        )
                        .expect("Could not load tower into sidebar");
                    }
                });
            },
        );
    }

    if matches!(command_state.preview, PreviewCommand::SidebarState(UiState::TowerInfo(..))) {
        egui::Panel::right("right_panel_tower_info").resizable(false).show(
            &mut viewport_ui,
            |ui| {
                ui.heading("assault-bober");
                ui.separator();
            },
        );
    }

    if matches!(command_state.preview, PreviewCommand::SidebarState(UiState::EnemiesList { .. })) {
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

    if matches!(command_state.preview, PreviewCommand::SidebarState(UiState::EnemyInfo(..))) {
        egui::Panel::right("right_panel_enemies_info").resizable(false).show(
            &mut viewport_ui,
            |ui| {
                ui.heading("zapano");
                ui.separator();
            },
        );
    }

    Ok(())
}

fn tower_entry(
    ui: &mut Ui, contexts: &mut EguiContexts, asset_server: &AssetServer,
    texture_pack_settings: &TexturePackSettings, tower_type: TowerType,
) -> Result {
    let attributes = tower_type.get_attributes();

    let image_handle =
        asset_server.load(texture_pack_settings.get_asset_path(attributes.preview_sprite));

    let texture_id = contexts.add_image(EguiTextureHandle::Strong(image_handle));

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(egui::RichText::new("assault-aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").strong());

                ui.label(egui::RichText::new(format!("${}", attributes.price)).size(16.0));
            });

            ui.add(egui::Image::new(egui::load::SizedTexture::new(texture_id, [64.0, 64.0])));
        });
    });

    ui.add_space(4.0);
    Ok(())
}
