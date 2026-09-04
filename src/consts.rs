use bevy::math::U16Vec2;
use bevy::prelude::Vec2;

pub const WINDOW_TITLE: &str = "2d game";
pub const WINDOW_RESOLUTION: [u32; 2] = [800, 450];

pub const PHYSICS_FRAME_RATE: u16 = 144;

pub const MAP_SIZE_TILES: U16Vec2 = U16Vec2 { x: 32, y: 16 };
pub const TILE_SIZE: u16 = 16;

pub const ENEMY_COUNT: usize = 40;
pub const ENEMY_SIZE_TILES: Vec2 = Vec2::splat(1.0);
pub const ENEMY_RADIUS: f32 = ENEMY_SIZE_TILES.x / 2.0;
pub const ENEMY_SPEED_TILES_PER_SECOND: f32 = 3.0;
pub const ENEMY_SPAWN_INTERVAL_MS: u64 = 2000;

pub const TOWER_SIZE_TILES: Vec2 = Vec2::splat(1.0);
pub const TOWER_RANGE_TILES: u16 = 5;
pub const TOWER_COOLDOWN_MS: u32 = 1000;

pub const PROJECTILE_SIZE_TILES: Vec2 = Vec2::splat(1.0);
pub const PROJECTILE_RADIUS: f32 = PROJECTILE_SIZE_TILES.x / 8.0;
pub const PROJECTILE_SPEED_TILES_PER_SECOND: f32 = 10.0;

pub const BULLET_ROTATION_DURATION_MS: u64 = 234;

pub mod viewports {
    use crate::camera::Viewport;

    pub const BASIC_CAMERA: Viewport = Viewport { min_zoom: 0.01, max_zoom: 0.5, zoom_speed: 0.1 };
}

pub mod map_logic_parsing {
    use crate::ui_overlay::grid::LogicGridTileColors;

    pub const LOGIC_GRID_TILE_COLORS: LogicGridTileColors = LogicGridTileColors {
        path_start: 0xff00ff,
        path: 0xffff00,
        restricted: 0xff0000,
        placeable: 0x00ff00,
        water: 0x0000ff,
    };
}

pub mod towers {
    use crate::texture_packs::TexturePackAssets;
    use crate::tower::TowerAttributes;
    use bevy::math::{U16Vec2, Vec2};

    pub const ASSAULT_TROOP_ATTRIBUTES: TowerAttributes = TowerAttributes {
        price: 100,
        size_tiles: Vec2::splat(1.0),
        range_tiles: U16Vec2::splat(3),
        cooldown_ms: 1000,
        bullet_speed: 10.0,
        sprites: [
            TexturePackAssets::Troops_Assault_AssaultTroopLvl1,
            TexturePackAssets::Troops_Assault_AssaultTroopLvl2,
            TexturePackAssets::Troops_Assault_AssaultTroopLvl3,
            TexturePackAssets::Troops_Assault_AssaultTroopLvl3,
        ],
        tower_rotates: true,
    };

    pub const BOOM_TROOP_ATTRIBUTES: TowerAttributes = TowerAttributes {
        price: 320,
        size_tiles: Vec2::splat(1.0),
        range_tiles: U16Vec2::splat(2),
        cooldown_ms: 3000,
        bullet_speed: 6.0,
        sprites: [
            TexturePackAssets::Troops_Boom_BoomTroopLvl1,
            TexturePackAssets::Troops_Boom_BoomTroopLvl2,
            TexturePackAssets::Troops_Boom_BoomTroopLvl3,
            TexturePackAssets::Troops_Boom_BoomTroopLvl3,
        ],
        tower_rotates: true,
    };

