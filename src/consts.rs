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
pub(crate) const MAX_SIM_SPEED: f32 = 20.0;

pub(crate) const MAP_SIZE_TILES: U16Vec2 = U16Vec2 { x: 32, y: 16 };
pub(crate) const TILE_SIZE: u16 = 16;

pub(crate) const ENEMY_SPAWN_INTERVAL_MS: u64 = 2000;

pub(crate) const TOWER_COOLDOWN_MS: u32 = 1000;

pub(crate) const PROJECTILE_SIZE_TILES: Vec2 = Vec2::splat(1.0);

pub(crate) const BULLET_ROTATION_DURATION_MS: u64 = 234;

pub(crate) const COMMAND_OPEN_SEPARATION_CHARACTER: char = '/';

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
                range: ValueTiers::B,
                cooldown_ms: ValueTiers::B,
                bullet_speed_tps: ValueTiers::C,
                bullet_type: BulletType::SmallGoldBullet,
                preview_sprite: TexturePackAssets::Troops_Assault_AssaultTroopLvl1,
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
                range: ValueTiers::C,
                cooldown_ms: ValueTiers::E,
                bullet_speed_tps: ValueTiers::C,
                bullet_type: BulletType::MediumMetalBall,
                preview_sprite: TexturePackAssets::Troops_Boom_BoomTroopLvl1,
                sprites: [
                    TexturePackAssets::Troops_Boom_BoomTroopLvl1,
                    TexturePackAssets::Troops_Boom_BoomTroopLvl2,
                    TexturePackAssets::Troops_Boom_BoomTroopLvl3,
                    TexturePackAssets::Troops_Boom_BoomTroopLvl3,
                ],
                tower_rotates: true,
                targeting_type: TargetingType::Basic,
            },
            TowerType::GatlingTower => TowerAttributes {
                price: 210,
                size_tiles: Vec2::splat(1.0),
                range: ValueTiers::B,
                cooldown_ms: ValueTiers::S,
                bullet_speed_tps: ValueTiers::B,
                bullet_type: BulletType::SmallCopperBullet,
                preview_sprite: TexturePackAssets::Troops_Gatling_GatlingTroopLvl1,
                sprites: [
                    TexturePackAssets::Troops_Gatling_GatlingTroopLvl1,
                    TexturePackAssets::Troops_Gatling_GatlingTroopLvl2,
                    TexturePackAssets::Troops_Gatling_GatlingTroopLvl3,
                    TexturePackAssets::Troops_Gatling_GatlingTroopLvl3,
                ],
                tower_rotates: true,
                targeting_type: TargetingType::Basic,
            },
            TowerType::SniperTower => TowerAttributes {
                price: 160,
                size_tiles: Vec2::splat(1.0),
                range: ValueTiers::S,
                cooldown_ms: ValueTiers::F,
                bullet_speed_tps: ValueTiers::S,
                bullet_type: BulletType::MediumGoldBullet,
                preview_sprite: TexturePackAssets::Troops_Sniper_SniperTroopLvl1,
                sprites: [
                    TexturePackAssets::Troops_Sniper_SniperTroopLvl1,
                    TexturePackAssets::Troops_Sniper_SniperTroopLvl2,
                    TexturePackAssets::Troops_Sniper_SniperTroopLvl3,
                    TexturePackAssets::Troops_Sniper_SniperTroopLvl3,
                ],
                tower_rotates: true,
                targeting_type: TargetingType::PredictiveWithLoadBalancing,
            },
            TowerType::RocketTroop => TowerAttributes {
                price: 0,
                size_tiles: Vec2::splat(2.0),
                range: ValueTiers::B,
                cooldown_ms: ValueTiers::F,
                bullet_speed_tps: ValueTiers::F,
                bullet_type: BulletType::SmallOrangeRocket,
                preview_sprite: TexturePackAssets::Troops_Rocket_RocketTroopLvl1,
                sprites: [
                    TexturePackAssets::Troops_Rocket_RocketTroopLvl1,
                    TexturePackAssets::Troops_Rocket_RocketTroopLvl1,
                    TexturePackAssets::Troops_Rocket_RocketTroopLvl1,
                    TexturePackAssets::Troops_Rocket_RocketTroopLvl1,
                ],
                tower_rotates: true,
                targeting_type: TargetingType::PredictiveWithLoadBalancing,
            },
            TowerType::Eitshtu => TowerAttributes {
                price: 80,
                size_tiles: Vec2::splat(1.0),
                range: ValueTiers::C,
                cooldown_ms: ValueTiers::S,
                bullet_speed_tps: ValueTiers::A,
                bullet_type: BulletType::EitshtuProjectile,
                preview_sprite: TexturePackAssets::ElementalRunes_Eitshtu_EitshtuLvl1,
                sprites: [
                    TexturePackAssets::ElementalRunes_Eitshtu_EitshtuLvl1,
                    TexturePackAssets::ElementalRunes_Eitshtu_EitshtuLvl2,
                    TexturePackAssets::ElementalRunes_Eitshtu_EitshtuLvl3,
                    TexturePackAssets::ElementalRunes_Eitshtu_EitshtuLvl4,
                ],
                tower_rotates: false,
                targeting_type: TargetingType::Basic,
            },
            TowerType::Acitonion => TowerAttributes {
                price: 260,
                size_tiles: Vec2::splat(1.0),
                range: ValueTiers::D,
                cooldown_ms: ValueTiers::C,
                bullet_speed_tps: ValueTiers::C,
                bullet_type: BulletType::AcitonionProjectile,
                preview_sprite: TexturePackAssets::ElementalRunes_Acitonion_AcitonionLvl1,
                sprites: [
                    TexturePackAssets::ElementalRunes_Acitonion_AcitonionLvl1,
                    TexturePackAssets::ElementalRunes_Acitonion_AcitonionLvl2,
                    TexturePackAssets::ElementalRunes_Acitonion_AcitonionLvl3,
                    TexturePackAssets::ElementalRunes_Acitonion_AcitonionLvl4,
                ],
                tower_rotates: false,
                targeting_type: TargetingType::Basic,
            },
            TowerType::Strorm => TowerAttributes {
                price: 420,
                size_tiles: Vec2::splat(1.0),
                range: ValueTiers::A,
                cooldown_ms: ValueTiers::F,
                bullet_speed_tps: ValueTiers::A,
                bullet_type: BulletType::StrormProjectile,
                preview_sprite: TexturePackAssets::ElementalRunes_Strorm_StrormLvl1,
                sprites: [
                    TexturePackAssets::ElementalRunes_Strorm_StrormLvl1,
                    TexturePackAssets::ElementalRunes_Strorm_StrormLvl2,
                    TexturePackAssets::ElementalRunes_Strorm_StrormLvl3,
                    TexturePackAssets::ElementalRunes_Strorm_StrormLvl4,
                ],
                tower_rotates: false,
                targeting_type: TargetingType::PredictiveWithLoadBalancing,
            },
            TowerType::Infernon => TowerAttributes {
                price: 380,
                size_tiles: Vec2::splat(1.0),
                range: ValueTiers::B,
                cooldown_ms: ValueTiers::D,
                bullet_speed_tps: ValueTiers::E,
                bullet_type: BulletType::InfernonProjectile,
                preview_sprite: TexturePackAssets::ElementalRunes_Infernon_InfernonLvl1,
                sprites: [
                    TexturePackAssets::ElementalRunes_Infernon_InfernonLvl2,
                    TexturePackAssets::ElementalRunes_Infernon_InfernonLvl2,
                    TexturePackAssets::ElementalRunes_Infernon_InfernonLvl3,
                    TexturePackAssets::ElementalRunes_Infernon_InfernonLvl4,
                ],
                tower_rotates: false,
                targeting_type: TargetingType::PredictiveWithLoadBalancing,
            },
            TowerType::Icebyte => TowerAttributes {
                price: 95,
                size_tiles: Vec2::splat(1.0),
                range: ValueTiers::B,
                cooldown_ms: ValueTiers::C,
                bullet_speed_tps: ValueTiers::A,
                bullet_type: BulletType::IcebyteProjectile,
                preview_sprite: TexturePackAssets::ElementalRunes_Icebyte_IcebyteLvl1,
                sprites: [
                    TexturePackAssets::ElementalRunes_Icebyte_IcebyteLvl2,
                    TexturePackAssets::ElementalRunes_Icebyte_IcebyteLvl2,
                    TexturePackAssets::ElementalRunes_Icebyte_IcebyteLvl3,
                    TexturePackAssets::ElementalRunes_Icebyte_IcebyteLvl4,
                ],
                tower_rotates: false,
                targeting_type: TargetingType::Basic,
            },

            // Memes
            TowerType::DonBanano => TowerAttributes {
                price: 0,
                size_tiles: Vec2::splat(3.0),
                range: ValueTiers::A,
                cooldown_ms: ValueTiers::A,
                bullet_speed_tps: ValueTiers::A,
                bullet_type: BulletType::DonsBananos,
                preview_sprite: TexturePackAssets::WipSprites_DonBananoCool,
                sprites: [
                    TexturePackAssets::WipSprites_DonBananoCool,
                    TexturePackAssets::WipSprites_DonBananoCool,
                    TexturePackAssets::WipSprites_DonBananoCool,
                    TexturePackAssets::WipSprites_DonBananoCool,
                ],
                tower_rotates: true,
                targeting_type: TargetingType::PredictiveWithLoadBalancing,
            },
        }
    }
    pub(crate) fn get_description(self) -> String {
        match self {
            TowerType::AssaultTower => "Normal ahh tower".to_string(),
            _ => "No description...".to_string(),
        }
    }
}

