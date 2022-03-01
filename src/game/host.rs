use bevy::prelude::*;

use crate::{GameState, GlobalState};

use super::{immune_system::ImmuneSystem, pathogens::Pathogen};

pub enum Status {
    Healthy,
    Sick,
    Dead,
}

pub struct Risks {
    pub bacteria: f32,
    pub virus: f32,
}

pub struct HostState {
    pub age: f32,
    pub status: Status,
    pub risks: Risks,
    pub sickness: f32,
    pub regen: f32,
}

pub fn aging(mut state: ResMut<HostState>, time: Res<Time>) {
    state.age += time.delta_seconds();
}

pub fn state_update(
    mut host_state: ResMut<HostState>,
    global_state: ResMut<GlobalState>,
    pathogens: Query<&Pathogen>,
    immune_system: Query<&ImmuneSystem>,
    mut state: ResMut<State<GameState>>,
) {
    let immune_system = immune_system.single();
    if immune_system.health / immune_system.original_health < 0.4 {
        host_state.status = Status::Sick
    } else {
        host_state.status = Status::Healthy
    }
    host_state.sickness =
        (pathogens.iter().len() as f32 / (global_state.generation as f32 * 25.0)).min(1.0);

    if immune_system.health <= 0.0 {
        host_state.status = Status::Dead;
        let _ = state.push(GameState::Dead);
    }
}
