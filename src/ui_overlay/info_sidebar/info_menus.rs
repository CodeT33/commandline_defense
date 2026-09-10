use egui::Ui;
use crate::ui_overlay::info_sidebar::draw_navigation_box;

pub(crate) fn draw_menus_list(ui: &mut Ui) {
    ui.heading("Menus");
    ui.separator();
    draw_navigation_box(ui, "towers".parse().unwrap());
    ui.add_space(4.0);
    draw_navigation_box(ui, "enemies".parse().unwrap());
    ui.add_space(4.0);
    draw_navigation_box(ui, "commands".parse().unwrap());
}