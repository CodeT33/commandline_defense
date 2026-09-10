pub mod info_commands;
pub mod info_enemies;
pub mod info_menus;
pub mod info_towers;

use crate::cli::command_line_state_management::PreviewCommand;
use crate::ecs_elements::resources::{CommandState, TexturePackSettings};
use crate::ui_overlay::info_sidebar::info_enemies::{
    draw_enemy_description, draw_enemy_info, draw_enemy_list,
};
use crate::ui_overlay::info_sidebar::info_menus::draw_menus_list;
use crate::ui_overlay::info_sidebar::info_towers::{
    draw_tower_description, draw_tower_info, draw_tower_list, draw_tower_upgrades,
};
use crate::ui_overlay::ui_state::{EnemyPage, TowerPage, UiState};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui::{LayerId, Pos2, Ui, UiBuilder};

pub fn draw_gui(
    mut contexts: EguiContexts, command_state: Res<CommandState>, asset_server: Res<AssetServer>,
    texture_pack_settings: Res<TexturePackSettings>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let mut viewport_ui = Ui::new(
        ctx.clone(),
        "viewport".into(),
        UiBuilder::new().layer_id(LayerId::background()).max_rect(ctx.viewport_rect()),
    );

    if let PreviewCommand::SidebarState(UiState::Menus) = command_state.preview {
        egui::Panel::right("Right Panel Menus").resizable(true).default_size(160.0).show(
            &mut viewport_ui,
            |ui| {
                draw_menus_list(ui);
            },
        );
    }

    if let PreviewCommand::SidebarState(UiState::TowersList { selected, further_details }) =
        command_state.preview
    {
        egui::Panel::right("Right Panel Tower List").resizable(true).default_size(160.0).show(
            &mut viewport_ui,
            |ui| {
                draw_tower_list(ui, &mut contexts, &asset_server, &texture_pack_settings);
            },
        );

        if let Some(tower_type) = selected {
            egui::Window::new("Tower Info")
                .resizable(true)
                .default_pos(Pos2 { x: 1120.0, y: 100.0 })
                .default_size(egui::Vec2 { x: 200.0, y: 100.0 })
                .show(&viewport_ui, |ui| {
                    draw_tower_info(
                        ui,
                        &mut contexts,
                        &asset_server,
                        &texture_pack_settings,
                        tower_type,
                    );
                });
        }

        if let Some(tower_type) = selected
            && further_details == TowerPage::Description
        {
            egui::Window::new("Description")
                .resizable(true)
                .default_pos(Pos2 { x: 900.0, y: 100.0 })
                .default_size(egui::Vec2 { x: 200.0, y: 100.0 })
                .show(&viewport_ui, |ui| {
                    draw_tower_description(ui, tower_type);
                });
        }

        if let Some(tower_type) = selected
            && further_details == TowerPage::Upgrades
        {
            egui::Window::new("Upgrades")
                .resizable(true)
                .default_pos(Pos2 { x: 900.0, y: 100.0 })
                .default_size(egui::Vec2 { x: 200.0, y: 100.0 })
                .show(&viewport_ui, |ui| {
                    draw_tower_upgrades(ui, tower_type);
                });
        }
    }

    if let PreviewCommand::SidebarState(UiState::EnemiesList { selected, further_details }) =
        command_state.preview
    {
        egui::Panel::right("Right Panel Enemies List").resizable(true).default_size(160.0).show(
            &mut viewport_ui,
            |ui| {
                draw_enemy_list(ui, &mut contexts, &asset_server, &texture_pack_settings);
            },
        );

        if let Some(enemy_type) = selected {
            egui::Window::new("Enemy Info")
                .resizable(true)
                .default_pos(Pos2 { x: 1120.0, y: 100.0 })
                .default_size(egui::Vec2 { x: 200.0, y: 100.0 })
                .show(&viewport_ui, |ui| {
                    draw_enemy_info(
                        ui,
                        &mut contexts,
                        &asset_server,
                        &texture_pack_settings,
                        enemy_type,
                    );
                });
        }

        if let Some(enemy_type) = selected
            && further_details == EnemyPage::Description
        {
            egui::Window::new("Enemy Description")
                .resizable(true)
                .default_pos(Pos2 { x: 900.0, y: 100.0 })
                .default_size(egui::Vec2 { x: 200.0, y: 100.0 })
                .show(&viewport_ui, |ui| {
                    draw_enemy_description(ui, enemy_type);
                });
        }
    }
    Ok(())
}

fn draw_navigation_box(ui: &mut Ui, text: String) -> egui::response::InnerResponse<()> {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label(egui::RichText::new(format!("/{}", text)).strong());
    })
}
