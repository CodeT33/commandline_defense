use bevy::math::Vec2;

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

        pub struct FontSettings {
            pub font_size: f32,
            pub font_weight: FontWeight,
            pub color: Color,
        }

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

        pub struct GridTileColors {
            pub none: Color,
            pub path_start: Color,
            pub path: Color,
            pub restricted: Color,
            pub placeable: Color,
            pub water: Color,
        }

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

pub mod paths {
    pub mod sprite {
        pub const ANGRY_BIRB: &str = "sprites/angry_birb.png";
        pub const APPLE: &str = "sprites/apple.png";
        pub const ENEMY: &str = "sprites/enemy.jpg";
        pub const MAUS_MEISTER: &str = "sprites/maus_meister.jpg";
        pub const TURRET: &str = "sprites/turret.png";
    }
    pub mod ui {
        pub const SELECTION_SQUARE: &str = "sprites/selection_square_arrow_top.png";
    }
    pub mod map {
        pub const MAP_BORDER: &str = "sprites/border_test.png";
        pub const MAP_PACKAGE_FOLDER: &str = "assets/maps/backrooms/";
        pub const MAP_VISUAL_LAYER: &str = "maps/one_bit_castle/visual_layer.png";
        pub const MAP_LOGIC_LAYER: &str = "assets/maps/one_bit_castle/logic_layer.png";
    }
    pub mod resources {
        pub mod enemies {
            pub const ENEMY_1: &str = "resource_packs/base_pack/enemies/enemy_1.png";
        }
        pub mod particles {}
        pub mod projectiles {
            pub const METAL_BALL: &str = "resource_packs/base_pack/particles/metal_ball.png";
            pub const NORMAL_MUNITION: &str =
                "resource_packs/base_pack/particles/normal_munition.png";
        }
        pub mod towers {
            pub mod assault_tower {
                pub const ASSAULT_TOWER_0_0_0: &str =
                    "resource_packs/base_pack/towers/assault_tower/0_0_0.png";
            }
            pub mod boom_tower {
                pub const BOOM_TOWER_0_0_0: &str =
                    "resource_packs/base_pack/towers/boom_tower/0_0_0.png";
            }
            pub mod gatling_tower {
                pub const GATLING_TOWER: &str =
                    "resource_packs/base_pack/towers/gatling_tower/0_0_0.png";
            }
            pub mod sniper_tower {
                pub const SNIPER_TOWER: &str =
                    "resource_packs/base_pack/towers/sniper_tower/0_0_0.png";
            }
        }
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
