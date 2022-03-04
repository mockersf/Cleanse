use bevy::prelude::*;

use crate::{assets::AudioAssets, GameState, GlobalState};

use super::{immune_system::ImmuneSystem, pathogens::Pathogen};

pub enum Status {
    Healthy,
    Sick,
    Dead,
}

#[derive(Debug)]
pub struct Risks {
    pub bacteria: f32,
    pub virus: f32,
    pub cancer: f32,
}

pub struct HostState {
    pub age: f32,
    pub status: Status,
    pub risks: Risks,
    pub sickness: f32,
    pub regen: f32,
    pub dilatation: f32,
    pub next_level_up: f32,
    pub exp: usize,
}

pub fn aging(
    mut host_state: ResMut<HostState>,
    time: Res<Time>,
    mut state: ResMut<State<GameState>>,
    mut oldest: Local<bool>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    host_state.age += time.delta_seconds();
    if !*oldest && host_state.age > 300.0 {
        audio.play(
            audio_assets.won.clone_weak(),
            PlaybackSettings {
                repeat: false,
                speed: 1.0,
                volume: 0.3,
            },
        );
        let _ = state.push(GameState::Oldest);
        *oldest = true;
    }
    if host_state.risks.cancer <= 0.01 && host_state.age > 50.0 {
        host_state.risks.cancer += 0.2;
    }
    if (host_state.age + host_state.exp as f32 * 2.0) > host_state.next_level_up {
        host_state.next_level_up += (host_state.next_level_up * 1.5).min(150.0);
        audio.play(
            audio_assets.levelup.clone_weak(),
            PlaybackSettings {
                repeat: false,
                speed: 1.0,
                volume: 0.2,
            },
        );
        let _ = state.push(GameState::LevelUp);
    }
    host_state.dilatation += time.delta_seconds() / 2.0;
}

pub fn state_update(
    mut host_state: ResMut<HostState>,
    global_state: ResMut<GlobalState>,
    pathogens: Query<&Pathogen>,
    immune_system: Query<&ImmuneSystem>,
    mut state: ResMut<State<GameState>>,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    let immune_system = immune_system.single();
    if immune_system.health / immune_system.original_health < 0.4 {
        host_state.status = Status::Sick
    } else {
        host_state.status = Status::Healthy
    }
    host_state.sickness = (pathogens.iter().len() as f32
        / ((global_state.generation as f32 + 1.0).min(5.0) * 25.0))
        .min(1.0);

    if immune_system.health <= 0.0 {
        host_state.status = Status::Dead;
        audio.play(
            audio_assets.lost.clone_weak(),
            PlaybackSettings {
                repeat: false,
                speed: 1.0,
                volume: 0.2,
            },
        );

        let _ = state.push(GameState::Dead);
    }
}
