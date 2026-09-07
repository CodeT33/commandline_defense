use crate::collision::CollisionPair;
use crate::consts::{self};
use crate::coordinates::GridCoordinate;
use crate::ecs_elements::components::{
    BulletEmissionData, ColliderShape, ColliderTypeB, CreationTime, Enemy, Tower, TowerData,
};
use crate::ecs_elements::messages::{CollisionEnded, CollisionStarted, PlaceTowerMessage};
use crate::ecs_elements::resources::{PlayerSuiteResource, TexturePackSettings};
use crate::entities::bullets::{BulletEmissionDataInner, BulletType};
use crate::player_suite::TransactionReturnStatus;
use crate::texture_packs::TexturePackAssets;
use bevy::asset::AssetServer;
use bevy::math::{Quat, Rot2, U16Vec2, Vec2};
use bevy::prelude::{
    Circle, Commands, Entity, MessageReader, Query, Res, ResMut, Sprite, SpriteImageMode,
    SpriteScalingMode, Transform, With, Without, default,
};
use std::f32::consts::PI;

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

        let tower_pos =
            GridCoordinate::new(message.tower_pos.position.x, message.tower_pos.position.y);
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
                    tower_pos.position.x as f32 + 0.5,
                    tower_pos.position.y as f32 + 0.5,
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
        let center = pos_tiles.position;
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
        let index = tile.position.y as usize * self.size.x as usize + tile.position.x as usize;
        &self.towers_in_range[index]
    }
}

pub fn update_towers_in_range_and_rotate(
    enemies: Query<Entity, With<Enemy>>,
    enemy_transforms: Query<(&Transform, &Enemy, &CreationTime), Without<Tower>>,
    mut towers: Query<(Entity, &mut Transform, &mut Tower, &mut BulletEmissionData)>,
    mut collision_started: MessageReader<CollisionStarted>,
    mut collision_ended: MessageReader<CollisionEnded>,
) {
    for CollisionStarted(CollisionPair { type_a, type_b }) in collision_started.read() {
        let Some(Ok((_, _, mut tower, _))) =
            enemies.contains(*type_a).then(|| towers.get_mut(*type_b))
        else {
            continue;
        };
        tower.enemies_in_range.insert(*type_a);
    }
    for CollisionEnded(CollisionPair { type_a, type_b }) in collision_ended.read() {
        let Ok((_, _, mut tower, _)) = towers.get_mut(*type_b) else {
            continue;
        };
        tower.enemies_in_range.remove(type_a);
    }

    for (_, mut t, tower, mut bullet_data) in &mut towers {
        let first_enemy = tower
            .enemies_in_range
            .iter()
            .flat_map(|e| enemy_transforms.get(*e))
            .max_by(|a, b| a.1.0.get_path_progress().total_cmp(&b.1.0.get_path_progress()));
        let Some((enemy_transform, _, _creation_time)) = first_enemy else {
            continue;
        };
        let bullet_pos = t.translation.truncate();

        let target_pos = enemy_transform.translation.truncate();

        let angle = (target_pos - bullet_pos).to_angle();
        t.rotation = Quat::from_rotation_z(angle - PI / 2.0);
        bullet_data.0.direction = Rot2::radians(angle);
    }
}
