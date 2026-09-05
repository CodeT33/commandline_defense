use crate::collision::CollisionPair;
use crate::coordinates::GridCoordinate;
use crate::entities::tower::TowerType;
use bevy::prelude::Message;

#[derive(Message)]
pub struct PlaceTowerMessage {
    pub tower_type: TowerType,
    pub tower_pos: GridCoordinate,
}

#[derive(Message)]
pub struct CollisionStarted(pub CollisionPair);

#[derive(Message)]
pub struct CollisionSustained(pub CollisionPair);

#[derive(Message)]
pub struct CollisionEnded(pub CollisionPair);

#[derive(Message, Debug)]
pub enum CommandEvent {
    Help,
    Select { tile: GridCoordinate },
    Place { tower_type: TowerType, tower_pos: GridCoordinate },
    Clear,
    Balance,
    ExitGame,
}
