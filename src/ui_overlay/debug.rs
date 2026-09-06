use crate::consts;
use crate::ecs_elements::components::{Bullet, ColliderShape, Enemy, Tower};
use bevy::math::Isometry2d;
use bevy::prelude::{Camera2d, Gizmos, Local, Query, Transform, With, Without};
use std::fs;
use std::process::exit;

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

fn dump_positions<'a, T>(query: T) -> String
where
    T: Iterator<Item = &'a Transform>,
{
    let mut positions: Vec<(f32, f32)> =
        query.map(|t| (t.translation.x, t.translation.y)).collect();
    positions.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
    positions.iter().map(|(x, y)| format!("{x},{y}")).collect::<Vec<_>>().join(" ")
}

#[allow(clippy::type_complexity)]
pub fn log_entity_positions(
    mut tick: Local<u64>, mut lines: Local<Vec<String>>, enemies: Query<&Transform, With<Enemy>>,
    bullets: Query<&Transform, With<Bullet>>, towers: Query<&Transform, With<Tower>>,
    others: Query<&Transform, (Without<Enemy>, Without<Bullet>, Without<Tower>, Without<Camera2d>)>,
) {
    *tick += 1;
    lines.push(format!(
        "tick {}: B[{}] E[{}] T[{}] U[{}]",
        *tick,
        dump_positions(bullets.iter()),
        dump_positions(enemies.iter()),
        dump_positions(towers.iter()),
        dump_positions(others.iter()),
    ));

    if *tick == consts::LOG_DURATION_SECS * consts::PHYSICS_FRAME_RATE as u64 {
        fs::create_dir_all("logs").expect("log directory must be creatable");
        let next = fs::read_dir("logs")
            .expect("log directory must be readable")
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                entry.file_name().to_str()?.strip_suffix(".log")?.parse::<u64>().ok()
            })
            .max()
            .unwrap_or(0)
            + 1;
        fs::write(format!("logs/{next}.log"), lines.join("\n")).expect("log file must be writable");
        exit(0);
    }
}
