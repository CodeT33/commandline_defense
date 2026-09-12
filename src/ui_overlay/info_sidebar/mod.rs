pub mod info_commands;
pub mod info_enemies;
pub mod info_menus;
pub mod info_towers;

use crate::cli::preview::PreviewCommand;
use crate::ecs_elements::resources::{CommandState, TexturePackSettings, UiHover};
use crate::tiers::ValueTiers;
use crate::ui_overlay::info_sidebar::info_enemies::{
    draw_enemy_description, draw_enemy_info, draw_enemy_list,
};
use crate::ui_overlay::info_sidebar::info_menus::draw_menus_list;
use crate::ui_overlay::info_sidebar::info_towers::{
    draw_tower_description, draw_tower_info, draw_tower_list, draw_tower_upgrades,
};
use crate::ui_overlay::ui_state::{EnemyPage, TowerPage, UiState};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use egui::{LayerId, Pos2, Ui, UiBuilder};

pub fn draw_gui(
    mut contexts: EguiContexts, command_state: Res<CommandState>, asset_server: Res<AssetServer>,
    texture_pack_settings: Res<TexturePackSettings>, mut ui_hover: ResMut<UiHover>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let mut viewport_ui = Ui::new(
        ctx.clone(),
        "viewport".into(),
        UiBuilder::new().layer_id(LayerId::background()).max_rect(ctx.viewport_rect()),
    );

    let ui_state = match &command_state.preview {
        PreviewCommand::SidebarState(sidebar_state) => sidebar_state,
        _ => {
            let Some(sidebar_state) = command_state.persistent_preview.as_ref() else {
                ui_hover.0 = false;
                return Ok(());
            };
            sidebar_state
        },
    };

    match ui_state {
        UiState::Menus => {
            egui::Panel::right("Right Panel Menus").resizable(true).default_size(200.0).show(
                &mut viewport_ui,
                |ui| {
                    draw_menus_list(ui);
                },
            );
        },
        UiState::TowersList { selected, further_details } => {
            egui::Panel::right("Right Panel Tower List").resizable(true).default_size(200.0).show(
                &mut viewport_ui,
                |ui| {
                    draw_tower_list(ui, &mut contexts, &asset_server, &texture_pack_settings);
                },
            );

            if let Some(tower_type) = selected {
                egui::Window::new("Details")
                    .resizable(false)
                    .default_pos(Pos2 { x: 1120.0, y: 100.0 })
                    .default_size(egui::Vec2 { x: 200.0, y: 100.0 })
                    .show(&viewport_ui, |ui| {
                        draw_tower_info(
                            ui,
                            &mut contexts,
                            &asset_server,
                            &texture_pack_settings,
                            *tower_type,
                        );
                    });

                match further_details {
                    Some(TowerPage::Description) => {
                        egui::Window::new("Description")
                            .resizable(false)
                            .default_pos(Pos2 { x: 900.0, y: 100.0 })
                            .default_size(egui::Vec2 { x: 200.0, y: 100.0 })
                            .show(&viewport_ui, |ui| {
                                draw_tower_description(ui, *tower_type);
                            });
                    },
                    Some(TowerPage::Upgrades) => {
                        egui::Window::new("Upgrades")
                            .resizable(false)
                            .default_pos(Pos2 { x: 900.0, y: 100.0 })
                            .default_size(egui::Vec2 { x: 200.0, y: 100.0 })
                            .show(&viewport_ui, |ui| {
                                draw_tower_upgrades(
                                    ui,
                                    &mut contexts,
                                    &asset_server,
                                    &texture_pack_settings,
                                    *tower_type,
                                );
                            });
                    },
                    None => {},
                }
            }
        },
        UiState::EnemiesList { selected, further_details } => {
            egui::Panel::right("Right Panel Enemies List")
                .resizable(true)
                .default_size(200.0)
                .show(&mut viewport_ui, |ui| {
                    draw_enemy_list(ui, &mut contexts, &asset_server, &texture_pack_settings);
                });

            if let Some(enemy_type) = selected {
                egui::Window::new("Details")
                    .resizable(false)
                    .default_pos(Pos2 { x: 1120.0, y: 100.0 })
                    .default_size(egui::Vec2 { x: 200.0, y: 100.0 })
                    .show(&viewport_ui, |ui| {
                        draw_enemy_info(
                            ui,
                            &mut contexts,
                            &asset_server,
                            &texture_pack_settings,
                            *enemy_type,
                        );
                    });

                match further_details {
                    Some(EnemyPage::Description) => {
                        egui::Window::new("Description")
                            .resizable(false)
                            .default_pos(Pos2 { x: 900.0, y: 100.0 })
                            .default_size(egui::Vec2 { x: 200.0, y: 100.0 })
                            .show(&viewport_ui, |ui| {
                                draw_enemy_description(ui, *enemy_type);
                            });
                    },
                    None => {},
                }
            }
        },
        _ => {},
    }

    let ctx = contexts.ctx_mut()?;
    let pointer = ctx.input(|input| input.pointer.interact_pos());
    let egui_hover = ctx.is_pointer_over_egui() || ctx.egui_wants_pointer_input();

    ui_hover.0 = egui_hover
        || pointer.is_some_and(|p| !viewport_ui.available_rect_before_wrap().contains(p));
    Ok(())
}

fn draw_navigation_box(ui: &mut Ui, text: String) -> egui::response::InnerResponse<()> {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label(egui::RichText::new(format!("/{}", text)).strong());
    })
}

pub(crate) fn colored_attribute(ui: &mut Ui, attribute_name: &str, attribute_value: ValueTiers) {
    ui.horizontal(|ui| {
        ui.label(format!("{}: ", attribute_name));

        ui.label(
            egui::RichText::new(format!("{:?}", attribute_value))
                .color(attribute_value.get_tier_color())
                .strong(),
        );
    });
}

pub(crate) fn tier_list(ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("Tier range:");
        ui.label(egui::RichText::new("S").color(ValueTiers::S.get_tier_color()).strong());
        ui.label(egui::RichText::new("A").color(ValueTiers::A.get_tier_color()).strong());
        ui.label(egui::RichText::new("B").color(ValueTiers::B.get_tier_color()).strong());
        ui.label(egui::RichText::new("C").color(ValueTiers::C.get_tier_color()).strong());
        ui.label(egui::RichText::new("D").color(ValueTiers::D.get_tier_color()).strong());
        ui.label(egui::RichText::new("E").color(ValueTiers::E.get_tier_color()).strong());
        ui.label(egui::RichText::new("F").color(ValueTiers::F.get_tier_color()).strong());
    });
}
