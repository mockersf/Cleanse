use bevy::prelude::*;

use crate::{tear_down, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(tear_down::<ScreenTag>),
            )
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(state_management));
    }
}

pub mod z_layers {
    pub const BLOODFIELD: f32 = 0.0;
}

#[derive(Component)]
struct ScreenTag;

fn setup() {}

fn state_management(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        let _ = state.set(GameState::Menu);
    }
}
