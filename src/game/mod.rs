use bevy::prelude::*;

use crate::{tear_down, GameState};

mod player_movement;
mod terrain;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup))
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(tear_down::<ScreenTag>),
            )
            .add_plugin(terrain::TerrainPlugin)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(state_management)
                    .with_system(player_movement::player_movements),
            );
    }
}

pub mod z_layers {
    pub const BLOODFIELD: f32 = 0.0;
    pub const TERRAIN: f32 = 1.0;
    pub const PLAYER: f32 = 2.0;
}

#[derive(Component)]
struct ScreenTag;

#[derive(Component)]
pub struct Player {
    speed: f32,
}

fn setup(
    mut commands: Commands,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, z_layers::PLAYER),
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player { speed: 100.0 })
        .insert(ScreenTag);

    if let Ok(mut transform) = camera.get_single_mut() {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
    }
}

fn state_management(keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if keyboard_input.pressed(KeyCode::Escape) {
        let _ = state.set(GameState::Menu);
    }
}
