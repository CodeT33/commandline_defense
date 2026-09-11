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
