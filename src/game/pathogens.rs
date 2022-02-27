use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use super::{host::HostState, immune_system::ImmuneSystem, z_layers, ScreenTag};

#[derive(Component)]
pub struct Bacteria;

#[derive(Component)]
pub struct Virus;

#[derive(Component)]
pub struct Pathogen {
    pub strength: f32,
    speed: f32,
}

pub fn spawn(
    mut commands: Commands,
    state: Res<HostState>,
    time: Res<Time>,
    windows: Res<Windows>,
) {
    let mut rng = rand::thread_rng();
    if rng.gen_bool((state.risks.bacteria * time.delta_seconds()) as f64) {
        let window = windows.get_primary().unwrap();
        let (width, height) = (window.width() * 0.985, window.height() * 0.975);
        let position = Vec2::new(
            rng.gen_range((-width / 2.0)..(width / 2.0)),
            rng.gen_range((-height / 2.0)..(height / 2.0 * 0.9)),
        );
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_translation(position.extend(z_layers::PATHOGEN)),
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert_bundle(RigidBodyBundle {
                position: position.into(),
                mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
                damping: RigidBodyDamping {
                    linear_damping: 10.0,
                    angular_damping: 1.0,
                }
                .into(),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                shape: ColliderShape::cuboid(8.0, 8.0).into(),
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete)
            .insert_bundle((
                Bacteria,
                Pathogen {
                    speed: 50.0,
                    strength: 10.0,
                },
                ScreenTag,
            ));
    }
    if rng.gen_bool((state.risks.virus * time.delta_seconds()) as f64) {
        let window = windows.get_primary().unwrap();
        let (width, height) = (window.width() * 0.985, window.height() * 0.975);
        let position = Vec2::new(
            rng.gen_range((-width / 2.0)..(width / 2.0)),
            rng.gen_range((-height / 2.0)..(height / 2.0 * 0.9)),
        );
        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_translation(position.extend(z_layers::PATHOGEN)),
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(10.0, 10.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert_bundle(RigidBodyBundle {
                position: position.into(),
                mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
                damping: RigidBodyDamping {
                    linear_damping: 10.0,
                    angular_damping: 10.0,
                }
                .into(),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                shape: ColliderShape::cuboid(5.0, 5.0).into(),
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete)
            .insert_bundle((
                Bacteria,
                Pathogen {
                    speed: 75.0,
                    strength: 2.0,
                },
                ScreenTag,
            ));
    }
}

pub fn movements(
    time: Res<Time>,
    immune_system: Query<&Transform, With<ImmuneSystem>>,
    mut pathogens: Query<(
        &RigidBodyPositionComponent,
        &mut RigidBodyForcesComponent,
        &Pathogen,
    )>,
) {
    let target = immune_system.single().translation.truncate();
    for (rb_position, mut rb_forces, pathogen) in pathogens.iter_mut() {
        let position: Vec2 = rb_position.position.translation.into();
        let order = target - position;
        let move_by = order.normalize() * time.delta_seconds() * pathogen.speed * 100000.0;
        rb_forces.force = move_by.into();
    }
}
