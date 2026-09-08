use crate::consts;
use crate::ecs_elements::components::ColliderShape;
use crate::ecs_elements::resources::DebugSettings;
use bevy::math::Isometry2d;
use bevy::prelude::{Gizmos, Query, Real, Res, ResMut, Time, Transform, Virtual};

pub(crate) fn draw_bounding_boxes(
    q: Query<(&ColliderShape, &Transform)>, mut gizmos: Gizmos, time: Res<Time<Real>>,
) {
    let toggle_index =
        (time.elapsed_secs() / consts::ui::BOUNDING_BOX_DEBUG_COLOR_TOGGLE_SECS) as u64;
    let color = if toggle_index.is_multiple_of(2) {
        consts::ui::BOUNDING_BOX_DEBUG_COLOR
    } else {
        consts::ui::BOUNDING_BOX_DEBUG_COLOR_ALT
    };
    for (shape, transform) in q.iter() {
        match shape {
            ColliderShape::Rectangle(rect) => gizmos.rect_2d(
                Isometry2d::from_translation(transform.translation.truncate()),
                rect.half_size * 2.0,
                color,
            ),
            ColliderShape::Circle(circle) => {
                gizmos.circle_2d(
                    Isometry2d::from_translation(transform.translation.truncate()),
                    circle.radius,
                    color,
                );
            },
        }
    }
}

pub(crate) fn set_simulation_speed(
    mut time: ResMut<Time<Virtual>>, debug_settings: Res<DebugSettings>,
) {
    time.set_relative_speed(debug_settings.sim_speed);
}
