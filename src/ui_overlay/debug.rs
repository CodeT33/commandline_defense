use crate::consts;
use crate::ecs_elements::components::ColliderShape;
use bevy::math::Isometry2d;
use bevy::prelude::{Gizmos, Query, Transform};

pub fn draw_bounding_boxes(q: Query<(&ColliderShape, &Transform)>, mut gizmos: Gizmos) {
    for (shape, transform) in q.iter() {
        match shape {
            ColliderShape::Rectangle(rect) => gizmos.rect_2d(
                Isometry2d::from_translation(transform.translation.truncate()),
                rect.half_size * 2.0,
                consts::ui::BOUNDING_BOX_DEBUG_COLOR,
            ),
            ColliderShape::Circle(circle) => {
                gizmos.circle_2d(
                    Isometry2d::from_translation(transform.translation.truncate()),
                    circle.radius,
                    consts::ui::BOUNDING_BOX_DEBUG_COLOR,
                );
            },
        }
    }
}
