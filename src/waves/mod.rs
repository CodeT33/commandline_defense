#![allow(unused)]

use crate::ecs_elements::messages::SpawnEnemy;
use crate::ecs_elements::resources::{DebugSettings, GameState};
use crate::entities::enemies::EnemyType;
use crate::scheduling::IntervalTimer;
use bevy::prelude::{Local, MessageWriter, Res, ResMut, Time};

impl WaveItem {
    pub(crate) fn new_enemy(
        enemy_type: EnemyType, spawn_cooldown: u16, spawn_amount: u16,
    ) -> WaveItem {
        WaveItem::Enemy { enemy_type, spawn_cooldown, spawn_amount }
    }
    pub(crate) fn new_pause(milliseconds: u16) -> WaveItem {
        WaveItem::Pause { milliseconds }
    }
}

impl Wave {
    pub(crate) fn new(wave_items: Vec<WaveItem>, finishing_reward: u16) -> Self {
        Wave { wave_items, finishing_reward }
    }
}

impl GameWaves {
    pub(crate) fn new(waves: Vec<Wave>) -> Self {
        GameWaves {
            waves,
            cursor: Some(WavesCursor { current_wave: 0, current_item: 0, current_enemy_idx: 0 }),
            paused: true,
        }
    }
}

pub(crate) enum WaveItem {
    Enemy { enemy_type: EnemyType, spawn_cooldown: u16, spawn_amount: u16 },
    Pause { milliseconds: u16 },
}

pub(crate) struct Wave {
    pub(crate) wave_items: Vec<WaveItem>,
    pub(crate) finishing_reward: u16,
}

pub(crate) struct WavesCursor {
    current_wave: usize,
    current_item: usize,
    current_enemy_idx: usize,
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
            Increment::WaitingForNextWave { reward, next_wave: wave_idx } => {
                timer.set_resume_immediately()
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
    WaitingForNextWave { next_wave: usize, reward: Option<u16> },
}

impl GameWaves {
    /// Returns the WaveItem and the optional reward
    pub(crate) fn increment_cursor(&mut self) -> Result<Increment, ()> {
        todo!()
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
