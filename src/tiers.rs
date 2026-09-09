#[allow(unused)]
pub(crate) enum ValueType {
    /// In `tiles`
    Range,
    /// In `tiles per second`
    BulletSpeed,
    /// In `tiles per second`
    MovementSpeed,
    /// In `shots per second`
    ReloadSpeed,
    /// In `lives`
    BulletDamage,
    /// In `tiles` which measures the diameter
    AreaDamage,
}

#[allow(unused)]
pub(crate) enum ValueTiers {
    S,
    A,
    B,
    C,
    D,
    E,
    F,
}
