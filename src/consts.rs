use bevy::math::Vec2;

pub const WINDOW_TITLE: &str = "2d game";
pub const WINDOW_RESOLUTION: [u32; 2] = [800, 450];
pub const MAP_SIZE_TILES: [u16; 2] = [32, 16];

pub(crate) const RECT_SIZE: Vec2 = Vec2::new(140.0, 80.0);
pub const ENEMY_SIZE_TILES: Vec2 = Vec2::splat(1.0);
pub const TOWER_SIZE_TILES: Vec2 = Vec2::splat(1.0);

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
