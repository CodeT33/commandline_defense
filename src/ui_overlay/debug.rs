use crate::collision::ColliderShape;
use bevy::math::Isometry2d;
use bevy::prelude::{Color, Gizmos, Query, Transform};

pub fn draw_bounding_boxes(q: Query<(&ColliderShape, &Transform)>, mut gizmos: Gizmos) {
    for (shape, transform) in q.iter() {
        match shape {
            ColliderShape::Rectangle(rect) => gizmos.rect_2d(
                Isometry2d::from_translation(transform.translation.truncate()),
                rect.half_size * 2.0,
                Color::hsv(0.3, 1.0, 1.0),
            ),
            ColliderShape::Circle(circle) => {
                gizmos.circle_2d(
                    Isometry2d::from_translation(transform.translation.truncate()),
                    circle.radius,
                    Color::hsv(0.3, 1.0, 1.0),
                );
            },
        }
    }
}
