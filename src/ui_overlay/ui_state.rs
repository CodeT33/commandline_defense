use crate::entities::enemies::EnemyType;
use crate::entities::tower::TowerType;

#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) enum UiState {
    #[default]
    Hidden,
    TowersList {
        selected: Option<TowerType>,
    },
    EnemiesList {
        selected: Option<EnemyType>,
    },
    TowerUpgrades(TowerType),
}
