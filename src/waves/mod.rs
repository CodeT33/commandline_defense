#![allow(unused)]

use crate::ecs_elements::messages::SpawnEnemy;
use crate::ecs_elements::resources::DebugSettings;
use crate::entities::enemies::EnemyType;
use crate::scheduling::IntervalTimer;
use bevy::prelude::{Local, MessageWriter, Res, Time};

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
        GameWaves { waves }
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

pub(crate) struct GameWaves {
    pub(crate) waves: Vec<Wave>,
}

pub(crate) fn handle_wave_enemy_spawns(
    mut enemy_spawns: MessageWriter<SpawnEnemy>, mut timer: Local<Option<IntervalTimer>>,
    time: Res<Time>, debug_settings: Res<DebugSettings>,
) {
    let t = timer
        .get_or_insert_with(|| IntervalTimer::new(debug_settings.enemy_spawn_interval_ms as u32));

    if t.get_interval_ms() as u64 != debug_settings.enemy_spawn_interval_ms {
        t.set_interval_ms(debug_settings.enemy_spawn_interval_ms as u32);
    }

    while let Some(tick_time) = t.tick_if_ready(&time) {
        enemy_spawns.write(SpawnEnemy { enemy_type: debug_settings.enemy_type, time: tick_time });
    }
}
