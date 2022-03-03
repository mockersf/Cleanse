use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        AssetLoader::new(LoadingState::Assets)
            .continue_to_state(LoadingState::Done)
            .with_collection::<ProgressAssets>()
            .with_collection::<InGameAssets>()
            .with_collection::<LevelUpAssets>()
            .build(app);
        app.add_state(LoadingState::Assets);
    }
}

#[derive(AssetCollection)]
pub struct ProgressAssets {
    #[asset(path = "sprites/pr-disinfectant.png")]
    pub disinfectant: Handle<Image>,
    #[asset(path = "sprites/pr-antibiotics.png")]
    pub antibiotics: Handle<Image>,
    #[asset(path = "sprites/pr-vaccine.png")]
    pub vaccine: Handle<Image>,
    #[asset(path = "sprites/pr-sanitation.png")]
    pub sanitation: Handle<Image>,
    #[asset(path = "sprites/pr-personal-hygiene.png")]
    pub personal_hygiene: Handle<Image>,
    #[asset(path = "sprites/pr-preventive-measures.png")]
    pub preventive_measures: Handle<Image>,
    #[asset(path = "sprites/pr-sick-days.png")]
    pub sick_days: Handle<Image>,
    #[asset(path = "sprites/pr-free-healthcare.png")]
    pub free_healthcare: Handle<Image>,
    #[asset(path = "sprites/pr-parental-leave.png")]
    pub parental_leave: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct InGameAssets {
    #[asset(path = "sprites/immune-system.png")]
    pub immune_system: Handle<Image>,
    #[asset(path = "sprites/bacteria.png")]
    pub bacteria: Handle<Image>,
    #[asset(path = "sprites/virus.png")]
    pub virus: Handle<Image>,
    #[asset(path = "sprites/cancer.png")]
    pub cancer: Handle<Image>,
    #[asset(path = "sprites/white-cell.png")]
    pub white_cell: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct LevelUpAssets {
    #[asset(path = "sprites/lvlup-attack.png")]
    pub attack: Handle<Image>,
    #[asset(path = "sprites/lvlup-speed.png")]
    pub speed: Handle<Image>,
    #[asset(path = "sprites/lvlup-total-health.png")]
    pub total_health: Handle<Image>,
    #[asset(path = "sprites/lvlup-current-health.png")]
    pub current_health: Handle<Image>,
    #[asset(path = "sprites/lvlup-regen.png")]
    pub regen: Handle<Image>,
    #[asset(path = "sprites/lvlup-dilatation.png")]
    pub dilatation: Handle<Image>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum LoadingState {
    Assets,
    Done,
}
