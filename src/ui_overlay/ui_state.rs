use crate::entities::enemies::EnemyType;
use crate::entities::tower::TowerType;

#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) enum UiState {
    #[default]
    Hidden,
    Menus,
    TowersList {
        selected: Option<TowerType>,
        further_details: TowerPage,
    },
    EnemiesList {
        selected: Option<EnemyType>,
        further_details: EnemyPage,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub(crate) enum TowerPage {
    #[default]
    None,
    Description,
    Upgrades,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub(crate) enum EnemyPage {
    #[default]
    None,
    Description,
}