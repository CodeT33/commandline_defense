use crate::consts;
use crate::coordinates::GridCoordinate;
use crate::ecs_elements::components::{Bullet, Enemy, TowerData};
use crate::ecs_elements::messages::CommandEvent;
use crate::entities::tower::TowerType;
use bevy::app::App;
use bevy::camera::Camera2d;
use bevy::prelude::{
    FixedLast, Local, MessageWriter, Plugin, Query, Startup, Transform, With, Without,
};
use std::fs;
use std::process::exit;

pub(crate) struct DeterminismHarnessPlugin;

impl Plugin for DeterminismHarnessPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, determinism_run_setup);
        app.add_systems(FixedLast, log_entity_positions);
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
fn log_entity_positions(
    mut tick: Local<u64>, mut lines: Local<Vec<String>>, enemies: Query<&Transform, With<Enemy>>,
    bullets: Query<&Transform, With<Bullet>>, towers: Query<&Transform, With<TowerData>>,
    others: Query<
        &Transform,
        (Without<Enemy>, Without<Bullet>, Without<TowerData>, Without<Camera2d>),
    >,
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

fn determinism_run_setup(mut commands_writer: MessageWriter<CommandEvent>) {
    for i in 0..10 {
        commands_writer.write(CommandEvent::Place {
            tower_type: TowerType::AssaultTower,
            tower_pos: GridCoordinate::new(i, i),
        });
    }
}
