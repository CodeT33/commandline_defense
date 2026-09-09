use crate::entities::bullets::{BulletStats, BulletType};
use crate::entities::enemies::{EnemyStats, EnemyType};
use crate::entities::tower::{TargetingType, TowerAttributes, TowerType};
use crate::texture_packs::TexturePackAssets;
use crate::tiers::{ValueTiers, ValueType};
use bevy::math::U16Vec2;
use bevy::prelude::Vec2;

pub(crate) const WINDOW_TITLE: &str = "Commandline Defense";
pub(crate) const WINDOW_RESOLUTION: [u32; 2] = [800, 450];

pub(crate) const PHYSICS_FRAME_RATE: u16 = 144 * 2;

pub(crate) const MAP_SIZE_TILES: U16Vec2 = U16Vec2 { x: 32, y: 16 };
pub(crate) const TILE_SIZE: u16 = 16;

pub(crate) const ENEMY_SPAWN_INTERVAL_MS: u64 = 2000;

pub(crate) const TOWER_COOLDOWN_MS: u32 = 1000;

pub(crate) const PROJECTILE_SIZE_TILES: Vec2 = Vec2::splat(1.0);

pub(crate) const BULLET_ROTATION_DURATION_MS: u64 = 234;

/// Number of simulated seconds captured before the log is saved and the program is terminated.
#[cfg(feature = "determinism")]
pub(crate) const LOG_DURATION_SECS: u64 = 15;

pub(crate) mod viewports {
    use crate::camera::Viewport;

    pub(crate) const BASIC_CAMERA: Viewport =
        Viewport { min_zoom: 0.01, max_zoom: 0.5, zoom_speed: 0.1 };
}

pub(crate) mod map_logic_parsing {
    pub(crate) const PATH_START: u32 = 0xff00ff;
    pub(crate) const PATH: u32 = 0xffff00;
    pub(crate) const RESTRICTED: u32 = 0xff0000;
    pub(crate) const PLACEABLE: u32 = 0x00ff00;
    pub(crate) const WATER: u32 = 0x0000ff;
}

