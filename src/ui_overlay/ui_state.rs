use crate::entities::enemies::EnemyType;
use crate::entities::tower::TowerType;

#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) enum UiState {
    #[default]
    Hidden,
    TowersList {
        filter: Option<String>,
    },
    EnemiesList {
        filter: Option<String>,
    },
    TowerInfo(TowerType),
    EnemyInfo(EnemyType),
    TowerUpgrades(TowerType),
}
