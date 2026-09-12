use crate::ecs_elements::components::Enemy;
use crate::ecs_elements::messages::SpawnEnemy;
use crate::ecs_elements::resources::{DebugSettings, GameState, PlayerSuiteResource};
use crate::scheduling::IntervalTimer;
use crate::waves::Task;
use bevy::prelude::{Local, MessageWriter, Query, Res, ResMut, Time, With};

pub(crate) fn enemy_wave_handler(
    mut enemy_spawns: MessageWriter<SpawnEnemy>, mut local_timer: Local<Option<IntervalTimer>>,
    time: Res<Time>, mut game_state: ResMut<GameState>, enemies: Query<(), With<Enemy>>,
    mut player_suite: ResMut<PlayerSuiteResource>, mut debug_settings: ResMut<DebugSettings>,
) {
    let timer = local_timer.get_or_insert_with(|| IntervalTimer::new(0));

    while let Some(tick_time) = timer.tick_if_ready(&time) {
        let Some(task) = game_state.waves.current_task() else {
            return;
        };
        match task {
            Task::WaitForResume => {
                debug_settings.paused = true;
                timer.set_resume_immediately();
                return;
            },
            Task::WaitDurationMs(sleep_ms) => {
                game_state.waves.pop_front();
                timer.set_interval_ms(sleep_ms as u32);
            },
            Task::SpawnEnemy { enemy_type, cooldown } => {
                game_state.waves.pop_front();
                timer.set_interval_ms(cooldown as u32);
                enemy_spawns.write(SpawnEnemy { enemy_type, time: tick_time });
            },
            Task::WaitForEnemiesDead => {
                timer.set_resume_immediately();
                if enemies.count() > 0 {
                    return;
                }
                game_state.waves.pop_front();
            },
            Task::RoundFinished { reward, finished_round } => {
                timer.set_resume_immediately();
                game_state.waves.pop_front();
                player_suite.next_wave = (finished_round+1) as u16;
                println!("Finished Round {} with reward {}", finished_round, reward);
                player_suite.money += reward;
            },
        }
    }
}
