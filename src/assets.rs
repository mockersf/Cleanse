use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        AssetLoader::new(LoadingState::Assets)
            .continue_to_state(LoadingState::Done)
            .with_collection::<ProgressAssets>()
            .with_collection::<InGameAssets>()
            .build(app);
        app.add_state(LoadingState::Assets);
    }
}

#[derive(AssetCollection)]
pub struct ProgressAssets {
    #[asset(path = "sprites/disinfectant.png")]
    pub disinfectant: Handle<Image>,
    #[asset(path = "sprites/antibiotics.png")]
    pub antibiotics: Handle<Image>,
    #[asset(path = "sprites/vaccine.png")]
    pub vaccine: Handle<Image>,
    #[asset(path = "sprites/sanitation.png")]
    pub sanitation: Handle<Image>,
    #[asset(path = "sprites/personal-hygiene.png")]
    pub personal_hygiene: Handle<Image>,
    #[asset(path = "sprites/preventive-measures.png")]
    pub preventive_measures: Handle<Image>,
    #[asset(path = "sprites/sick-days.png")]
    pub sick_days: Handle<Image>,
    #[asset(path = "sprites/free-healthcare.png")]
    pub free_healthcare: Handle<Image>,
    #[asset(path = "sprites/parental-leave.png")]
    pub parental_leave: Handle<Image>,
    #[asset(path = "sprites/placeholder.png")]
    pub levelup_speed: Handle<Image>,
    #[asset(path = "sprites/placeholder.png")]
    pub levelup_attack: Handle<Image>,
    #[asset(path = "sprites/placeholder.png")]
    pub levelup_total_health: Handle<Image>,
    #[asset(path = "sprites/placeholder.png")]
    pub levelup_current_health: Handle<Image>,
    #[asset(path = "sprites/placeholder.png")]
    pub levelup_regen: Handle<Image>,
    #[asset(path = "sprites/placeholder.png")]
    pub levelup_dilatation: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct InGameAssets {
    #[asset(path = "sprites/immune-system.png")]
    pub immune_system: Handle<Image>,
    #[asset(path = "sprites/bacteria.png")]
    pub bacteria: Handle<Image>,
    #[asset(path = "sprites/virus.png")]
    pub virus: Handle<Image>,
    #[asset(path = "sprites/white-cell.png")]
    pub white_cell: Handle<Image>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum LoadingState {
    Assets,
    Done,
}
