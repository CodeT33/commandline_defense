use crate::consts::{self};
use crate::coordinates::GridCoordinate;
use crate::ecs_elements::components::{
    BulletEmissionData, ColliderShape, ColliderTypeB, CreationTime, Enemy, HealthStats, TowerData,
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
use crate::tiers::ValueTiers;
use crate::tiers::ValueType::{
    BulletPierce, BulletSpeed, EnemyMovementSpeed, TowerRange, TowerReloadSpeed,
};
use bevy::asset::AssetServer;
use bevy::ecs::entity::EntityHashSet;
use bevy::math::{Quat, Rot2, Vec2};
use bevy::prelude::{
    Circle, Commands, Entity, MessageReader, MessageWriter, Query, Res, ResMut, Sprite,
    SpriteImageMode, SpriteScalingMode, Time, Transform, With, Without, default,
};
use clap::ValueEnum;
use std::f32::consts::PI;
use std::time::Duration;
use strum::EnumIter;

pub(crate) struct TowerDataInner {
    tower_type: TowerType,
    #[allow(unused)]
    upgrade_level: UpgradeLevel,
    #[allow(unused)]
    effects: Vec<Effect>,
    enemies_in_range: EntityHashSet,
    target: Vec<Entity>,
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

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, EnumIter)]
pub(crate) enum TowerType {
    AssaultTower,
    BoomTower,
    GatlingTower,
    SniperTower,
    Eitshtu,
    Acitonion,
    Strorm,
    Infernon,
    Icebyte,
    //Goldt,
    //Copprina,

    //meme stuff
    DonBanano,
    RocketTroop,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum TargetingType {
    #[allow(unused)]
    Basic,
    #[allow(unused)]
    Predictive,
    PredictiveWithLoadBalancing,
}

pub(crate) struct TowerAttributes {
    pub(crate) price: u16,
    pub(crate) size_tiles: Vec2,
    pub(crate) range: ValueTiers,
    pub(crate) cooldown_ms: ValueTiers,
    pub(crate) bullet_speed_tps: ValueTiers,
    pub(crate) bullet_type: BulletType,
    pub(crate) preview_sprite: TexturePackAssets,
    pub(crate) sprites: [TexturePackAssets; 4],
    pub(crate) tower_rotates: bool,
    pub(crate) targeting_type: TargetingType,
}

pub(crate) fn handle_tower_placing_events(
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

        let tower_pos = GridCoordinate::from_u16vec2(*message.tower_pos);
        let tower_data = TowerData(TowerDataInner {
            tower_type: message.tower_type,
            upgrade_level: UpgradeLevel::SmallSchlongKongStrong,
            effects: vec![],
            enemies_in_range: EntityHashSet::default(),
            target: vec![],
        });

        TowerData::spawn(
            &mut commands,
            &asset_server,
            &texture_pack_settings,
            tower_pos,
            tower_data,
        );
    }
}

