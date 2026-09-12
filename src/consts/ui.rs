use bevy::prelude::{BorderRadius, Color, Val};

pub(crate) const CONSOLE_ERROR_COLOR: Color = Color::linear_rgb(1.0, 0.0, 0.0);
pub(crate) const BOUNDING_BOX_DEBUG_COLOR: Color = Color::hsv(120.0, 1.0, 1.0);
pub(crate) const BOUNDING_BOX_DEBUG_COLOR_ALT: Color = Color::hsv(0.0, 1.0, 1.0);
/// How often the debug bounding box color toggles between the two colors.
pub(crate) const BOUNDING_BOX_DEBUG_COLOR_TOGGLE_SECS: f32 = 0.25;

pub(crate) const BEVY_UI_BACKGROUND_COLOR: Color = Color::srgba(0.1, 0.1, 0.12, 0.9);
pub(crate) const BEVY_UI_BORDER_RADIUS: BorderRadius =
    BorderRadius::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0));

pub(crate) mod health_bars {
    use bevy::prelude::Color;

    pub(crate) const HEALTH_BAR_BACKGROUND_COLOR: Color = Color::srgb(0.12, 0.12, 0.12);
    pub(crate) const HEALTH_BAR_FILL_COLOR: Color = Color::srgb(0.2, 0.8, 0.3);
    pub(crate) const HEALTH_BAR_WIDTH_TILES: f32 = 1.0;
    pub(crate) const HEALTH_BAR_HEIGHT_TILES: f32 = 0.07;
    pub(crate) const HEALTH_BAR_OFFSET_TILES: f32 = 0.7;
}

pub(crate) mod grid {
    use crate::ui_overlay::grid::{FontSettings, GridTileColors};
    use bevy::prelude::{Color, FontWeight};

    pub(crate) const GRID_POSITION: FontSettings = FontSettings {
        font_size: 12.0,
        font_weight: FontWeight(160),
        color: Color::srgba(0.5, 1.0, 0.5, 0.5),
    };

    pub(crate) const GRID_META_POSITION: FontSettings = FontSettings {
        font_size: 18.0,
        font_weight: FontWeight(240),
        color: Color::srgba(1.0, 1.0, 1.0, 1.0),
    };

    pub(crate) const GRID_POSITION_TILE_COLORS: GridTileColors = GridTileColors {
        none: Color::srgba(0.0, 0.0, 0.0, 0.0),
        path_start: Color::srgba(1.0, 0.5, 1.0, 0.9),
        path: Color::srgba(1.0, 1.0, 0.5, 0.9),
        restricted: Color::srgba(1.0, 0.5, 0.5, 0.9),
        placeable: Color::srgba(0.5, 1.0, 0.5, 0.9),
        water: Color::srgba(0.5, 0.5, 1.0, 0.9),
    };

    pub(crate) const GRID_LINE_THICKNESS: f32 = 0.025;
    pub(crate) const GRID_LINE_COLOR: Color = Color::srgba(0.5, 1.0, 0.5, 0.2);
    pub(crate) const GRID_CONTRAST_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.5);
}
