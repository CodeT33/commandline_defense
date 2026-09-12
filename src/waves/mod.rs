#![allow(unused)]

pub mod cursor;

use crate::ecs_elements::messages::SpawnEnemy;
use crate::ecs_elements::resources::{DebugSettings, GameState};
use crate::entities::enemies::EnemyType;
use crate::scheduling::IntervalTimer;
use bevy::prelude::{Local, MessageWriter, Res, ResMut, Time};
use cursor::WavesCursor;

impl WaveItem {
    pub(crate) fn new_enemy(
        enemy_type: EnemyType, spawn_cooldown: u16, spawn_amount: u16,
    ) -> WaveItem {
        WaveItem::Enemy { enemy_type, spawn_cooldown, spawn_amount }
    }
    pub(crate) fn new_pause(milliseconds: u16) -> WaveItem {
        WaveItem::Pause { duration_ms: milliseconds }
    }
}

impl Wave {
    pub(crate) fn new(wave_items: Vec<WaveItem>, finishing_reward: u16) -> Self {
        Wave { wave_items, finishing_reward }
    }
}

impl GameWaves {
    pub(crate) fn new(waves: Vec<Wave>) -> Self {
        GameWaves { waves, cursor: Some(WavesCursor::default()), paused: true }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum WaveItem {
    Enemy { enemy_type: EnemyType, spawn_cooldown: u16, spawn_amount: u16 },
    Pause { duration_ms: u16 },
}

pub(crate) struct Wave {
    pub(crate) wave_items: Vec<WaveItem>,
    pub(crate) finishing_reward: u16,
}

pub(crate) struct GameWaves {
    waves: Vec<Wave>,
    cursor: Option<WavesCursor>,
    paused: bool,
}

pub(crate) fn enemy_wave_handler(
    mut enemy_spawns: MessageWriter<SpawnEnemy>, mut local_timer: Local<Option<IntervalTimer>>,
    time: Res<Time>, debug_settings: Res<DebugSettings>, mut game_state: ResMut<GameState>,
) {
    let timer = local_timer.get_or_insert_with(|| IntervalTimer::new(0));

    if game_state.waves.is_paused() || game_state.waves.is_finished() {
        return;
    }

    while let Some(tick_time) = timer.tick_if_ready(&time) {
        let Ok(increment) = game_state.waves.increment_cursor() else {
            println!("Hey you finished the game. Congratulations!");
            return;
        };
        match increment {
            Increment::GameRunning { enemy, cooldown } => {
                timer.set_interval_ms(cooldown as u32);

                if let Some(enemy_type) = enemy {
                    enemy_spawns.write(SpawnEnemy { enemy_type, time: tick_time });
                }
            },
            Increment::WaitingForRoundToFinish => {
                timer.set_resume_immediately();
                break;
            },
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self { waves: GameWaves::current_default() }
    }
}

pub(crate) enum Increment {
    GameRunning { enemy: Option<EnemyType>, cooldown: u16 },
    WaitingForRoundToFinish,
}

impl GameWaves {
    /// Returns the WaveItem and the optional reward.
    /// Resumes the game.
    pub(crate) fn increment_cursor(&mut self) -> Result<Increment, ()> {
        if let Some(cursor) = &mut self.cursor {
            let Some(current_wave) = self.waves.get(cursor.wave_idx()) else {
                self.cursor = None;
                return Err(());
            };
            let Some(&wave_item) = current_wave.wave_items.get(cursor.item_idx()) else {
                cursor.increment_wave();
                self.paused = true;

                return Ok(Increment::WaitingForRoundToFinish);
            };
            match wave_item {
                WaveItem::Pause { duration_ms } => {
                    cursor.increment_item();
                    Ok(Increment::GameRunning { cooldown: duration_ms, enemy: None })
                },
                WaveItem::Enemy { enemy_type, spawn_cooldown, spawn_amount } => {
                    if cursor.item_inner_idx() + 1 >= spawn_amount as usize {
                        cursor.increment_item()
                    } else {
                        cursor.increment_item_inner()
                    }
                    Ok(Increment::GameRunning {
                        cooldown: spawn_cooldown,
                        enemy: enemy_type.into(),
                    })
                },
            }
        } else {
            Err(())
        }
    }

    pub(crate) fn is_paused(&self) -> bool {
        self.paused
    }
    pub(crate) fn is_finished(&self) -> bool {
        self.cursor.is_none()
    }
    pub(crate) fn resume(&mut self) {
        self.paused = false;
    }
}
