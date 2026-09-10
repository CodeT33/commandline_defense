use crate::ecs_elements::resources::TexturePackSettings;
use crate::entities::tower::TowerType;
use crate::ui_overlay::info_sidebar::draw_navigation_box;
use bevy::asset::AssetServer;
use bevy::prelude::Res;
use bevy_egui::{EguiContexts, EguiTextureHandle};
use clap::ValueEnum;
use egui::Ui;

pub(crate) fn draw_tower_info(
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

pub(crate) fn draw_tower_list(
    ui: &mut Ui, ctx: &mut EguiContexts, asset_server: &Res<AssetServer>,
    texture_pack_settings: &Res<TexturePackSettings>,
) {
    ui.heading("Towers");
    ui.separator();

    egui::ScrollArea::vertical().show(ui, |ui| {
        for tower in TowerType::value_variants() {
            tower_entry(ui, ctx, asset_server, texture_pack_settings, *tower)
                .expect("Could not load tower into sidebar");
        }
    });
}

fn tower_entry(
    ui: &mut Ui, contexts: &mut EguiContexts, asset_server: &AssetServer,
    texture_pack_settings: &TexturePackSettings, tower_type: TowerType,
) -> bevy::prelude::Result {
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


pub(crate) fn draw_tower_description(ui: &mut Ui, tower_type: TowerType) {
    let tower_name = tower_type
        .to_possible_value()
        .map(|pv| pv.get_name().to_owned())
        .expect("value skipped by Clap");

    ui.heading(tower_name);
    ui.separator();
    ui.label("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet");
}

pub(crate) fn draw_tower_upgrades(ui: &mut Ui, tower_type: TowerType) {
    let tower_name = tower_type
        .to_possible_value()
        .map(|pv| pv.get_name().to_owned())
        .expect("value skipped by Clap");

    ui.heading(tower_name);
    ui.separator();
    ui.label("Upgrade 1...");
}

