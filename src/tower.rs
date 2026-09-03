use crate::bullets::BulletEmissionData;
use crate::consts::towers::TowerAttributes;
use crate::consts::{self};
use crate::coordinates::GridCoordinate;
use crate::game_cli::command_event_handling::PlaceTowerMessage;
use crate::player_suite::{PlayerSuiteResource, TransactionReturnStatus};
use crate::texture_packs::TexturePackSettings;
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

#[allow(unused)]
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

    Eitshtu,
    Acitonion,
    Strorm,
    Infernon,
    Icebyte,
    Goldt,
    Copprina,
}

impl Default for TowerRangeMap {
    fn default() -> Self {
        let size = U16Vec2::from_array(<[u16; 2]>::from(consts::MAP_SIZE_TILES));
        Self { size, towers_in_range: vec![Vec::new(); (size.x * size.y) as usize] }
    }
}

pub fn handle_tower_placing_events(
    mut messages: MessageReader<PlaceTowerMessage>, mut commands: Commands,
    asset_server: Res<AssetServer>, mut tower_range_map: ResMut<TowerRangeMap>,
    mut player_suite: ResMut<PlayerSuiteResource>, texture_pack_settings: Res<TexturePackSettings>,
) {
    for message in messages.read() {
        let attributes: TowerAttributes = match message.tower_type {
            TowerType::None => continue,
            TowerType::AssaultTower => consts::towers::ASSAULT_TROOP_ATTRIBUTES,
            TowerType::BoomTower => consts::towers::BOOM_TROOP_ATTRIBUTES,
            TowerType::GatlingTower => consts::towers::GATLING_TROOP_ATTRIBUTES,
            TowerType::SniperTower => consts::towers::SNIPER_TROOP_ATTRIBUTES,
            TowerType::Eitshtu => consts::towers::EITSHTU_ATTRIBUTES,
            _ => consts::towers::ASSAULT_TROOP_ATTRIBUTES,
        };

        if player_suite.perform_transaction(attributes.price)
            == TransactionReturnStatus::NotEnoughMoney
        {
            println!("Not enough money!");
            return;
        }
        println!("Performing transaction of {:?}", attributes.price);

        let tower_pos =
            GridCoordinate::new(message.tower_pos.position.x, message.tower_pos.position.y);
        let bullet_emission_data: BulletEmissionData = BulletEmissionData {
            last_spawn_time_ms: Some(0),
            direction: Rot2::degrees(0.0),
            bullet_speed: attributes.bullet_speed,
            spawn_cooldown_ms: attributes.cooldown_ms,
        };
        let sprite: Sprite = Sprite {
            image: asset_server
                .load(texture_pack_settings.get_asset_path(&attributes.sprites[0])),
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
        commands: &mut Commands, sprite: Sprite, tower_pos: GridCoordinate, tower_data: TowerData,
        bullet_emission_data: BulletEmissionData, tower_range_map: &mut ResMut<TowerRangeMap>,
    ) {
        let entity = commands
            .spawn((
                Tower::default(),
                tower_data,
                sprite,
                bullet_emission_data,
                Transform::from_xyz(
                    tower_pos.position.x as f32 + 0.5,
                    tower_pos.position.y as f32 + 0.5,
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
