use bevy::prelude::*;

use crate::{tear_down, GameState};

use self::host::{HostState, Risks, Status};

mod host;
mod pathogens;
mod player_movement;
mod terrain;
mod ui;

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
                    .with_system(player_movement::player_movements)
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
    pub const PLAYER: f32 = 2.0;
    pub const PATHOGEN: f32 = 3.0;
}

#[derive(Component)]
struct ScreenTag;

#[derive(Component)]
pub struct ImmuneSystem {
    speed: f32,
}

fn setup(
    mut commands: Commands,
    mut camera: Query<&mut Transform, (With<Camera>, Without<ImmuneSystem>)>,
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
        .insert(ImmuneSystem { speed: 100.0 })
        .insert(ScreenTag);

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
