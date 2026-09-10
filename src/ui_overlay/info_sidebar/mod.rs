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

    if let PreviewCommand::SidebarState(UiState::TowersList {selected}) = command_state.preview {
        egui::Panel::right("right_panel_towers_list").resizable(true).default_size(160.0).show(
            &mut viewport_ui,
            |ui| {
                draw_towers_list(ui, &mut contexts, &asset_server, &texture_pack_settings);
            },
        );

        if let Some(tower_type) = selected {
            egui::Panel::right("right_panel_tower_info").resizable(true).default_size(160.0).show(
                &mut viewport_ui,
                |ui| {
                    draw_tower_info(
                        ui,
                        &mut contexts,
                        &asset_server,
                        &texture_pack_settings,
                        tower_type,
                    );
                },
            );
        }
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

    if matches!(command_state.preview, PreviewCommand::SidebarState(UiState::EnemiesList {..})) {
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

fn draw_tower_info(
    ui: &mut Ui, ctx: &mut EguiContexts, asset_server: &Res<AssetServer>,
    texture_pack_settings: &Res<TexturePackSettings>, tower_type: TowerType,
) {
    let tower_attributes = tower_type.get_attributes();
    let bullet_attributes = tower_attributes.bullet_type.get_attributes();

    let image_handle =
        asset_server.load(texture_pack_settings.get_asset_path(tower_attributes.preview_sprite));
    let texture_id = ctx.add_image(EguiTextureHandle::Strong(image_handle));

    ui.heading("assault-bober");
    ui.separator();
    ui.add(egui::Image::new(egui::load::SizedTexture::new(texture_id, [128.0, 128.0])));
    ui.label(egui::RichText::new(format!("${}", tower_attributes.price)).size(32.0));
    ui.separator();

    ui.label(egui::RichText::new("Tower Attributes").strong());
    ui.label(format!("Cooldown: {}ms", tower_attributes.cooldown_ms));
    ui.label(format!("Range: {}m", tower_attributes.range));
    ui.label(format!(
        "Tower size: {}x{}m",
        tower_attributes.size_tiles.x, tower_attributes.size_tiles.y
    ));
    ui.label(format!("Targeting type: {:?}", tower_attributes.targeting_type));

    ui.separator();

    ui.label(egui::RichText::new("Bullet Attributes").strong());
    ui.label(format!("Damage: {}", bullet_attributes.damage));
    ui.label(format!("Piercing: {}", bullet_attributes.health));
    ui.label(format!("Speed: {}tps", tower_attributes.bullet_speed_tps));
    ui.label(format!("Relative collider size: {}m", bullet_attributes.relative_collider_size));
}

fn draw_towers_list(
    ui: &mut Ui, ctx: &mut EguiContexts, asset_server: &Res<AssetServer>,
    texture_pack_settings: &Res<TexturePackSettings>,
) {
    ui.heading("Towers");
    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        for tower in TowerType::iter() {
            tower_entry(ui, ctx, asset_server, texture_pack_settings, tower)
                .expect("Could not load tower into sidebar");
        }
    });
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
                ui.label(egui::RichText::new("assault-bober").strong());

                ui.label(egui::RichText::new(format!("${}", attributes.price)).size(16.0));
            });

            ui.add(egui::Image::new(egui::load::SizedTexture::new(texture_id, [64.0, 64.0])));
        });
    });

    ui.add_space(4.0);
    Ok(())
}
