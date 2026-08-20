use crate::bullets::BulletEmissionData;
use crate::consts;
use crate::game_cli::command_event_handling::PlaceTowerMessage;
use avian2d::parry::glamx::U16Vec2;
use bevy::asset::AssetServer;
use bevy::ecs::entity::EntityHashSet;
use bevy::prelude::{
    Commands, Component, Entity, MessageReader, Res, ResMut, Resource, Sprite, SpriteImageMode,
    SpriteScalingMode, Transform, default,
};

#[derive(Component, Default)]
pub struct Tower {
    pub enemies_in_range: EntityHashSet,
}

enum UpgradeLevel {
    SmallSchlongKongStrong,
    SchlongMediumIchKackeImTediRum,
    SchlongusLongus,
    MaximusBigschlongus,
    UnbreakableSnake,
}

enum Effect {
    BallBoost,
    BigBirbMode,
}

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
        let tower_pos = U16Vec2::new(message.tower_pos.x, consts::MAP_SIZE_TILES[1] - message.tower_pos.y - 1);
        let tower_type = message.tower_type;

        println!("Balls {:?} at {:?}", tower_type, tower_pos);

        Tower::spawn(
            &mut commands,
            &asset_server,
            tower_pos,
            BulletEmissionData::default(),
            &mut tower_range_map,
        );
    }
}

impl Tower {
    pub fn spawn(
        commands: &mut Commands, asset_server: &Res<AssetServer>, tower_pos: U16Vec2,
        data: BulletEmissionData, tower_range_map: &mut ResMut<TowerRangeMap>,
    ) {
        let entity = commands
            .spawn((
                Tower::default(),
                Sprite {
                    image: asset_server.load(consts::assets::resource_packs::base_pack::towers::assault_tower::S0_0_0),
                    custom_size: consts::TOWER_SIZE_TILES.into(),
                    image_mode: SpriteImageMode::Scale(SpriteScalingMode::FitCenter),
                    ..default()
                },
                data,
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
