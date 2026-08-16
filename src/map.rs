use crate::consts;
use bevy::asset::AssetServer;
use bevy::prelude::*;

pub struct Map {
    enemies: Vec<[u16; 2]>,
    towers: Vec<[u16; 2]>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            enemies: vec![[0, 0], [1, 2], [5, 5], [5, 0]],
            towers: vec![[7, 2], [6, 5], [5, 3], [31, 15]],
        }
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Tower;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct BulletSpawnData {
    last_spawn_time_ms: Option<u64>,
    pub direction: Rot2,
    pub bullet_speed: f32,
    pub spawn_cooldown_ms: u32,
}

impl Default for BulletSpawnData {
    fn default() -> Self {
        Self {
            last_spawn_time_ms: None,
            direction: Rot2::degrees(0.0),
            bullet_speed: 10.0,
            spawn_cooldown_ms: 1000,
        }
    }
}

impl BulletSpawnData {
    /// Call this function in a loop until it returns None to ensure no bullets are dropped.\
    /// When a shot is available, the function returns the time at which the shot was fired. Otherwise, it returns None.
    pub fn shoot_if_ready(&mut self, current_time_ms: u64) -> Option<u64> {
        if let Some(last_spawn_time_ms) = &mut self.last_spawn_time_ms {
            if *last_spawn_time_ms + self.spawn_cooldown_ms as u64 <= current_time_ms {
                *last_spawn_time_ms += self.spawn_cooldown_ms as u64;
                Some(*last_spawn_time_ms)
            } else {
                None
            }
        } else {
            self.last_spawn_time_ms = Some(current_time_ms);
            Some(current_time_ms)
        }
    }
}

pub fn spawn_map(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let map = Map::default();
    for &enemy_pos in &map.enemies {
        commands.spawn((
            Enemy,
            Sprite {
                image: asset_server.load(consts::paths::sprite::ENEMY),
                custom_size: consts::ENEMY_SIZE_TILES.into(),
                ..default()
            },
            Transform::from_xyz(enemy_pos[0] as f32 + 0.5, enemy_pos[1] as f32 + 0.5, 0.0),
        ));
    }
    for &tower_pos in &map.towers {
        commands.spawn((
            Tower,
            Sprite {
                image: asset_server.load(consts::paths::sprite::TURRET),
                custom_size: consts::TOWER_SIZE_TILES.into(),
                image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
                ..default()
            },
            BulletSpawnData::default(),
            Transform::from_xyz(tower_pos[0] as f32 + 0.5, tower_pos[1] as f32 + 0.5, 0.0),
        ));
    }
}