impl EnemyType {
    pub(crate) fn get_attributes(self) -> EnemyStats {
        match self {
            EnemyType::WideBirb => EnemyStats {
                reward: 80,
                health: ValueTiers::C,
                player_health_penalty: ValueTiers::A,
                speed_tps: ValueTiers::B,
                relative_collider_size: 0.25,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::WipSprites_Enemy,
            },
            EnemyType::Mausmeister => EnemyStats {
                reward: 5000,
                health: ValueTiers::D,
                player_health_penalty: ValueTiers::A,
                speed_tps: ValueTiers::A,
                relative_collider_size: 0.25,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::WipSprites_MausMeister,
            },
            EnemyType::RocherOfTheNight => EnemyStats {
                reward: 500,
                health: ValueTiers::S,
                player_health_penalty: ValueTiers::A,
                speed_tps: ValueTiers::E,
                relative_collider_size: 0.5,
                texture_size_tiles: 2.0,
                asset: TexturePackAssets::Enemies_Rocher_RocherDarkmodeLvl1,
            },
            EnemyType::Rocher => EnemyStats {
                reward: 500,
                health: ValueTiers::A,
                player_health_penalty: ValueTiers::B,
                speed_tps: ValueTiers::D,
                relative_collider_size: 0.5,
                texture_size_tiles: 2.0,
                asset: TexturePackAssets::Enemies_Rocher_RocherLvl1,
            },
            EnemyType::ZapanoOfTheNight => EnemyStats {
                reward: 250,
                health: ValueTiers::C,
                player_health_penalty: ValueTiers::A,
                speed_tps: ValueTiers::C,
                relative_collider_size: 0.25,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::Enemies_Zapano_ZapanoDarkmodeFrontendLvl1,
            },
            EnemyType::Zapano => EnemyStats {
                reward: 250,
                health: ValueTiers::D,
                player_health_penalty: ValueTiers::E,
                speed_tps: ValueTiers::B,
                relative_collider_size: 0.5,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::Enemies_Zapano_ZapanoFrontendLvl1,
            },
        }
    }
    pub(crate) fn get_description(self) -> String {
        match self {
            EnemyType::Mausmeister => "Hail the master of mice!".to_string(),
            _ => "No description...".to_string(),
        }
    }
}