impl TowerData {
    pub(crate) fn spawn(
        commands: &mut Commands, asset_server: &AssetServer,
        texture_pack_settings: &TexturePackSettings, tower_pos: GridCoordinate,
        tower_data: TowerData,
    ) {
        let attributes = tower_data.tower_type.get_attributes();
        let sprite: Sprite = Sprite {
            image: asset_server.load(texture_pack_settings.get_asset_path(attributes.sprites[0])),
            custom_size: attributes.size_tiles.into(),
            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
            ..default()
        };
        let bullet_emission_data = BulletEmissionData(BulletEmissionDataInner::new(
            attributes.cooldown_ms.get_value(TowerReloadSpeed) as u32,
        ));
        let collider_shape =
            ColliderShape::Circle(Circle::new(attributes.range.get_value(TowerRange)));
        _ = commands
            .spawn((
                tower_data,
                sprite,
                bullet_emission_data,
                collider_shape,
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

pub(crate) fn update_enemies_in_range(
    enemies: Query<Entity, With<Enemy>>, mut towers: Query<&mut TowerData>,
    mut collision_started: MessageReader<CollisionStarted>,
    mut collision_ended: MessageReader<CollisionEnded>,
) {
    for pair in collision_started.read() {
        let Some(Ok(mut tower)) =
            enemies.contains(pair.type_a).then(|| towers.get_mut(pair.type_b))
        else {
            continue;
        };
        tower.enemies_in_range.insert(pair.type_a);
    }
    for pair in collision_ended.read() {
        let Ok(mut tower) = towers.get_mut(pair.type_b) else {
            continue;
        };
        tower.enemies_in_range.remove(&pair.type_a);
    }
}

pub(crate) fn select_tower_target_enemy(
    enemy_transforms: Query<(Entity, &Enemy), With<Enemy>>, mut towers: Query<&mut TowerData>,
) {
    for mut tower in &mut towers {
        let mut enemies: Vec<_> =
            tower.enemies_in_range.iter().flat_map(|e| enemy_transforms.get(*e)).collect();
        // highest progress first
        enemies
            .sort_by(|a, b| a.1.get_path_progress().total_cmp(&b.1.get_path_progress()).reverse());

        tower.target = enemies.iter().map(|e| e.0).collect();
    }
}

pub(crate) fn shoot_bullets(
    mut bullet_spawns: MessageWriter<SpawnBullet>,
    enemies: Query<(&Enemy, &CreationTime, &Transform, &HealthStats), Without<TowerData>>,
    mut towers: Query<(&mut Transform, &TowerData, &mut BulletEmissionData)>, time: Res<Time>,
    map: Res<MapResource>,
) {
    for (mut tower_transform, tower_data, mut data) in &mut towers {
        // 0. if there are no enemies, pause and continue
        if tower_data.target.is_empty() {
            data.timer.pause();
            continue;
        }

        let tower_attributes = tower_data.tower_type.get_attributes();

        // 1. get next tick time -> if none available continue
        // 2. go through the enemies and if a target_pos is acquired, apply the tick and go back to 1.
        // 3. if not tick anyways
        while let Some(shoot_time) = data.timer.tick_if_ready(&time) {
            for (target_entity, target_enemy, enemy_creation_time, enemy_transform, enemy_health) in
                tower_data.target.iter().filter_map(|e| {
                    enemies
                        .get(*e)
                        .ok()
                        .map(|(enemy, ect, trfm, health)| (e, enemy, ect, trfm, health))
                })
            {
                let target_pos = if matches!(
                    tower_attributes.targeting_type,
                    TargetingType::Predictive | TargetingType::PredictiveWithLoadBalancing
                ) {
                    if tower_attributes.targeting_type == TargetingType::PredictiveWithLoadBalancing
                        && target_enemy.get_planned_bullet_damage() >= enemy_health.current_health()
                    {
                        continue;
                    }
                    let Some((target_pos, _hit_time)) = calculate_target_position(
                        enemy_creation_time.0,
                        shoot_time,
                        tower_transform.translation.truncate(),
                        map.enemy_path(),
                        tower_attributes.bullet_speed_tps.get_value(BulletSpeed),
                        target_enemy
                            .get_type()
                            .get_attributes()
                            .speed_tps
                            .get_value(EnemyMovementSpeed),
                    ) else {
                        continue;
                    };
                    target_pos
                } else {
                    enemy_transform.translation.truncate()
                };

                let angle = (target_pos - tower_transform.translation.truncate()).to_angle();

                if tower_attributes.tower_rotates {
                    tower_transform.rotation = Quat::from_rotation_z(angle - PI / 2.0);
                }

                let shoot_direction = Rot2::radians(angle);

                bullet_spawns.write(SpawnBullet {
                    bullet_type: tower_attributes.bullet_type,
                    time: shoot_time,
                    position: tower_transform.translation.truncate(),
                    direction: shoot_direction,
                    speed_tps: tower_attributes.bullet_speed_tps.get_value(BulletSpeed),
                    target_entity: matches!(
                        tower_attributes.targeting_type,
                        TargetingType::PredictiveWithLoadBalancing
                    )
                    .then_some(*target_entity),
                });
                break;
            }
        }
    }
}

fn calculate_target_position(
    enemy_creation_time: TimePoint, bullet_creation_time: TimePoint, bullet_position: Vec2,
    path: &EnemyPath, bullet_speed_tps: f32, enemy_speed_tps: f32,
) -> Option<(Vec2, TimePoint)> {
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
        let start = current_corner.position().as_vec2() + Vec2::splat(0.5);
        let end = next_corner.position().as_vec2() + Vec2::splat(0.5);
        // 6. Calculate position of enemy on the line at bullet shoot time + movement vector of enemy
        let enemy_life_time_at_start = bullet_creation_time - enemy_creation_time;
        let direction_line = (end - start).normalize();
        let line_start_at_t0 = start - direction_line * current_corner.path_length() as f32;
        let enemy_velocity = direction_line * enemy_speed_tps;
        let line_pos_at_shoot_time =
            line_start_at_t0 + enemy_velocity * enemy_life_time_at_start.as_secs_f32();

        // 7. Calculate t for bullet_speed, enemy_speed_v, enemy_pos, bullet_pos
        let d = line_pos_at_shoot_time - bullet_position;
        let t = calculate_collision_time(bullet_speed_tps, enemy_velocity, d)?;
        let hit_time_point = bullet_creation_time + Duration::from_secs_f32(t);
        let hit_pos = line_pos_at_shoot_time + enemy_velocity * t;
        return Some((hit_pos, hit_time_point));
    }

    None
}

/// Written using AI
/// - `vb` = bullet velocity
/// - `ve` = enemy velocity
/// - `d` = enemy_pos - tower_pos
pub(crate) fn calculate_collision_time(vb: f32, ve: Vec2, d: Vec2) -> Option<f32> {
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
