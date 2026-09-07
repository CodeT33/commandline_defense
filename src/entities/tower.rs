use crate::collision::CollisionPair;
use crate::consts::{self};
use crate::coordinates::GridCoordinate;
use crate::ecs_elements::components::{
    BulletEmissionData, ColliderShape, ColliderTypeB, CreationTime, Enemy, Tower, TowerData,
};
use crate::ecs_elements::messages::{
    CollisionEnded, CollisionStarted, PlaceTowerMessage, SpawnBullet,
};
use crate::ecs_elements::resources::{MapResource, PlayerSuiteResource, TexturePackSettings};
use crate::entities::bullets::{BulletEmissionDataInner, BulletType};
use crate::map::map_logic_parsing::EnemyPath;
use crate::player_suite::TransactionReturnStatus;
use crate::scheduling::TimePoint;
use crate::texture_packs::TexturePackAssets;
use bevy::asset::AssetServer;
use bevy::math::{Quat, Rot2, U16Vec2, Vec2};
use bevy::prelude::{
    Circle, Commands, Entity, MessageReader, MessageWriter, Query, Res, ResMut, Sprite,
    SpriteImageMode, SpriteScalingMode, Time, Transform, With, default,
};
use std::f32::consts::PI;
use std::time::Duration;

pub struct TowerDataInner {
    #[allow(unused)]
    tower_type: TowerType,
    #[allow(unused)]
    upgrade_level: UpgradeLevel,
    #[allow(unused)]
    effects: Vec<Effect>,
    pub bullet_speed_tps: f32,
    pub bullet_type: BulletType,
}

#[allow(unused)]
enum UpgradeLevel {
    SmallSchlongKongStrong,
    SchlongMediumIchKackeImTediRum,
    SchlongusLongus,
    MaximusBigschlongus,
    UnbreakableSnake,
}

#[allow(unused)]
enum Effect {
    BallBoost,
    BigBirbMode,
}

#[derive(Debug, Clone, Copy)]
pub enum TowerType {
    AssaultTower,
    BoomTower,
    GatlingTower,
    SniperTower,

    Eitshtu,
    Acitonion,
    Strorm,
    Infernon,
    Icebyte,
    Goldt,
    Copprina,
}

pub struct TowerRangeMapInner {
    pub size: U16Vec2,
    towers_in_range: Vec<Vec<Entity>>,
}

pub struct TowerAttributes {
    pub price: u16,
    pub size_tiles: Vec2,
    pub range_tiles: U16Vec2,
    pub cooldown_ms: u32,
    pub bullet_speed_tps: f32,
    pub bullet_type: BulletType,
    pub sprites: [TexturePackAssets; 4],
    pub tower_rotates: bool,
}

impl Default for TowerRangeMapInner {
    fn default() -> Self {
        let size = U16Vec2::from_array(<[u16; 2]>::from(consts::MAP_SIZE_TILES));
        Self { size, towers_in_range: vec![Vec::new(); (size.x * size.y) as usize] }
    }
}

pub fn handle_tower_placing_events(
    mut messages: MessageReader<PlaceTowerMessage>, mut commands: Commands,
    asset_server: Res<AssetServer>, mut player_suite: ResMut<PlayerSuiteResource>,
    texture_pack_settings: Res<TexturePackSettings>,
) {
    for message in messages.read() {
        let attributes: TowerAttributes = message.tower_type.get_attributes();

        if player_suite.perform_transaction(attributes.price)
            == TransactionReturnStatus::NotEnoughMoney
        {
            println!("Not enough money!");
            continue;
        }
        println!("Performing transaction of {:?}", attributes.price);

        let tower_pos = GridCoordinate::new(message.tower_pos.x, message.tower_pos.y);
        let bullet_emission_data: BulletEmissionData =
            BulletEmissionData(BulletEmissionDataInner::new(attributes.cooldown_ms));
        let sprite: Sprite = Sprite {
            image: asset_server.load(texture_pack_settings.get_asset_path(attributes.sprites[0])),
            custom_size: attributes.size_tiles.into(),
            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
            ..default()
        };
        let tower_data = TowerData(TowerDataInner {
            tower_type: message.tower_type,
            upgrade_level: UpgradeLevel::SmallSchlongKongStrong,
            effects: vec![],
            bullet_speed_tps: attributes.bullet_speed_tps,
            bullet_type: attributes.bullet_type,
        });

        Tower::spawn(&mut commands, sprite, tower_pos, tower_data, bullet_emission_data);
    }
}

impl Tower {
    pub fn spawn(
        commands: &mut Commands, sprite: Sprite, tower_pos: GridCoordinate, tower_data: TowerData,
        bullet_emission_data: BulletEmissionData,
    ) {
        _ = commands
            .spawn((
                Tower::default(),
                tower_data,
                sprite,
                bullet_emission_data,
                ColliderShape::Circle(Circle::new(consts::TOWER_RANGE_TILES as f32)),
                ColliderTypeB,
                Transform::from_xyz(
                    tower_pos.x as f32 + 0.5,
                    tower_pos.y as f32 + 0.5,
                    consts::rendering_layers::ENTITY,
                ),
            ))
            .id();
    }
}

impl TowerRangeMapInner {
    pub fn clear(&mut self) {
        for x in &mut self.towers_in_range {
            x.clear();
        }
    }

    pub fn range_bounds(&self, pos_tiles: GridCoordinate, range_tiles: u16) -> (U16Vec2, U16Vec2) {
        let center = pos_tiles;
        let range = U16Vec2::splat(range_tiles);
        let min = center.saturating_sub(range);
        let max = center.saturating_add(range).min(self.size.saturating_sub(U16Vec2::ONE));
        (min, max)
    }

