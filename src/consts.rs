use bevy::math::Vec2;

pub const WINDOW_TITLE: &str = "2d game";
pub const WINDOW_RESOLUTION: [u32; 2] = [800, 450];
pub const PHYSICS_FRAME_RATE: u16 = 144;
pub const MAP_SIZE_TILES: [u16; 2] = [32, 16];
pub const TILE_SIZE: u16 = 16;

pub const RECT_SIZE: Vec2 = Vec2::new(140.0, 80.0);
pub const PROJECTILE_SIZE_TILES: Vec2 = Vec2::splat(0.3);
pub const ENEMY_SIZE_TILES: Vec2 = Vec2::splat(1.0);
pub const TOWER_SIZE_TILES: Vec2 = Vec2::splat(1.0);
pub const TOWER_RANGE_TILES: u16 = 5;

pub const PROJECTILE_RADIUS: f32 = PROJECTILE_SIZE_TILES.x / 2.0;
pub const ENEMY_RADIUS: f32 = ENEMY_SIZE_TILES.x / 2.0;

pub const ENEMY_PATH_DURATION_MS: u64 = 12000;

pub const BULLET_ROTATION_DURATION_MS: u64 = 234;

pub mod paths {
    pub mod sprite {
        pub const ANGRY_BIRB: &str = "sprites/angry_birb.png";
        pub const APPLE: &str = "sprites/apple.png";
        pub const ENEMY: &str = "sprites/enemy.jpg";
        pub const MAUS_MEISTER: &str = "sprites/maus_meister.jpg";
        pub const TURRET: &str = "sprites/turret.png";
    }
    pub mod map {
        pub const MAP_VISUAL_LAYER: &str = "maps/backrooms/visual_layer.png";
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
