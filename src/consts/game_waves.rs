use crate::entities::enemies::EnemyType::{
    Mausmeister, Rocher, RocherOfTheNight, Zapano, ZapanoOfTheNight,
};
use crate::waves::{GameWaves, Wave, WaveItem};

impl GameWaves {
    #[allow(unused)]
    pub(crate) fn current_default() -> GameWaves {
        GameWaves::build(Self::expand_zapanos(vec![
            Wave::new(vec![WaveItem::new_enemy(Zapano, 0, 1)], 50),
            Wave::new(
                vec![
                    WaveItem::new_enemy(Zapano, 0, 5),
                    WaveItem::new_pause(2000),
                    WaveItem::new_enemy(Rocher, 1000, 1),
                ],
                100,
            ),
            Wave::new(
                vec![
                    WaveItem::new_enemy(Zapano, 0, 3),
                    WaveItem::new_enemy(Rocher, 3000, 1),
                    WaveItem::new_enemy(Zapano, 0, 20),
                    WaveItem::new_pause(2000),
                    WaveItem::new_enemy(Rocher, 2000, 3),
                ],
                100,
            ),
            Wave::new(
                vec![
                    WaveItem::new_enemy(Zapano, 0, 1),
                    WaveItem::new_pause(2000),
                    WaveItem::new_enemy(Zapano, 0, 10),
                ],
                200,
            ),
            Wave::new(
                vec![
                    WaveItem::new_enemy(Rocher, 1000, 5),
                    WaveItem::new_enemy(Zapano, 0, 5),
                    WaveItem::new_pause(2000),
                    WaveItem::new_enemy(Rocher, 200, 3),
                ],
                200,
            ),
            Wave::new(
                vec![
                    WaveItem::new_enemy(Rocher, 1000, 10),
                    WaveItem::new_enemy(Zapano, 0, 10),
                    WaveItem::new_pause(2000),
                    WaveItem::new_enemy(Zapano, 0, 10),
                    WaveItem::new_pause(2000),
                    WaveItem::new_enemy(ZapanoOfTheNight, 0, 5),
                ],
                200,
            ),
            Wave::new(
                vec![
                    WaveItem::new_enemy(ZapanoOfTheNight, 0, 5),
                    WaveItem::new_pause(5000),
                    WaveItem::new_enemy(ZapanoOfTheNight, 0, 10),
                ],
                200,
            ),
            Wave::new(vec![WaveItem::new_enemy(RocherOfTheNight, 0, 1)], 200),
            Wave::new(
                vec![
                    WaveItem::new_enemy(RocherOfTheNight, 500, 3),
                    WaveItem::new_enemy(ZapanoOfTheNight, 0, 20),
                ],
                250,
            ),
            Wave::new(vec![WaveItem::new_enemy(Mausmeister, 100, 3)], 25000),
            Wave::new(
                vec![
                    WaveItem::new_enemy(RocherOfTheNight, 500, 3),
                    WaveItem::new_enemy(ZapanoOfTheNight, 0, 20),
                ],
                250,
            ),
        ]))
    }
}
