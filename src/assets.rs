use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        AssetLoader::new(LoadingState::Assets)
            .continue_to_state(LoadingState::Done)
            .with_collection::<RawAssets>()
            .build(app);
        app.add_state(LoadingState::Assets);
    }
}

#[derive(AssetCollection)]
struct RawAssets {}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum LoadingState {
    Assets,
    Done,
}