impl TowerType {
    pub(crate) fn get_attributes(self) -> TowerAttributes {
        match self {
            TowerType::AssaultTower => TowerAttributes {
                price: 100,
                size_tiles: Vec2::splat(1.0),
                range: 5.0,
                cooldown_ms: 1000,
                bullet_speed_tps: 10.0,
                bullet_type: BulletType::OrangeBall,
                sprites: [
                    TexturePackAssets::Troops_Assault_AssaultTroopLvl1,
                    TexturePackAssets::Troops_Assault_AssaultTroopLvl2,
                    TexturePackAssets::Troops_Assault_AssaultTroopLvl3,
                    TexturePackAssets::Troops_Assault_AssaultTroopLvl3,
                ],
                tower_rotates: true,
                targeting_type: TargetingType::PredictiveWithLoadBalancing,
            },
            TowerType::BoomTower => TowerAttributes {
                price: 320,
                size_tiles: Vec2::splat(1.0),
                range: 3.0,
                cooldown_ms: 3000,
                bullet_speed_tps: 6.0,
                bullet_type: BulletType::MetalBall,
                sprites: [
                    TexturePackAssets::Troops_Boom_BoomTroopLvl1,
                    TexturePackAssets::Troops_Boom_BoomTroopLvl2,
                    TexturePackAssets::Troops_Boom_BoomTroopLvl3,
                    TexturePackAssets::Troops_Boom_BoomTroopLvl3,
                ],
                tower_rotates: true,
                targeting_type: TargetingType::PredictiveWithLoadBalancing,
            },
            TowerType::GatlingTower => TowerAttributes {
                price: 210,
                size_tiles: Vec2::splat(1.0),
                range: 2.0,
                cooldown_ms: 100,
                bullet_speed_tps: 16.0,
                bullet_type: BulletType::AppleBall,
                sprites: [
                    TexturePackAssets::Troops_Gatling_GatlingTroopLvl1,
                    TexturePackAssets::Troops_Gatling_GatlingTroopLvl2,
                    TexturePackAssets::Troops_Gatling_GatlingTroopLvl3,
                    TexturePackAssets::Troops_Gatling_GatlingTroopLvl3,
                ],
                tower_rotates: true,
                targeting_type: TargetingType::PredictiveWithLoadBalancing,
            },
            TowerType::SniperTower => TowerAttributes {
                price: 160,
                size_tiles: Vec2::splat(1.0),
                range: 8.0,
                cooldown_ms: 4000,
                bullet_speed_tps: 100.0,
                bullet_type: BulletType::Bullet,
                sprites: [
                    TexturePackAssets::Troops_Sniper_SniperTroopLvl1,
                    TexturePackAssets::Troops_Sniper_SniperTroopLvl2,
                    TexturePackAssets::Troops_Sniper_SniperTroopLvl3,
                    TexturePackAssets::Troops_Sniper_SniperTroopLvl3,
                ],
                tower_rotates: true,
                targeting_type: TargetingType::PredictiveWithLoadBalancing,
            },
            TowerType::Eitshtu => TowerAttributes {
                price: 0,
                size_tiles: Vec2::splat(1.0),
                range: 3.0,
                cooldown_ms: 300,
                bullet_speed_tps: 10.0,
                bullet_type: BulletType::Bullet,
                sprites: [
                    TexturePackAssets::ElementalRunes_Eitshtu_EitshtuLvl1,
                    TexturePackAssets::ElementalRunes_Eitshtu_EitshtuLvl2,
                    TexturePackAssets::ElementalRunes_Eitshtu_EitshtuLvl3,
                    TexturePackAssets::ElementalRunes_Eitshtu_EitshtuLvl4,
                ],
                tower_rotates: false,
                targeting_type: TargetingType::PredictiveWithLoadBalancing,
            },
            TowerType::DonBanano => TowerAttributes {
                price: 0,
                size_tiles: Vec2::splat(3.0),
                range: 4.0,
                cooldown_ms: 500,
                bullet_speed_tps: 10.0,
                bullet_type: BulletType::DonsBananos,
                sprites: [
                    TexturePackAssets::WipSprites_DonBananoCool,
                    TexturePackAssets::WipSprites_DonBananoCool,
                    TexturePackAssets::WipSprites_DonBananoCool,
                    TexturePackAssets::WipSprites_DonBananoCool,
                ],
                tower_rotates: true,
                targeting_type: TargetingType::PredictiveWithLoadBalancing,
            },
            TowerType::RocketTroop => TowerAttributes {
                price: 0,
                size_tiles: Vec2::splat(2.0),
                range: 3.0,
                cooldown_ms: 2000,
                bullet_speed_tps: 10.0,
                bullet_type: BulletType::Rocket,
                sprites: [
                    TexturePackAssets::Troops_Rocket_RocketTroopLvl1,
                    TexturePackAssets::Troops_Rocket_RocketTroopLvl1,
                    TexturePackAssets::Troops_Rocket_RocketTroopLvl1,
                    TexturePackAssets::Troops_Rocket_RocketTroopLvl1,
                ],
                tower_rotates: true,
                targeting_type: TargetingType::PredictiveWithLoadBalancing,
            },
            tt => panic!("TowerType {tt:?} not implemented yet"),
        }
    }
}

impl EnemyType {
    pub(crate) fn get_stats(self) -> EnemyStats {
        match self {
            EnemyType::WideBirb => EnemyStats {
                health: 4.0,
                player_health_penalty: 4,
                speed_tps: 1.0,
                relative_collider_size: 0.25,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::WipSprites_Enemy,
            },
            EnemyType::Mausmeister => EnemyStats {
                health: 0.5,
                player_health_penalty: 1,
                speed_tps: 4.0,
                relative_collider_size: 0.25,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::WipSprites_MausMeister,
            },
            EnemyType::Rocher => EnemyStats {
                health: 10.0,
                player_health_penalty: 1,
                speed_tps: 0.5,
                relative_collider_size: 0.5,
                texture_size_tiles: 2.0,
                asset: TexturePackAssets::Enemies_Rocher_RocherDarkmodeLvl1,
            },
            EnemyType::Zapano => EnemyStats {
                health: 2.0,
                player_health_penalty: 1,
                speed_tps: 1.0,
                relative_collider_size: 0.25,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::Enemies_Zapano_ZapanoFrontendLvl1,
            },
        }
    }
}

