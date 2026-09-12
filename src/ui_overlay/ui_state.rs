use crate::entities::enemies::EnemyType;
use crate::entities::tower::TowerType;
use clap::ValueEnum;

#[allow(unused)]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) enum UiState {
    #[default]
    Hidden,
    Menus,
    TowersList {
        selected: Option<TowerType>,
        further_details: Option<TowerPage>,
    },
    EnemiesList {
        selected: Option<EnemyType>,
        further_details: Option<EnemyPage>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub(crate) enum TowerPage {
    Description,
    Upgrades,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub(crate) enum EnemyPage {
    Description,
}
