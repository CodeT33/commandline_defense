use crate::cli::command_line_state_management::PreviewCommand;
use crate::ecs_elements::resources::{CommandState, TexturePackSettings};
use crate::entities::enemies::EnemyType;
use crate::entities::tower::TowerType;
use crate::ui_overlay::ui_state::UiState;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiTextureHandle, egui};
use clap::ValueEnum;
use egui::{LayerId, Pos2, Ui, UiBuilder};
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

    if let PreviewCommand::SidebarState(UiState::Menus) = command_state.preview {
        egui::Panel::right("Right Panel Menus").resizable(true).default_size(160.0).show(
            &mut viewport_ui,
            |ui| {
                draw_menus_list(ui);
            },
        );
    }

    if let PreviewCommand::SidebarState(UiState::TowersList { selected }) = command_state.preview {
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
    }

    if let PreviewCommand::SidebarState(UiState::EnemiesList { selected }) = command_state.preview {
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
    }
    Ok(())
}

fn draw_menus_list(ui: &mut Ui) {
    ui.heading("Menus");
    ui.separator();
    draw_navigation_box(ui, "towers".parse().unwrap());
    ui.add_space(4.0);
    draw_navigation_box(ui, "enemies".parse().unwrap());
    ui.add_space(4.0);
    draw_navigation_box(ui, "commands".parse().unwrap());
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

    let tower_name = tower_type
        .to_possible_value()
        .map(|pv| pv.get_name().to_owned())
        .expect("value skipped by Clap");

    ui.heading(tower_name);
    ui.separator();
    ui.add(egui::Image::new(egui::load::SizedTexture::new(texture_id, [128.0, 128.0])));
    ui.separator();
    ui.label(egui::RichText::new(format!("Price: ${}", tower_attributes.price)).size(16.0));
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
    ui.separator();
    ui.add_space(4.0);
    draw_navigation_box(ui, "upgrades".parse().unwrap());
    ui.add_space(4.0);
    draw_navigation_box(ui, "description".parse().unwrap());
}

fn draw_tower_list(
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
    let tower_name = tower_type
        .to_possible_value()
        .map(|pv| pv.get_name().to_owned())
        .expect("value skipped by Clap");

    let attributes = tower_type.get_attributes();

    let image_handle =
        asset_server.load(texture_pack_settings.get_asset_path(attributes.preview_sprite));

    let texture_id = contexts.add_image(EguiTextureHandle::Strong(image_handle));

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                draw_navigation_box(ui, tower_name);
                ui.label(egui::RichText::new(format!("${}", attributes.price)).size(16.0));
            });

            ui.add(egui::Image::new(egui::load::SizedTexture::new(texture_id, [64.0, 64.0])));
        });
    });

    ui.add_space(4.0);
    Ok(())
}

fn draw_enemy_info(
    ui: &mut Ui, ctx: &mut EguiContexts, asset_server: &Res<AssetServer>,
    texture_pack_settings: &Res<TexturePackSettings>, enemy_type: EnemyType,
) {
    let enemy_attributes = enemy_type.get_attributes();

    let image_handle =
        asset_server.load(texture_pack_settings.get_asset_path(enemy_attributes.asset));
    let texture_id = ctx.add_image(EguiTextureHandle::Strong(image_handle));

    let enemy_name = enemy_type
        .to_possible_value()
        .map(|pv| pv.get_name().to_owned())
        .expect("value skipped by Clap");

    ui.heading(enemy_name);
    ui.separator();
    ui.add(egui::Image::new(egui::load::SizedTexture::new(texture_id, [128.0, 128.0])));
    ui.separator();
    ui.label(egui::RichText::new(format!("Reward: ${}", enemy_attributes.reward)).size(16.0));
    ui.separator();

    ui.label(egui::RichText::new("Enemy Attributes").strong());
    ui.label(format!("Health: {}", enemy_attributes.health));
    ui.label(format!("Speed: {}tps", enemy_attributes.speed_tps));
    ui.label(format!(
        "Enemy size: {}x{}m",
        enemy_attributes.texture_size_tiles, enemy_attributes.texture_size_tiles
    ));
    ui.label(format!("Relative collider size: {}m", enemy_attributes.relative_collider_size));
    ui.separator();
    ui.add_space(4.0);
    draw_navigation_box(ui, "description".parse().unwrap());
}

fn draw_enemy_list(
    ui: &mut Ui, ctx: &mut EguiContexts, asset_server: &Res<AssetServer>,
    texture_pack_settings: &Res<TexturePackSettings>,
) {
    ui.heading("Enemies");
    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        for &enemy in EnemyType::value_variants() {
            enemy_entry(ui, ctx, asset_server, texture_pack_settings, enemy)
                .expect("Could not load tower into sidebar");
        }
    });
}

fn enemy_entry(
    ui: &mut Ui, contexts: &mut EguiContexts, asset_server: &AssetServer,
    texture_pack_settings: &TexturePackSettings, enemy_type: EnemyType,
) -> Result {
    let enemy_name = enemy_type
        .to_possible_value()
        .map(|pv| pv.get_name().to_owned())
        .expect("value skipped by Clap");

    let attributes = enemy_type.get_attributes();

    let image_handle = asset_server.load(texture_pack_settings.get_asset_path(attributes.asset));

    let texture_id = contexts.add_image(EguiTextureHandle::Strong(image_handle));

    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                draw_navigation_box(ui, enemy_name);
                ui.label(egui::RichText::new(format!("${}", attributes.reward)).size(16.0));
            });

            ui.add(egui::Image::new(egui::load::SizedTexture::new(texture_id, [64.0, 64.0])));
        });
    });

    ui.add_space(4.0);
    Ok(())
}

fn draw_navigation_box(ui: &mut Ui, text: String) -> egui::response::InnerResponse<()> {
    egui::Frame::group(ui.style()).show(ui, |ui| {
        ui.label(egui::RichText::new(format!("/{}", text)).strong());
    })
}