impl BulletType {
    pub(crate) fn get_stats(self) -> BulletStats {
        match self {
            BulletType::Bullet => BulletStats {
                damage: 2.0,
                health: 20.0,
                spins: false,
                relative_collider_size: 0.25,
                texture_size_tiles: PROJECTILE_SIZE_TILES.x,
                asset: TexturePackAssets::Projectiles_NormalMunition,
            },
            BulletType::MetalBall => BulletStats {
                damage: 1.0,
                health: 3.0,
                spins: true,
                relative_collider_size: 0.25,
                texture_size_tiles: PROJECTILE_SIZE_TILES.x,
                asset: TexturePackAssets::Projectiles_MetalBall,
            },
            BulletType::AppleBall => BulletStats {
                damage: 0.1,
                health: 1.0,
                spins: true,
                relative_collider_size: 1.0,
                texture_size_tiles: 0.4,
                asset: TexturePackAssets::WipSprites_Apple,
            },
            BulletType::OrangeBall => BulletStats {
                damage: 3.0,
                health: 1.0,
                spins: true,
                relative_collider_size: 1.0,
                texture_size_tiles: 0.8,
                asset: TexturePackAssets::WipSprites_Enemy,
            },
            BulletType::DonsBananos => BulletStats {
                damage: 3.0,
                health: 1.0,
                spins: true,
                relative_collider_size: 1.0,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::WipSprites_Banana,
            },
            BulletType::Rocket => BulletStats {
                damage: 4.0,
                health: 1.0,
                spins: false,
                relative_collider_size: 1.0,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::Projectiles_OrangeRocket,
            },
        }
    }
}

// ui
pub(crate) mod ui {
    use bevy::prelude::Color;

    pub(crate) const BOUNDING_BOX_DEBUG_COLOR: Color = Color::hsv(120.0, 1.0, 1.0);
    pub(crate) const BOUNDING_BOX_DEBUG_COLOR_ALT: Color = Color::hsv(0.0, 1.0, 1.0);
    /// How often the debug bounding box color toggles between the two colors.
    pub(crate) const BOUNDING_BOX_DEBUG_COLOR_TOGGLE_SECS: f32 = 0.25;

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
}

pub(crate) mod rendering_layers {
    pub(crate) const MAP: f32 = 0.0;
    pub(crate) const CONTRAST: f32 = 1.0;
    pub(crate) const ENTITY: f32 = 5.0;
    pub(crate) const GRID: f32 = 10.0;
    pub(crate) const GRID_LABEL: f32 = 11.0;
    pub(crate) const HEALTH_BARS: f32 = 15.0;
    pub(crate) const HIGHLIGHT: f32 = 20.0;
}

pub(crate) const BASE_TEXTURE_PACK_PATH: &str = "assets/texture_packs/default";

impl ValueTiers {
    #[allow(unused)]
    pub fn get_value(&self, value_type: ValueType) -> f32 {
        match value_type {
            ValueType::Range => match self {
                ValueTiers::S => 1.0,
                ValueTiers::A => 1.0,
                ValueTiers::B => 1.0,
                ValueTiers::C => 1.0,
                ValueTiers::D => 1.0,
                ValueTiers::E => 1.0,
                ValueTiers::F => 1.0,
            },
            ValueType::BulletSpeed => match self {
                ValueTiers::S => 1.0,
                ValueTiers::A => 1.0,
                ValueTiers::B => 1.0,
                ValueTiers::C => 1.0,
                ValueTiers::D => 1.0,
                ValueTiers::E => 1.0,
                ValueTiers::F => 1.0,
            },
            ValueType::MovementSpeed => match self {
                ValueTiers::S => 1.0,
                ValueTiers::A => 1.0,
                ValueTiers::B => 1.0,
                ValueTiers::C => 1.0,
                ValueTiers::D => 1.0,
                ValueTiers::E => 1.0,
                ValueTiers::F => 1.0,
            },
            ValueType::ReloadSpeed => match self {
                ValueTiers::S => 1.0,
                ValueTiers::A => 1.0,
                ValueTiers::B => 1.0,
                ValueTiers::C => 1.0,
                ValueTiers::D => 1.0,
                ValueTiers::E => 1.0,
                ValueTiers::F => 1.0,
            },
            ValueType::BulletDamage => match self {
                ValueTiers::S => 1.0,
                ValueTiers::A => 1.0,
                ValueTiers::B => 1.0,
                ValueTiers::C => 1.0,
                ValueTiers::D => 1.0,
                ValueTiers::E => 1.0,
                ValueTiers::F => 1.0,
            },
            ValueType::AreaDamage => match self {
                ValueTiers::S => 1.0,
                ValueTiers::A => 1.0,
                ValueTiers::B => 1.0,
                ValueTiers::C => 1.0,
                ValueTiers::D => 1.0,
                ValueTiers::E => 1.0,
                ValueTiers::F => 1.0,
            },
        }
    }
}