impl BulletType {
    pub(crate) fn get_attributes(self) -> BulletStats {
        match self {
            // For boom-tower
            BulletType::MediumMetalBall => BulletStats {
                damage: ValueTiers::B,
                pierce: ValueTiers::B,
                spins: true,
                relative_collider_size: 0.25,
                texture_size_tiles: PROJECTILE_SIZE_TILES.x,
                asset: TexturePackAssets::Projectiles_MetalBall,
            },
            BulletType::BigMetalBall => BulletStats {
                damage: ValueTiers::A,
                pierce: ValueTiers::B,
                spins: true,
                relative_collider_size: 0.5,
                texture_size_tiles: PROJECTILE_SIZE_TILES.x,
                asset: TexturePackAssets::Projectiles_BigMetalBall,
            },

            // For assault-tower
            BulletType::SmallGoldBullet => BulletStats {
                damage: ValueTiers::D,
                pierce: ValueTiers::E,
                spins: false,
                relative_collider_size: 0.25,
                texture_size_tiles: PROJECTILE_SIZE_TILES.x,
                asset: TexturePackAssets::Projectiles_SmallGoldBullet,
            },
            // For sniper-tower
            BulletType::MediumGoldBullet => BulletStats {
                damage: ValueTiers::A,
                pierce: ValueTiers::F,
                spins: false,
                relative_collider_size: 0.25,
                texture_size_tiles: PROJECTILE_SIZE_TILES.x,
                asset: TexturePackAssets::Projectiles_MediumGoldBullet,
            },
            // For gatling-tower
            BulletType::SmallCopperBullet => BulletStats {
                damage: ValueTiers::D,
                pierce: ValueTiers::F,
                spins: false,
                relative_collider_size: 0.5,
                texture_size_tiles: PROJECTILE_SIZE_TILES.x,
                asset: TexturePackAssets::Projectiles_SmallCopperBullet,
            },
            BulletType::MediumCopperBullet => BulletStats {
                damage: ValueTiers::C,
                pierce: ValueTiers::F,
                spins: false,
                relative_collider_size: 0.5,
                texture_size_tiles: PROJECTILE_SIZE_TILES.x,
                asset: TexturePackAssets::Projectiles_MediumCopperBullet,
            },
            BulletType::SmallOrangeRocket => BulletStats {
                damage: ValueTiers::B,
                pierce: ValueTiers::F,
                spins: false,
                relative_collider_size: 0.5,
                texture_size_tiles: PROJECTILE_SIZE_TILES.x,
                asset: TexturePackAssets::Projectiles_SmallOrangeRocket,
            },
            // For rocket-tower
            BulletType::MediumOrangeRocket => BulletStats {
                damage: ValueTiers::S,
                pierce: ValueTiers::F,
                spins: false,
                relative_collider_size: 0.5,
                texture_size_tiles: PROJECTILE_SIZE_TILES.x,
                asset: TexturePackAssets::Projectiles_MediumOrangeRocket,
            },

            // Elemental rune projectiles
            BulletType::EitshtuProjectile => BulletStats {
                damage: ValueTiers::E,
                pierce: ValueTiers::F,
                spins: false,
                relative_collider_size: 0.5,
                texture_size_tiles: PROJECTILE_SIZE_TILES.x,
                asset: TexturePackAssets::Projectiles_EitshtuProjectile,
            },
            BulletType::AcitonionProjectile => BulletStats {
                damage: ValueTiers::C,
                pierce: ValueTiers::B,
                spins: false,
                relative_collider_size: 0.5,
                texture_size_tiles: PROJECTILE_SIZE_TILES.x,
                asset: TexturePackAssets::Projectiles_AcitonionProjectile,
            },
            BulletType::StrormProjectile => BulletStats {
                damage: ValueTiers::A,
                pierce: ValueTiers::F,
                spins: false,
                relative_collider_size: 0.5,
                texture_size_tiles: PROJECTILE_SIZE_TILES.x,
                asset: TexturePackAssets::Projectiles_StrormProjectile,
            },
            BulletType::InfernonProjectile => BulletStats {
                damage: ValueTiers::A,
                pierce: ValueTiers::F,
                spins: false,
                relative_collider_size: 1.0,
                texture_size_tiles: PROJECTILE_SIZE_TILES.x,
                asset: TexturePackAssets::Projectiles_InfernonProjectile,
            },
            BulletType::IcebyteProjectile => BulletStats {
                damage: ValueTiers::C,
                pierce: ValueTiers::S,
                spins: true,
                relative_collider_size: 1.0,
                texture_size_tiles: PROJECTILE_SIZE_TILES.x,
                asset: TexturePackAssets::Projectiles_IcebyteProjectile,
            },

            // Memes
            BulletType::AppleBall => BulletStats {
                damage: ValueTiers::E,
                pierce: ValueTiers::F,
                spins: true,
                relative_collider_size: 1.0,
                texture_size_tiles: 0.4,
                asset: TexturePackAssets::WipSprites_Apple,
            },
            BulletType::OrangeBall => BulletStats {
                damage: ValueTiers::E,
                pierce: ValueTiers::E,
                spins: true,
                relative_collider_size: 1.0,
                texture_size_tiles: 0.8,
                asset: TexturePackAssets::WipSprites_Enemy,
            },
            BulletType::DonsBananos => BulletStats {
                damage: ValueTiers::A,
                pierce: ValueTiers::A,
                spins: true,
                relative_collider_size: 1.0,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::WipSprites_Banana,
            },
        }
    }
}

