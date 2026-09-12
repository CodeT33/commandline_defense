use bevy::color::Color;
use egui::Color32;
use crate::tiers::{ValueTiers, ValueType};

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
            ValueTiers::S => {
                Color32::from_rgb(255, 255, 0)
            }
            ValueTiers::A => {
                Color32::from_rgb(255, 0, 255)
            }
            ValueTiers::B => {
                Color32::from_rgb(150, 150, 255)
            }
            ValueTiers::C => {
                Color32::from_rgb(100, 100, 255)
            }
            ValueTiers::D => {
                Color32::from_rgb(150, 255, 150)
            }
            ValueTiers::E => {
                Color32::from_rgb(100, 255, 100)
            }
            ValueTiers::F => {
                Color32::from_rgb(200, 200, 200)
            }
        }
    }
}
