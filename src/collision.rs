use crate::ecs_elements::components::{ColliderShape, ColliderTypeA, ColliderTypeB};
use crate::ecs_elements::messages::{CollisionEnded, CollisionStarted, CollisionSustained};
use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};
use bevy::prelude::{
    Circle, Entity, Local, MessageWriter, Query, Rectangle, Transform, Vec2, With,
};
use std::collections::HashSet;

enum Collider {
    Aabb(Aabb2d),
    Circle(BoundingCircle),
}

pub struct CollisionPair {
    pub type_a: Entity,
    pub type_b: Entity,
}

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
    type_a: Query<(Entity, &ColliderShape, &Transform), With<ColliderTypeA>>,
    type_b: Query<(Entity, &ColliderShape, &Transform), With<ColliderTypeB>>,
    mut old_collisions: Local<HashSet<(Entity, Entity)>>,
    mut new_collisions: Local<HashSet<(Entity, Entity)>>,
    mut started_writer: MessageWriter<CollisionStarted>,
    mut sustained_writer: MessageWriter<CollisionSustained>,
    mut ended_writer: MessageWriter<CollisionEnded>,
) {
    new_collisions.clear();
    for (entity_a, col_a, tf_a) in type_a.iter() {
        let col_a = col_a.to_collider(tf_a.translation.truncate());

        for (entity_b, col_b, tf_b) in type_b.iter() {
            let col_b = col_b.to_collider(tf_b.translation.truncate());

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
