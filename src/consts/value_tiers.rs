use crate::tiers::{ValueTiers, ValueType};
use egui::Color32;

impl ValueTiers {
    #[allow(unused)]
    pub(crate) fn get_value(&self, value_type: ValueType) -> f32 {
        match value_type {
            // Tower
            ValueType::TowerRange => match self {
                ValueTiers::S => 16.0,
                ValueTiers::A => 8.0,
                ValueTiers::B => 5.0,
                ValueTiers::C => 4.0,
                ValueTiers::D => 3.0,
                ValueTiers::E => 2.0,
                ValueTiers::F => 1.0,
            },
            ValueType::TowerReloadSpeed => match self {
                ValueTiers::S => 250.0,
                ValueTiers::A => 500.0,
                ValueTiers::B => 750.0,
                ValueTiers::C => 1000.0,
                ValueTiers::D => 2500.0,
                ValueTiers::E => 3500.0,
                ValueTiers::F => 5000.0,
            },
            ValueType::TowerAreaDamage => match self {
                ValueTiers::S => 1.0,
                ValueTiers::A => 1.0,
                ValueTiers::B => 1.0,
                ValueTiers::C => 1.0,
                ValueTiers::D => 1.0,
                ValueTiers::E => 1.0,
                ValueTiers::F => 1.0,
            },

            // Bullet
            ValueType::BulletSpeed => match self {
                ValueTiers::S => 20.0,
                ValueTiers::A => 14.0,
                ValueTiers::B => 10.0,
                ValueTiers::C => 7.0,
                ValueTiers::D => 5.0,
                ValueTiers::E => 3.0,
                ValueTiers::F => 1.0,
            },
            ValueType::BulletDamage => match self {
                ValueTiers::S => 25.0,
                ValueTiers::A => 10.0,
                ValueTiers::B => 4.0,
                ValueTiers::C => 2.0,
                ValueTiers::D => 1.5,
                ValueTiers::E => 1.0,
                ValueTiers::F => 0.5,
            },
            ValueType::BulletPierce => match self {
                ValueTiers::S => 10.0,
                ValueTiers::A => 8.0,
                ValueTiers::B => 6.0,
                ValueTiers::C => 4.0,
                ValueTiers::D => 3.0,
                ValueTiers::E => 2.0,
                ValueTiers::F => 1.0,
            },

            // Enemy
            ValueType::EnemyMovementSpeed => match self {
                ValueTiers::S => 10.0,
                ValueTiers::A => 7.5,
                ValueTiers::B => 5.0,
                ValueTiers::C => 3.0,
                ValueTiers::D => 1.5,
                ValueTiers::E => 1.0,
                ValueTiers::F => 0.5,
            },
            ValueType::EnemyHealth => match self {
                ValueTiers::S => 100.0,
                ValueTiers::A => 25.0,
                ValueTiers::B => 10.0,
                ValueTiers::C => 7.0,
                ValueTiers::D => 5.0,
                ValueTiers::E => 2.0,
                ValueTiers::F => 1.0,
            },
            ValueType::EnemyPlayerHealthPenalty => match self {
                ValueTiers::S => 25.0,
                ValueTiers::A => 10.0,
                ValueTiers::B => 5.0,
                ValueTiers::C => 3.0,
                ValueTiers::D => 2.0,
                ValueTiers::E => 1.0,
                ValueTiers::F => 0.0,
            },
        }
    }

    pub(crate) fn get_tier_color(&self) -> Color32 {
        match self {
            ValueTiers::S => Color32::from_rgb(0x79, 0xff, 0xf4), // aqua
            ValueTiers::A => Color32::from_rgb(0xfa, 0xda, 0x4b), // yellow
            ValueTiers::B => Color32::from_rgb(0xf7, 0x6a, 0x12), // orange
            ValueTiers::C => Color32::from_rgb(0xa7, 0x45, 0xce), // purple
            ValueTiers::D => Color32::from_rgb(0x4e, 0x5a, 0xfe), // blue
            ValueTiers::E => Color32::from_rgb(0x1f, 0xdc, 0x1d), // green
            ValueTiers::F => Color32::from_rgb(0xbd, 0xbd, 0xb5), // grey
        }
    }
}
