use crate::entities::enemies::{EnemyStats, EnemyType};
use crate::texture_packs::TexturePackAssets;
use crate::tiers::ValueTiers;

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
                reward: 100,
                health: ValueTiers::S,
                player_health_penalty: ValueTiers::S,
                speed_tps: ValueTiers::C,
                relative_collider_size: 1.0,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::WipSprites_MausMeister,
            },
            EnemyType::RocherOfTheNight => EnemyStats {
                reward: 50,
                health: ValueTiers::S,
                player_health_penalty: ValueTiers::C,
                speed_tps: ValueTiers::E,
                relative_collider_size: 0.5,
                texture_size_tiles: 2.0,
                asset: TexturePackAssets::Enemies_Rocher_RocherDarkmodeLvl1,
            },
            EnemyType::Rocher => EnemyStats {
                reward: 20,
                health: ValueTiers::A,
                player_health_penalty: ValueTiers::D,
                speed_tps: ValueTiers::D,
                relative_collider_size: 0.5,
                texture_size_tiles: 2.0,
                asset: TexturePackAssets::Enemies_Rocher_RocherLvl1,
            },
            EnemyType::ZapanoOfTheNight => EnemyStats {
                reward: 10,
                health: ValueTiers::C,
                player_health_penalty: ValueTiers::D,
                speed_tps: ValueTiers::C,
                relative_collider_size: 1.0,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::Enemies_Zapano_ZapanoDarkmodeFrontendLvl1,
            },
            EnemyType::ZapanoOfTheNightBody => EnemyStats {
                reward: 5,
                health: ValueTiers::D,
                player_health_penalty: ValueTiers::D,
                speed_tps: ValueTiers::C,
                relative_collider_size: 1.0,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::Enemies_Zapano_ZapanoDarkmodeBodyLvl1,
            },
            EnemyType::ZapanoOfTheNightBackend => EnemyStats {
                reward: 5,
                health: ValueTiers::D,
                player_health_penalty: ValueTiers::D,
                speed_tps: ValueTiers::C,
                relative_collider_size: 1.0,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::Enemies_Zapano_ZapanoDarkmodeBackendLvl1,
            },
            EnemyType::Zapano => EnemyStats {
                reward: 5,
                health: ValueTiers::D,
                player_health_penalty: ValueTiers::E,
                speed_tps: ValueTiers::B,
                relative_collider_size: 1.0,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::Enemies_Zapano_ZapanoFrontendLvl1,
            },
            EnemyType::ZapanoBody => EnemyStats {
                reward: 2,
                health: ValueTiers::F,
                player_health_penalty: ValueTiers::E,
                speed_tps: ValueTiers::B,
                relative_collider_size: 1.0,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::Enemies_Zapano_ZapanoBodyLvl1,
            },
            EnemyType::ZapanoBackend => EnemyStats {
                reward: 2,
                health: ValueTiers::F,
                player_health_penalty: ValueTiers::E,
                speed_tps: ValueTiers::B,
                relative_collider_size: 1.0,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::Enemies_Zapano_ZapanoBackendLvl1,
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
