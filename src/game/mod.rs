use bevy::prelude::*;

use crate::{progress::Progress, tear_down, GameState, GlobalState};

pub use self::host::HostState;
use self::{
    host::{Risks, Status},
    immune_system::ImmuneSystem,
};

mod host;
mod immune_system;
mod intro;
mod oldest;
mod pathogens;
pub mod tissue;
pub mod ui;
mod white_cells;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(setup)
                .with_system(immune_system::setup),
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(tear_down::<ScreenTag>))
        .add_plugin(tissue::TissuePlugin)
        .add_plugin(intro::IntroPlugin)
        .add_plugin(oldest::OldestPlugin)
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(state_management)
                .with_system(immune_system::movements)
                .with_system(immune_system::health)
                .with_system(immune_system::spawn_white_cell)
                .with_system(host::aging)
                .with_system(host::state_update)
                .with_system(pathogens::spawn)
                .with_system(pathogens::movements)
                .with_system(pathogens::collisions)
                .with_system(pathogens::refresh_hit)
                .with_system(pathogens::cancer_replication)
                .with_system(white_cells::movements)
                .with_system(white_cells::attack)
                .with_system(ui::status),
        );
    }
}

pub mod z_layers {
    pub const BLOODFIELD: f32 = 0.0;
    pub const TISSUE: f32 = 1.0;
    pub const IMMUNE_SYSTEM: f32 = 2.0;
    pub const PATHOGEN: f32 = 3.0;
}

#[derive(Component)]
pub struct ScreenTag;

fn setup(
    mut commands: Commands,
    mut camera: Query<&mut Transform, (With<Camera>, Without<ImmuneSystem>)>,
    mut state: ResMut<State<GameState>>,
    global_state: Res<GlobalState>,
) {
    if let Ok(mut transform) = camera.get_single_mut() {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
    }

    let mut bacteria = 2.0;
    let mut virus = 1.5;
    let mut cancer = 0.0;
    let mut regen = 0.0;
    let mut dilatation = 500.0 + global_state.generation as f32 * 5.0;
    if global_state.has(&Progress::Disinfectant) {
        bacteria -= 0.2;
        virus -= 0.2;
    }
    if global_state.has(&Progress::Antibiotics) {
        regen += 0.3;
    }
    if global_state.has(&Progress::Vaccine) {
        virus -= 0.5;
    }
    if global_state.has(&Progress::PersonalHygiene) {
        bacteria -= 0.15;
        virus -= 0.15;
        regen += 0.15;
    }
    if global_state.has(&Progress::Sanitation) {
        bacteria -= 0.2;
        virus -= 0.2;
    }
    if global_state.has(&Progress::PreventiveMeasures) {
        bacteria -= 0.2;
        virus -= 0.2;
    }
    if global_state.has(&Progress::SickDays) {
        dilatation += 200.0;
        bacteria -= 0.15;
        virus -= 0.1;
    }
    if global_state.has(&Progress::FreeHealthcare) {
        dilatation += 200.0;
        cancer -= 0.02;
    }

    commands.insert_resource(HostState {
        age: 0.0,
        status: Status::Healthy,
        risks: Risks {
            bacteria,
            virus,
            cancer,
        },
        sickness: 0.0,
        regen,
        dilatation,
    });

    let _ = state.push(GameState::Intro);
}

fn state_management(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        let _ = state.set(GameState::Menu);
    }
}
