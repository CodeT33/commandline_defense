pub mod system;

use crate::ecs_elements::resources::GameState;
use crate::entities::enemies::EnemyType;
use crate::entities::enemies::EnemyType::{
    Zapano, ZapanoBackend, ZapanoBody, ZapanoOfTheNight, ZapanoOfTheNightBackend,
    ZapanoOfTheNightBody,
};
use std::cmp::PartialEq;
use std::collections::VecDeque;

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

#[derive(Debug, Clone, Copy)]
pub(crate) enum WaveItem {
    Enemy { enemy_type: EnemyType, spawn_cooldown: u16, spawn_amount: u16 },
    Pause { duration_ms: u16 },
}

pub(crate) struct Wave {
    pub(crate) wave_items: Vec<WaveItem>,
    pub(crate) finishing_reward: u16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Task {
    WaitForResume,
    WaitDurationMs(u16),
    SpawnEnemy { enemy_type: EnemyType, cooldown: u16 },
    WaitForEnemiesDead,
    RoundFinished { finished_round: usize, reward: u16 },
}

pub(crate) struct GameWaves {
    tasks: VecDeque<Task>,
}

impl Default for GameState {
    fn default() -> Self {
        Self { waves: GameWaves::current_default() }
    }
}

impl GameWaves {
    pub(crate) fn build(waves: Vec<Wave>) -> Self {
        let mut actions = VecDeque::new();
        for (wave_idx, wave) in waves.into_iter().enumerate() {
            actions.push_back(Task::WaitForResume);
            for item in wave.wave_items {
                match item {
                    WaveItem::Enemy { enemy_type, spawn_cooldown, spawn_amount } => {
                        actions.extend(std::iter::repeat_n(
                            Task::SpawnEnemy { enemy_type, cooldown: spawn_cooldown },
                            spawn_amount as usize,
                        ));
                    },
                    WaveItem::Pause { duration_ms } => {
                        actions.push_back(Task::WaitDurationMs(duration_ms));
                    },
                }
            }
            actions.push_back(Task::WaitForEnemiesDead);
            actions.push_back(Task::RoundFinished {
                reward: wave.finishing_reward,
                finished_round: wave_idx,
            });
        }
        Self { tasks: actions }
    }

    #[allow(unused)]
    pub(crate) fn expand_zapanos(mut waves: Vec<Wave>) -> Vec<Wave> {
        for wave in &mut waves {
            wave.wave_items =
                wave.wave_items.iter().copied().flat_map(Self::expand_zapano_item).collect();
        }
        waves
    }

    fn expand_zapano_item(item: WaveItem) -> Vec<WaveItem> {
        let WaveItem::Enemy { enemy_type, spawn_amount, .. } = item else {
            return vec![item];
        };
        match enemy_type {
            Zapano => vec![
                WaveItem::new_enemy(Zapano, 200, 1),
                WaveItem::new_enemy(ZapanoBody, 200, spawn_amount),
                WaveItem::new_enemy(ZapanoBackend, 200, 1),
            ],
            ZapanoOfTheNight => vec![
                WaveItem::new_enemy(ZapanoOfTheNight, 200, 1),
                WaveItem::new_enemy(ZapanoOfTheNightBody, 200, spawn_amount),
                WaveItem::new_enemy(ZapanoOfTheNightBackend, 200, 1),
            ],
            _ => vec![item],
        }
    }

    pub(crate) fn current_task(&self) -> Option<Task> {
        self.tasks.front().copied()
    }

    pub(crate) fn resume(&mut self) {
        if self.tasks.front().copied() == Some(Task::WaitForResume) {
            self.tasks.pop_front();
        }
    }

    pub(crate) fn is_waiting_for_resume(&self) -> bool {
        self.tasks.front().copied() == Some(Task::WaitForResume)
    }

    pub(crate) fn pop_front(&mut self) {
        self.tasks.pop_front();
    }
}
