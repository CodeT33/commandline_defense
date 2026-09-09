use crate::coordinates::GridCoordinate;
use crate::ecs_elements::messages::CommandEvent;
use crate::entities::tower::TowerType;
use bevy::prelude::{App, Local, MessageWriter, Plugin, Res, Time, Timer, TimerMode, Update};

/// Delay before the demo command fires.
pub(crate) const DEMO_TRIGGER_DELAY_SECS: f32 = 15.0;

pub(crate) struct DemoCommandPlugin;

impl Plugin for DemoCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, run_demo_command);
    }
}

fn run_demo_command(
    mut timer: Local<Option<Timer>>, mut fired: Local<bool>, time: Res<Time>,
    mut events: MessageWriter<CommandEvent>,
) {
    if timer.is_none() {
        *timer = Some(Timer::from_seconds(DEMO_TRIGGER_DELAY_SECS, TimerMode::Once));
    }

    let timer = timer.as_mut().expect("timer is set on first frame by construction");
    timer.tick(time.delta());

    if *fired {
        return;
    }

    if timer.is_finished() {
        *fired = true;
        events.write(CommandEvent::Place {
            tower_type: TowerType::GatlingTower,
            tower_pos: GridCoordinate::new(26, 0),
        });
    }
}