// ui
pub(crate) mod ui {
    use bevy::prelude::Color;

    pub(crate) const CONSOLE_ERROR_COLOR: Color = Color::linear_rgb(1.0, 0.0, 0.0);
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
    pub(crate) fn get_value(&self, value_type: ValueType) -> f32 {
        match value_type {
            // Tower
            ValueType::TowerRange => match self {
                ValueTiers::S => 16.0,
                ValueTiers::A => 8.0,
                ValueTiers::B => 5.0,
                ValueTiers::C => 4.0,
                ValueTiers::D => 3.0,
                ValueTiers::E => 2.0,
                ValueTiers::F => 1.0,
            },
            ValueType::TowerReloadSpeed => match self {
                ValueTiers::S => 250.0,
                ValueTiers::A => 500.0,
                ValueTiers::B => 750.0,
                ValueTiers::C => 1000.0,
                ValueTiers::D => 2500.0,
                ValueTiers::E => 3500.0,
                ValueTiers::F => 5000.0,
            },
            ValueType::TowerAreaDamage => match self {
                ValueTiers::S => 1.0,
                ValueTiers::A => 1.0,
                ValueTiers::B => 1.0,
                ValueTiers::C => 1.0,
                ValueTiers::D => 1.0,
                ValueTiers::E => 1.0,
                ValueTiers::F => 1.0,
            },

            // Bullet
            ValueType::BulletSpeed => match self {
                ValueTiers::S => 15.0,
                ValueTiers::A => 10.0,
                ValueTiers::B => 6.0,
                ValueTiers::C => 4.0,
                ValueTiers::D => 3.0,
                ValueTiers::E => 2.0,
                ValueTiers::F => 1.0,
            },
            ValueType::BulletDamage => match self {
                ValueTiers::S => 25.0,
                ValueTiers::A => 10.0,
                ValueTiers::B => 4.0,
                ValueTiers::C => 2.0,
                ValueTiers::D => 1.5,
                ValueTiers::E => 1.0,
                ValueTiers::F => 0.5,
            },
            ValueType::BulletPierce => match self {
                ValueTiers::S => 10.0,
                ValueTiers::A => 8.0,
                ValueTiers::B => 6.0,
                ValueTiers::C => 4.0,
                ValueTiers::D => 3.0,
                ValueTiers::E => 2.0,
                ValueTiers::F => 1.0,
            },

            // Enemy
            ValueType::EnemyMovementSpeed => match self {
                ValueTiers::S => 10.0,
                ValueTiers::A => 7.5,
                ValueTiers::B => 5.0,
                ValueTiers::C => 3.0,
                ValueTiers::D => 1.5,
                ValueTiers::E => 1.0,
                ValueTiers::F => 0.5,
            },
            ValueType::EnemyHealth => match self {
                ValueTiers::S => 100.0,
                ValueTiers::A => 25.0,
                ValueTiers::B => 10.0,
                ValueTiers::C => 7.0,
                ValueTiers::D => 5.0,
                ValueTiers::E => 2.0,
                ValueTiers::F => 1.0,
            },
            ValueType::EnemyPlayerHealthPenalty => match self {
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

pub(crate) mod waves {
    use crate::entities::enemies::EnemyType;
    use crate::waves::{GameWaves, Wave, WaveItem};

    pub(crate) fn get_game_waves() -> GameWaves {
        GameWaves::new(vec![
            Wave::new(
                vec![
                    WaveItem::new_enemy(EnemyType::Rocher, 3000, 3),
                    WaveItem::new_enemy(EnemyType::Zapano, 1000, 10),
                    WaveItem::new_pause(2000),
                ],
                500,
            ),
            Wave::new(vec![WaveItem::new_enemy(EnemyType::RocherOfTheNight, 5000, 3)], 250),
        ])
    }
}
