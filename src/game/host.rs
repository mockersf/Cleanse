use bevy::prelude::*;

use crate::GameState;

use super::{immune_system::ImmuneSystem, pathogens::Pathogen};

pub enum Status {
    Healthy,
    Sick,
    Dead,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Healthy => f.pad("Healthy"),
            Status::Sick => f.pad("Sick"),
            Status::Dead => f.pad("Dead"),
        }
    }
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
}

pub fn aging(mut state: ResMut<HostState>, time: Res<Time>) {
    state.age += time.delta_seconds();
}

pub fn state_update(
    mut host_state: ResMut<HostState>,
    pathogens: Query<&Pathogen>,
    immune_system: Query<&ImmuneSystem>,
    mut state: ResMut<State<GameState>>,
) {
    let pathogen_level: f32 = pathogens.iter().map(|p| p.strength).sum();
    if pathogen_level > 10.0 {
        host_state.status = Status::Sick
    } else {
        host_state.status = Status::Healthy
    }
    host_state.sickness = (pathogens.iter().len() as f32 / 50.0).min(1.0);

    let immune_system = immune_system.single();
    if immune_system.health < 0.0 {
        host_state.status = Status::Dead;
        let _ = state.push(GameState::Dead);
    }
}
