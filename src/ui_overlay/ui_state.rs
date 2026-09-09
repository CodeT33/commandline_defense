use crate::entities::enemies::EnemyType;
use crate::entities::tower::TowerType;

#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum UiState {
    Hidden,
    TowersList { filter: Option<String> },
    EnemiesList { filter: Option<String> },
    TowerInfo(TowerType),
    EnemyInfo(EnemyType),
    TowerUpgrades(TowerType),
}
