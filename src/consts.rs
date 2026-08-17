use bevy::math::Vec2;

pub const WINDOW_TITLE: &str = "2d game";
pub const WINDOW_RESOLUTION: [u32; 2] = [800, 450];
pub const PHYSICS_FRAME_RATE: u16 = 144;
pub const MAP_SIZE_TILES: [u16; 2] = [32, 16];

pub const RECT_SIZE: Vec2 = Vec2::new(140.0, 80.0);
pub const PROJECTILE_SIZE_TILES: Vec2 = Vec2::splat(0.3);
pub const ENEMY_SIZE_TILES: Vec2 = Vec2::splat(1.0);
pub const TOWER_SIZE_TILES: Vec2 = Vec2::splat(1.0);
pub const TOWER_RANGE_TILES: u16 = 5;

pub const PROJECTILE_RADIUS: f32 = PROJECTILE_SIZE_TILES.x / 2.0;
pub const ENEMY_RADIUS: f32 = ENEMY_SIZE_TILES.x / 2.0;

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
        pub const BACKROOMS: &str = "maps/backrooms";
    }
}
