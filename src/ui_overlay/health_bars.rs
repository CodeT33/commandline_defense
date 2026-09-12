use crate::consts;
use crate::consts::ui::health_bars::{
    HEALTH_BAR_BACKGROUND_COLOR, HEALTH_BAR_FILL_COLOR, HEALTH_BAR_HEIGHT_TILES,
    HEALTH_BAR_OFFSET_TILES, HEALTH_BAR_WIDTH_TILES,
};
use crate::ecs_elements::components::{Bullet, HealthStats};
use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;

pub(crate) fn draw_health_bars(
    mut painter: ShapePainter, query: Query<(&Transform, &HealthStats), Without<Bullet>>,
) {
    for (transform, health) in &query {
        if health.current_health == health.max_health {
            continue;
        }

        let ratio = health.ratio();
        let base_pos = transform
            .translation
            .with_y(transform.translation.y - HEALTH_BAR_OFFSET_TILES)
            .with_z(consts::rendering_layers::HEALTH_BARS);

        painter.reset();
        painter.transform.translation = base_pos;
        painter.color = HEALTH_BAR_BACKGROUND_COLOR;
        painter.rect(Vec2::new(HEALTH_BAR_WIDTH_TILES, HEALTH_BAR_HEIGHT_TILES));

        if ratio > 0.0 {
            let fill_width = HEALTH_BAR_WIDTH_TILES * ratio;
            let x_offset = -HEALTH_BAR_WIDTH_TILES * (1.0 - ratio) / 2.0;

            painter.transform.translation = base_pos + Vec3::new(x_offset, 0.0, 0.1);
            painter.color = HEALTH_BAR_FILL_COLOR;
            painter.rect(Vec2::new(fill_width, HEALTH_BAR_HEIGHT_TILES));
        }
    }
}