    pub const GATLING_TROOP_ATTRIBUTES: TowerAttributes = TowerAttributes {
        price: 210,
        size_tiles: Vec2::splat(1.0),
        range_tiles: U16Vec2::splat(4),
        cooldown_ms: 300,
        bullet_speed: 8.0,
        sprites: [
            TexturePackAssets::Troops_Gatling_GatlingTroopLvl1,
            TexturePackAssets::Troops_Gatling_GatlingTroopLvl2,
            TexturePackAssets::Troops_Gatling_GatlingTroopLvl3,
            TexturePackAssets::Troops_Gatling_GatlingTroopLvl3,
        ],

        tower_rotates: true,
    };

    pub const SNIPER_TROOP_ATTRIBUTES: TowerAttributes = TowerAttributes {
        price: 160,
        size_tiles: Vec2::splat(1.0),
        range_tiles: U16Vec2::splat(8),
        cooldown_ms: 4000,
        bullet_speed: 100.0,
        sprites: [
            TexturePackAssets::Troops_Sniper_SniperTroopLvl1,
            TexturePackAssets::Troops_Sniper_SniperTroopLvl2,
            TexturePackAssets::Troops_Sniper_SniperTroopLvl3,
            TexturePackAssets::Troops_Sniper_SniperTroopLvl3,
        ],
        tower_rotates: true,
    };

    pub const EITSHTU_ATTRIBUTES: TowerAttributes = TowerAttributes {
        price: 0,
        size_tiles: Vec2::splat(1.0),
        range_tiles: U16Vec2::splat(3),
        cooldown_ms: 300,
        bullet_speed: 10.0,
        sprites: [
            TexturePackAssets::ElementalRunes_Eitshtu_EitshtuLvl1,
            TexturePackAssets::ElementalRunes_Eitshtu_EitshtuLvl2,
            TexturePackAssets::ElementalRunes_Eitshtu_EitshtuLvl3,
            TexturePackAssets::ElementalRunes_Eitshtu_EitshtuLvl4,
        ],
        tower_rotates: false,
    };
}

// ui
pub mod ui {
    use bevy::prelude::Color;

    pub const BOUNDING_BOX_DEBUG_COLOR: Color = Color::hsv(0.3, 1.0, 1.0);

    pub mod grid {
        use crate::ui_overlay::grid::{FontSettings, GridTileColors};
        use bevy::prelude::{Color, FontWeight};

        pub const GRID_POSITION: FontSettings = FontSettings {
            font_size: 12.0,
            font_weight: FontWeight(160),
            color: Color::srgba(0.5, 1.0, 0.5, 0.5),
        };

        pub const GRID_META_POSITION: FontSettings = FontSettings {
            font_size: 18.0,
            font_weight: FontWeight(240),
            color: Color::srgba(1.0, 1.0, 1.0, 1.0),
        };

        pub const GRID_POSITION_TILE_COLORS: GridTileColors = GridTileColors {
            none: Color::srgba(0.0, 0.0, 0.0, 0.0),
            path_start: Color::srgba(1.0, 0.5, 1.0, 0.9),
            path: Color::srgba(1.0, 1.0, 0.5, 0.9),
            restricted: Color::srgba(1.0, 0.5, 0.5, 0.9),
            placeable: Color::srgba(0.5, 1.0, 0.5, 0.9),
            water: Color::srgba(0.5, 0.5, 1.0, 0.9),
        };

        pub const GRID_LINE_THICKNESS: f32 = 0.025;
        pub const GRID_LINE_COLOR: Color = Color::srgba(0.5, 1.0, 0.5, 0.2);
        pub const GRID_CONTRAST_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.5);
    }
}

pub mod rendering_layers {
    pub const MAP: f32 = 0.0;
    pub const CONTRAST: f32 = 1.0;
    pub const ENTITY: f32 = 5.0;
    pub const GRID: f32 = 10.0;
    pub const GRID_LABEL: f32 = 11.0;
    pub const HIGHLIGHT: f32 = 20.0;
}

pub const BASE_TEXTURE_PACK_PATH: &str = "assets/texture_packs/default";
