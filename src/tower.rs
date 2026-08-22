use crate::bullets::BulletEmissionData;
use crate::consts;
use crate::consts::towers::TowerAttributes;
use crate::game_cli::command_event_handling::PlaceTowerMessage;
use bevy::asset::AssetServer;
use bevy::ecs::entity::EntityHashSet;
use bevy::math::{Rot2, U16Vec2};
use bevy::prelude::{
    Commands, Component, Entity, MessageReader, Res, ResMut, Resource, Sprite, SpriteImageMode,
    SpriteScalingMode, Transform, default,
};

#[derive(Component, Default)]
pub struct Tower {
    pub enemies_in_range: EntityHashSet,
}

#[warn(unused)]
enum UpgradeLevel {
    SmallSchlongKongStrong,
    SchlongMediumIchKackeImTediRum,
    SchlongusLongus,
    MaximusBigschlongus,
    UnbreakableSnake,
}

#[warn(unused)]
enum Effect {
    BallBoost,
    BigBirbMode,
}

#[warn(unused)]
#[derive(Component)]
pub struct TowerData {
    tower_type: TowerType,
    upgrade_level: UpgradeLevel,
    effects: Vec<Effect>,
}

#[derive(Resource)]
pub struct TowerRangeMap {
    pub size: U16Vec2,
    towers_in_range: Vec<Vec<Entity>>,
}

#[derive(Debug, Clone, Copy)]
pub enum TowerType {
    None,
    AssaultTower,
    BoomTower,
    GatlingTower,
    SniperTower,
}

impl Default for TowerRangeMap {
    fn default() -> Self {
        let size = U16Vec2::from_array(consts::MAP_SIZE_TILES);
        Self { size, towers_in_range: vec![Vec::new(); (size.x * size.y) as usize] }
    }
}

pub fn handle_tower_placing_events(
    mut messages: MessageReader<PlaceTowerMessage>, mut commands: Commands,
    asset_server: Res<AssetServer>, mut tower_range_map: ResMut<TowerRangeMap>,
) {
    for message in messages.read() {
        let attributes: TowerAttributes = match message.tower_type {
            TowerType::None => continue,
            TowerType::AssaultTower => consts::towers::ASSAULT_TOWER_ATTRIBUTES,
            TowerType::BoomTower => consts::towers::BOOM_TOWER_ATTRIBUTES,
            TowerType::GatlingTower => consts::towers::GATLING_TOWER_ATTRIBUTES,
            TowerType::SniperTower => consts::towers::SNIPER_TOWER_ATTRIBUTES,
        };

        let tower_pos =
            U16Vec2::new(message.tower_pos.x, consts::MAP_SIZE_TILES[1] - message.tower_pos.y - 1);
        let bullet_emission_data: BulletEmissionData = BulletEmissionData {
            last_spawn_time_ms: Some(0),
            direction: Rot2::degrees(0.0),
            bullet_speed: attributes.bullet_speed,
            spawn_cooldown_ms: attributes.cooldown_ms,
        };
        let sprite: Sprite = Sprite {
            image: asset_server.load(attributes.sprite.s0_0_0),
            custom_size: attributes.size_tiles.into(),
            image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
            ..default()
        };
        let tower_data = TowerData {
            tower_type: message.tower_type,
            upgrade_level: UpgradeLevel::SmallSchlongKongStrong,
            effects: vec![],
        };

        Tower::spawn(
            &mut commands,
            sprite,
            tower_pos,
            tower_data,
            bullet_emission_data,
            &mut tower_range_map,
        );
    }
}

impl Tower {
    pub fn spawn(
        commands: &mut Commands, sprite: Sprite, tower_pos: U16Vec2, tower_data: TowerData,
        bullet_emission_data: BulletEmissionData, tower_range_map: &mut ResMut<TowerRangeMap>,
    ) {
        let entity = commands
            .spawn((
                Tower::default(),
                tower_data,
                sprite,
                bullet_emission_data,
                Transform::from_xyz(
                    tower_pos[0] as f32 + 0.5,
                    tower_pos[1] as f32 + 0.5,
                    consts::rendering_layers::ENTITY,
                ),
            ))
            .id();
        tower_range_map.add_range_rect(tower_pos, consts::TOWER_RANGE_TILES, entity);
    }
}

impl TowerRangeMap {
    pub fn clear(&mut self) {
        for x in &mut self.towers_in_range {
            x.clear();
        }
    }

    pub fn range_bounds(&self, pos_tiles: U16Vec2, range_tiles: u16) -> (U16Vec2, U16Vec2) {
        let center = pos_tiles;
        let range = U16Vec2::splat(range_tiles);
        let min = center.saturating_sub(range);
        let max = center.saturating_add(range).min(self.size.saturating_sub(U16Vec2::ONE));
        (min, max)
    }

    pub fn add_range_rect(&mut self, pos_tiles: U16Vec2, range_tiles: u16, entity: Entity) {
        let (min, max) = self.range_bounds(pos_tiles, range_tiles);
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                self.towers_in_range[(y * self.size.x + x) as usize].push(entity);
            }
        }
    }

    pub fn towers_in_range_at(&self, tile: U16Vec2) -> &[Entity] {
        let index = tile.y as usize * self.size.x as usize + tile.x as usize;
        &self.towers_in_range[index]
    }
}
