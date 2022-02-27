use bevy::prelude::*;

use crate::{tear_down, GameState};

use self::{
    host::{HostState, Risks, Status},
    immune_system::ImmuneSystem,
};

mod host;
mod immune_system;
mod pathogens;
mod terrain;
mod ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(setup)
                .with_system(immune_system::setup),
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(tear_down::<ScreenTag>))
        .add_plugin(terrain::TerrainPlugin)
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(state_management)
                .with_system(immune_system::movements)
                .with_system(host::aging)
                .with_system(host::is_sick)
                .with_system(pathogens::under_attack)
                .with_system(ui::status),
        );
    }
}

pub mod z_layers {
    pub const BLOODFIELD: f32 = 0.0;
    pub const TERRAIN: f32 = 1.0;
    pub const IMMUNE_SYSTEM: f32 = 2.0;
    pub const PATHOGEN: f32 = 3.0;
}

#[derive(Component)]
struct ScreenTag;

fn setup(
    mut commands: Commands,
    mut camera: Query<&mut Transform, (With<Camera>, Without<ImmuneSystem>)>,
) {
    if let Ok(mut transform) = camera.get_single_mut() {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
    }

    commands.insert_resource(HostState {
        age: 0.0,
        status: Status::Healthy,
        risks: Risks {
            bacteria: 1.0,
            virus: 1.0,
        },
    });
}

fn state_management(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        let _ = state.set(GameState::Menu);
    }
}
