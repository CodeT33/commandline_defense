use crate::ecs_elements::resources::TexturePackSettings;
use crate::entities::enemies::EnemyType;
use crate::tiers::{Formatting, ValueTiers};
use crate::ui_overlay::info_sidebar::draw_navigation_box;
use bevy::asset::AssetServer;
use bevy::prelude::Res;
use bevy_egui::{EguiContexts, EguiTextureHandle};
use clap::ValueEnum;
use egui::Ui;

pub(crate) fn draw_enemy_info(
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
    ui.label(format!("Health: {:?}", enemy_attributes.health));
    ui.label(format!("Speed: {:?}", enemy_attributes.speed_tps));
    ui.label(format!(
        "Enemy size: {}x{}m",
        enemy_attributes.texture_size_tiles, enemy_attributes.texture_size_tiles
    ));
    ui.label(format!("Relative collider size: {}m", enemy_attributes.relative_collider_size));
    ui.separator();
    ui.add_space(4.0);
    draw_navigation_box(ui, "description".parse().unwrap());
}

pub(crate) fn draw_enemy_list(
    ui: &mut Ui, ctx: &mut EguiContexts, asset_server: &Res<AssetServer>,
    texture_pack_settings: &Res<TexturePackSettings>,
) {
    ui.heading("Enemies");
    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        for enemy in EnemyType::value_variants() {
            enemy_entry(ui, ctx, asset_server, texture_pack_settings, *enemy)
                .expect("Could not load tower into sidebar");
        }
    });
}

fn enemy_entry(
    ui: &mut Ui, contexts: &mut EguiContexts, asset_server: &AssetServer,
    texture_pack_settings: &TexturePackSettings, enemy_type: EnemyType,
) -> bevy::prelude::Result {
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

            ui.add(egui::Image::new(egui::load::SizedTexture::new(texture_id, [64.0, 64.0])))
                .on_hover_ui(|ui| {
                    let formatting = Formatting::Roman;
                    let format = |ui: &mut Ui, value: ValueTiers, name: &str| {
                        ui.label(format!("{}: {}", value.format_in(formatting), name))
                    };
                    ui.label(format!("${}", attributes.reward));
                    ui.separator();
                    format(ui, attributes.health, "Health");
                    format(ui, attributes.speed_tps, "Speed");
                    format(ui, attributes.player_health_penalty, "Player Damage");
                });
        });
    });

    ui.add_space(4.0);
    Ok(())
}

pub(crate) fn draw_enemy_description(ui: &mut Ui, enemy_type: EnemyType) {
    let enemy_name = enemy_type
        .to_possible_value()
        .map(|pv| pv.get_name().to_owned())
        .expect("value skipped by Clap");

    ui.heading(enemy_name);
    ui.separator();
    ui.label(enemy_type.get_description());
}
