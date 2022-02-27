use bevy::prelude::*;

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

pub struct HostState {
    pub age: f32,
    pub status: Status,
}

pub fn aging(mut state: ResMut<HostState>, time: Res<Time>) {
    state.age += time.delta_seconds();
}
