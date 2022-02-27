use bevy::prelude::*;

use super::pathogens::Pathogen;

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

pub fn is_sick(mut state: ResMut<HostState>, pathogens: Query<&Pathogen>) {
    let pathogen_level: f32 = pathogens.iter().map(|p| p.strength).sum();
    if pathogen_level > 10.0 {
        state.status = Status::Sick
    } else {
        state.status = Status::Healthy
    }
    state.sickness = (pathogens.iter().len() as f32 / 50.0).min(1.0);
}
