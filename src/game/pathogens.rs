use bevy::prelude::*;
use rand::Rng;

use super::{host::HostState, z_layers, ScreenTag};

#[derive(Component)]
pub struct Bacteria {
    speed: f32,
}

#[derive(Component)]
pub struct Virus {
    speed: f32,
}

#[derive(Component)]
pub struct Pathogen {
    pub strength: f32,
}

pub fn under_attack(
    mut commands: Commands,
    state: Res<HostState>,
    time: Res<Time>,
    windows: Res<Windows>,
) {
    let mut rng = rand::thread_rng();
    if rng.gen_bool((state.risks.bacteria * time.delta_seconds()) as f64) {
        let window = windows.get_primary().unwrap();
        let (width, height) = (window.width() * 0.985, window.height() * 0.975);
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(
                    rng.gen_range((-width / 2.0)..(width / 2.0)),
                    rng.gen_range((-height / 2.0)..(height / 2.0 * 0.9)),
                    z_layers::PATHOGEN,
                ),
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert_bundle((
                Bacteria { speed: 100.0 },
                Pathogen { strength: 1.0 },
                ScreenTag,
            ));
    }
    if rng.gen_bool((state.risks.virus * time.delta_seconds()) as f64) {
        let window = windows.get_primary().unwrap();
        let (width, height) = (window.width() * 0.985, window.height() * 0.975);
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(
                    rng.gen_range((-width / 2.0)..(width / 2.0)),
                    rng.gen_range((-height / 2.0)..(height / 2.0 * 0.9)),
                    z_layers::PATHOGEN,
                ),
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert_bundle((
                Bacteria { speed: 200.0 },
                Pathogen { strength: 2.0 },
                ScreenTag,
            ));
    }
}
