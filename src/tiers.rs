#[allow(unused)]
pub(crate) enum ValueType {
    // Tower
    /// In `tiles`
    TowerRange,
    /// In `shots per second`
    TowerReloadSpeed,
    /// In `tiles` which measures the diameter
    TowerAreaDamage,

    // Bullet
    /// In `tiles per second`
    BulletSpeed,
    /// In `lives`
    BulletDamage,
    /// In `enemies to pass through`
    BulletPierce,

    // Enemy
    /// In `tiles per second`
    EnemyMovementSpeed,
    /// In `lives`
    EnemyHealth,
    /// In `player lives`
    EnemyPlayerHealthPenalty,
}

#[allow(unused)]
#[derive(Clone, Copy, Debug)]
pub(crate) enum ValueTiers {
    S,
    A,
    B,
    C,
    D,
    E,
    F,
}

#[allow(unused)]
#[derive(Copy, Clone, Debug)]
pub enum Formatting {
    Roman,
    Decimal,
    Letters,
}

impl ValueTiers {
    pub fn format_in(self, format: Formatting) -> String {
        match format {
            Formatting::Roman => match self {
                ValueTiers::S => "VII".to_string(),
                ValueTiers::A => "VI ".to_string(),
                ValueTiers::B => "V  ".to_string(),
                ValueTiers::C => "IV ".to_string(),
                ValueTiers::D => "III ".to_string(),
                ValueTiers::E => "II  ".to_string(),
                ValueTiers::F => "I   ".to_string(),
            },
            Formatting::Decimal => match self {
                ValueTiers::S => "7".to_string(),
                ValueTiers::A => "6".to_string(),
                ValueTiers::B => "5".to_string(),
                ValueTiers::C => "4".to_string(),
                ValueTiers::D => "3".to_string(),
                ValueTiers::E => "2".to_string(),
                ValueTiers::F => "1".to_string(),
            },
            Formatting::Letters => format!("{:?}", self),
        }
    }
}
