use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::{
    Circle, Component, Entity, Local, Message, MessageWriter, Query, Rectangle, Transform, Vec2,
};
use std::collections::HashSet;

#[derive(Copy, Clone)]
pub enum ColliderShape {
    Rectangle(Rectangle),
    Circle(Circle),
}

enum Collider {
    Aabb(Aabb2d),
    Circle(BoundingCircle),
}

#[derive(Component)]
pub struct ColliderTypeA(pub ColliderShape);

#[derive(Component)]
pub struct ColliderTypeB(pub ColliderShape);

pub struct CollisionPair {
    pub type_a: Entity,
    pub type_b: Entity,
}

#[derive(Message)]
pub struct CollisionStarted(pub CollisionPair);

#[derive(Message)]
pub struct CollisionSustained(pub CollisionPair);

#[derive(Message)]
pub struct CollisionEnded(pub CollisionPair);

impl CollisionPair {
    pub fn new(type_a: Entity, type_b: Entity) -> Self {
        Self { type_a, type_b }
    }
}

impl ColliderShape {
    pub fn circle(radius: f32) -> Self {
        ColliderShape::Circle(Circle::new(radius))
    }

    pub fn rect(size: Vec2) -> Self {
        ColliderShape::Rectangle(Rectangle::from_size(size))
    }

    fn to_collider(self, position: Vec2) -> Collider {
        match self {
            ColliderShape::Rectangle(rect) => Collider::Aabb(Aabb2d::new(position, rect.half_size)),
            ColliderShape::Circle(circle) => {
                Collider::Circle(BoundingCircle::new(position, circle.radius))
            },
        }
    }
}

impl Collider {
    #[inline]
    fn intersects(&self, other: &Self) -> bool {
        match (self, other) {
            (Collider::Aabb(a), Collider::Aabb(b)) => a.intersects(b),
            (Collider::Circle(a), Collider::Circle(b)) => a.intersects(b),
            (Collider::Circle(a), Collider::Aabb(b)) => a.intersects(b),
            (Collider::Aabb(a), Collider::Circle(b)) => a.intersects(b),
        }
    }
}

pub fn calculate_collisions(
    type_a: Query<(Entity, &ColliderTypeA, &Transform)>,
    type_b: Query<(Entity, &ColliderTypeB, &Transform)>,
    mut old_collisions: Local<HashSet<(Entity, Entity)>>,
    mut new_collisions: Local<HashSet<(Entity, Entity)>>,
    mut started_writer: MessageWriter<CollisionStarted>,
    mut sustained_writer: MessageWriter<CollisionSustained>,
    mut ended_writer: MessageWriter<CollisionEnded>,
) {
    new_collisions.clear();
    for (entity_a, type_a, tf_a) in type_a.iter() {
        let col_a = type_a.0.to_collider(tf_a.translation.truncate());

        for (entity_b, type_b, tf_b) in type_b.iter() {
            let col_b = type_b.0.to_collider(tf_b.translation.truncate());

            if col_a.intersects(&col_b) {
                new_collisions.insert((entity_a, entity_b));
            }
        }
    }

    for &pair in &new_collisions {
        if old_collisions.contains(&pair) {
            sustained_writer.write(CollisionSustained(CollisionPair::new(pair.0, pair.1)));
            old_collisions.remove(&pair);
        } else {
            started_writer.write(CollisionStarted(CollisionPair::new(pair.0, pair.1)));
        }
    }
    for &pair in &old_collisions {
        ended_writer.write(CollisionEnded(CollisionPair::new(pair.0, pair.1)));
    }

    std::mem::swap(&mut *old_collisions, &mut *new_collisions);
}
