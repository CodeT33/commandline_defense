use bevy::math::Vec2;
use macros::generate_dir_structure_as_modules;

pub const WINDOW_TITLE: &str = "2d game";
pub const WINDOW_RESOLUTION: [u32; 2] = [800, 450];
pub const PHYSICS_FRAME_RATE: u16 = 144;
pub const MAP_SIZE_TILES: [u16; 2] = [32, 16];
pub const TILE_SIZE: u16 = 16;

pub const PROJECTILE_SIZE_TILES: Vec2 = Vec2::splat(0.3);
pub const ENEMY_SIZE_TILES: Vec2 = Vec2::splat(1.0);
pub const TOWER_SIZE_TILES: Vec2 = Vec2::splat(1.0);
pub const TOWER_RANGE_TILES: u16 = 5;

pub const PROJECTILE_RADIUS: f32 = PROJECTILE_SIZE_TILES.x / 2.0;
pub const ENEMY_RADIUS: f32 = ENEMY_SIZE_TILES.x / 2.0;

pub const ENEMY_PATH_DURATION_MS: u64 = 12000;

pub const BULLET_ROTATION_DURATION_MS: u64 = 234;

// ui
pub mod ui {
    pub mod grid {
        use bevy::color::Color;
        use bevy::prelude::FontWeight;

        pub const GRID_LINE_THICKNESS: f32 = 0.025;
        pub const GRID_LINE_COLOR: Color = Color::srgba(0.5, 1.0, 0.5, 0.3);
        pub const GRID_POSITION_FONT_SIZE: f32 = 12.0;
        pub const GRID_POSITION_FONT_WEIGHT: FontWeight = FontWeight(160);
        pub const GRID_POSITION_COLOR: Color = Color::srgba(0.5, 1.0, 0.5, 0.5);
        pub const GRID_META_POSITION_FONT_SIZE: f32 = 18.0;
        pub const GRID_META_POSITION_FONT_WEIGHT: FontWeight = FontWeight(240);
        pub const GRID_META_POSITION_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 1.0);
        pub const TILE_HIGHLIGHT_COLOR: Color = Color::srgba(0.5, 1.0, 0.5, 0.3);
    }
}

pub mod rendering_layers {
    pub const MAP: f32 = 0.0;
    pub const ENTITY: f32 = 5.0;
    pub const GRID: f32 = 10.0;
    pub const GRID_LABEL: f32 = 11.0;
    pub const HIGHLIGHT: f32 = 20.0;
}

generate_dir_structure_as_modules!(assets);