    pub fn add_range_rect(&mut self, pos_tiles: GridCoordinate, range_tiles: u16, entity: Entity) {
        let (min, max) = self.range_bounds(pos_tiles, range_tiles);
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                self.towers_in_range[(y * self.size.x + x) as usize].push(entity);
            }
        }
    }

    pub fn towers_in_range_at(&self, tile: GridCoordinate) -> &[Entity] {
        let index = tile.y as usize * self.size.x as usize + tile.x as usize;
        &self.towers_in_range[index]
    }
}

pub fn update_enemies_in_range(
    enemies: Query<Entity, With<Enemy>>, mut towers: Query<(Entity, &mut Tower)>,
    mut collision_started: MessageReader<CollisionStarted>,
    mut collision_ended: MessageReader<CollisionEnded>,
) {
    for CollisionStarted(CollisionPair { type_a, type_b }) in collision_started.read() {
        let Some(Ok((_, mut tower))) = enemies.contains(*type_a).then(|| towers.get_mut(*type_b))
        else {
            continue;
        };
        tower.enemies_in_range.insert(*type_a);
    }
    for CollisionEnded(CollisionPair { type_a, type_b }) in collision_ended.read() {
        let Ok((_, mut tower)) = towers.get_mut(*type_b) else {
            continue;
        };
        tower.enemies_in_range.remove(type_a);
    }
}

pub fn select_tower_target_enemy(
    enemy_transforms: Query<(Entity, &Enemy), With<Enemy>>, mut towers: Query<&mut Tower>,
) {
    for mut tower in &mut towers {
        let first_enemy = tower
            .enemies_in_range
            .iter()
            .flat_map(|e| enemy_transforms.get(*e))
            .max_by(|a, b| a.1.0.get_path_progress().total_cmp(&b.1.0.get_path_progress()));

        tower.target = first_enemy.map(|e| e.0);
    }
}

pub fn request_bullet_spawns(
    mut bullet_spawns: MessageWriter<SpawnBullet>, enemies: Query<(&Enemy, &CreationTime)>,
    mut towers: Query<(&mut Transform, &Tower, &TowerData, &mut BulletEmissionData), With<Tower>>,
    time: Res<Time>, map: Res<MapResource>,
) {
    for (mut tower_transform, tower, tower_data, mut data) in &mut towers {
        let emission_data = &mut data.0;
        let Some((target_enemy, enemy_creation_time)) =
            tower.target.and_then(|entity| enemies.get(entity).ok())
        else {
            emission_data.timer.pause();
            continue;
        };

        while let Some(shoot_time) = emission_data.timer.tick_if_ready(&time) {
            let (target_pos, _hit_time) = calculate_target_position(
                enemy_creation_time.0,
                shoot_time,
                tower_transform.translation.truncate(),
                map.0.enemy_path(),
                tower_data.0.bullet_speed_tps,
                target_enemy.0.get_type().get_stats().speed_tps,
            );

            let angle = (target_pos - tower_transform.translation.truncate()).to_angle();
            tower_transform.rotation = Quat::from_rotation_z(angle - PI / 2.0);
            let shoot_direction = Rot2::radians(angle);

            bullet_spawns.write(SpawnBullet {
                bullet_type: tower_data.0.bullet_type,
                time: shoot_time,
                position: tower_transform.translation.truncate(),
                direction: shoot_direction,
                speed_tps: tower_data.0.bullet_speed_tps,
            });
        }
    }
}

fn calculate_target_position(
    enemy_creation_time: TimePoint, bullet_creation_time: TimePoint, bullet_position: Vec2,
    path: &EnemyPath, bullet_speed_tps: f32, enemy_speed_tps: f32,
) -> (Vec2, TimePoint) {
    for corners in path.corners().windows(2) {
        // 1. Determine next corner
        let current_corner = corners[0];
        let next_corner = corners[1];

        // 2. Calculate Enemy and Bullet Time for hit
        let corner_pos = next_corner.position().as_vec2() + Vec2::splat(0.5);

        let bullet_duration_secs = corner_pos.distance(bullet_position) / bullet_speed_tps;
        let bullet_arrive_time =
            bullet_creation_time + Duration::from_secs_f32(bullet_duration_secs);

        let enemy_duration_secs = next_corner.path_length() as f32 / enemy_speed_tps;
        let enemy_arrive_time = enemy_creation_time + Duration::from_secs_f32(enemy_duration_secs);

        // 3. If bullet arrives earlier than enemy, then range is this corner and the one before that
        // 4. Else increase corner idx by 1
        if bullet_arrive_time > enemy_arrive_time {
            continue;
        }

        // 5. Calculate Line
        // 6. Calculate position of enemy on the line at bullet shoot time
        // 7. Calculate movement vector of enemy
        // 8. Calculate t for bullet_speed, enemy_speed_v, enemy_pos, bullet_pos
    }

    todo!()
}

/// - `vb` = bullet velocity
/// - `ve` = enemy velocity
/// - `d` = enemy_pos - tower_pos
pub fn calculate_collision_time(vb: f32, ve: Vec2, d: Vec2) -> Option<f32> {
    // a = ||ve||^2 - vb^2
    let a = (-vb).mul_add(vb, ve.length_squared());
    let h = d.dot(ve);
    let c = d.length_squared();

    // disc = h^2 - a * c
    let disc = (-a).mul_add(c, h * h);

    // Target cannot be intercepted
    if disc < 0.0 {
        return None;
    }

    // Stable formulation: t = c / (sqrt(disc) - h)
    // Works seamlessly across a < 0, a == 0, and a > 0
    let denom = disc.sqrt() - h;

    if denom > 0.0 { Some(c / denom) } else { None }
}
