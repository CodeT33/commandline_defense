use crate::consts::PROJECTILE_SIZE_TILES;
use crate::entities::bullets::{BulletStats, BulletType};
use crate::texture_packs::TexturePackAssets;
use crate::tiers::ValueTiers;

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
                pierce: ValueTiers::F,
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
                pierce: ValueTiers::D,
                spins: false,
                relative_collider_size: 0.5,
                texture_size_tiles: PROJECTILE_SIZE_TILES.x,
                asset: TexturePackAssets::Projectiles_SmallOrangeRocket,
            },
            // For rocket-tower
            BulletType::MediumOrangeRocket => BulletStats {
                damage: ValueTiers::S,
                pierce: ValueTiers::D,
                spins: false,
                relative_collider_size: 0.5,
                texture_size_tiles: PROJECTILE_SIZE_TILES.x,
                asset: TexturePackAssets::Projectiles_MediumOrangeRocket,
            },

            // Elemental rune projectiles
            BulletType::EitshtuProjectile => BulletStats {
                damage: ValueTiers::F,
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
                damage: ValueTiers::S,
                pierce: ValueTiers::S,
                spins: true,
                relative_collider_size: 1.0,
                texture_size_tiles: 1.0,
                asset: TexturePackAssets::WipSprites_Banana,
            },
        }
    }
}
