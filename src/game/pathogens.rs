use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

use super::{host::HostState, immune_system::ImmuneSystem, z_layers, ScreenTag};

#[derive(Component)]
pub struct Bacteria;

#[derive(Component)]
pub struct Virus;

#[derive(Component)]
pub struct Cancer {
    replication: f32,
}

#[derive(Component)]
pub struct Pathogen {
    pub strength: f32,
    speed: f32,
    last_hit: Timer,
    in_contact: bool,
}

pub fn spawn(
    mut commands: Commands,
    state: Res<HostState>,
    time: Res<Time>,
    windows: Res<Windows>,
) {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(
        ((state.risks.bacteria + state.age / 400.0) * time.delta_seconds()).clamp(0.0, 1.0) as f64,
    ) {
        let window = windows.get_primary().unwrap();
        let (width, height) = (window.width() * 0.985, window.height() * 0.975);
        let position = std::iter::repeat_with(|| {
            Vec2::new(
                rng.gen_range((-width / 2.0)..(width / 2.0)),
                rng.gen_range((-height / 2.0)..(height / 2.0 * 0.9)),
            )
        })
        .find(|pos| pos.distance_squared(Vec2::ZERO) > 40_000.0)
        .unwrap();
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
                flags: ColliderFlags {
                    solver_groups: InteractionGroups::new(1, 1),
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete)
            .insert_bundle((
                Bacteria,
                Pathogen {
                    speed: 50.0,
                    strength: 10.0,
                    last_hit: Timer::from_seconds(1.0, true),
                    in_contact: false,
                },
                ScreenTag,
            ));
    }
    if rng.gen_bool(
        ((state.risks.virus + state.age / 400.0) * time.delta_seconds()).clamp(0.0, 1.0) as f64,
    ) {
        let window = windows.get_primary().unwrap();
        let (width, height) = (window.width() * 0.985, window.height() * 0.975);
        let position = std::iter::repeat_with(|| {
            Vec2::new(
                rng.gen_range((-width / 2.0)..(width / 2.0)),
                rng.gen_range((-height / 2.0)..(height / 2.0 * 0.9)),
            )
        })
        .find(|pos| pos.distance_squared(Vec2::ZERO) > 40_000.0)
        .unwrap();
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
                flags: ColliderFlags {
                    solver_groups: InteractionGroups::new(1, 1),
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete)
            .insert_bundle((
                Bacteria,
                Pathogen {
                    speed: 75.0,
                    strength: 2.0,
                    last_hit: Timer::from_seconds(1.0, true),
                    in_contact: false,
                },
                ScreenTag,
            ));
    }
    if rng.gen_bool((state.risks.cancer * time.delta_seconds()).clamp(0.0, 1.0) as f64) {
        let window = windows.get_primary().unwrap();
        let (width, height) = (window.width() * 0.985, window.height() * 0.975);
        let position = std::iter::repeat_with(|| {
            Vec2::new(
                rng.gen_range((-width / 2.0)..(width / 2.0)),
                rng.gen_range((-height / 2.0)..(height / 2.0 * 0.9)),
            )
        })
        .find(|pos| pos.distance_squared(Vec2::ZERO) > 60_000.0)
        .unwrap();
        spawn_cancer_cell(&mut commands, position, 0.12);
    }
}

pub fn movements(
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
        let move_by = order.normalize() * pathogen.speed * 1000.0;
        rb_forces.force = move_by.into();
    }
}

pub fn collisions(
    mut contact_events: EventReader<ContactEvent>,
    mut pathogens: Query<&mut Pathogen>,
    immune_system: Query<&ImmuneSystem>,
) {
    for contact_event in contact_events.iter() {
        match contact_event {
            ContactEvent::Started(h1, h2) => {
                let entity1 = h1.entity();
                let entity2 = h2.entity();
                if let Ok(mut pat) = {
                    if immune_system.contains(entity1) {
                        pathogens.get_mut(entity2)
                    } else if immune_system.contains(entity2) {
                        pathogens.get_mut(entity1)
                    } else {
                        continue;
                    }
                } {
                    pat.in_contact = true;
                    let d = pat.last_hit.duration();
                    pat.last_hit.set_elapsed(d);
                }
            }
            ContactEvent::Stopped(h1, h2) => {
                let entity1 = h1.entity();
                let entity2 = h2.entity();
                if let Ok(mut pat) = {
                    if immune_system.contains(entity1) {
                        pathogens.get_mut(entity2)
                    } else if immune_system.contains(entity2) {
                        pathogens.get_mut(entity1)
                    } else {
                        continue;
                    }
                } {
                    pat.in_contact = false;
                }
            }
        };
    }
}

pub fn refresh_hit(
    mut pathogens: Query<&mut Pathogen>,
    mut immune_system: Query<&mut ImmuneSystem>,
    time: Res<Time>,
) {
    for mut pathogen in pathogens.iter_mut() {
        if pathogen.last_hit.tick(time.delta()).just_finished() && pathogen.in_contact {
            let mut immune_system = immune_system.single_mut();
            immune_system.health -= pathogen.strength;
        }
    }
}

pub fn cancer_replication(
    mut commands: Commands,
    time: Res<Time>,
    mut cancer_cells: Query<(&Transform, &mut Cancer)>,
) {
    let mut rng = rand::thread_rng();
    for (transform, mut cancer) in cancer_cells.iter_mut() {
        if rng.gen_bool((cancer.replication * time.delta_seconds()).clamp(0.0, 1.0) as f64) {
            cancer.replication /= 2.0;
            let position = transform.translation.truncate()
                + Vec2::new(
                    time.seconds_since_startup().sin() as f32,
                    time.seconds_since_startup().cos() as f32,
                ) * 4.0;
            spawn_cancer_cell(&mut commands, position, 0.03);
        }
    }
}

fn spawn_cancer_cell(commands: &mut Commands, position: Vec2, replication: f32) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(position.extend(z_layers::PATHOGEN)),
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: position.into(),
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            damping: RigidBodyDamping {
                linear_damping: 200.0,
                angular_damping: 200.0,
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            mass_properties: ColliderMassProps::Density(100.0).into(),
            shape: ColliderShape::ball(10.0).into(),
            flags: ColliderFlags {
                solver_groups: InteractionGroups::new(2, 2),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert_bundle((
            Cancer { replication },
            Pathogen {
                speed: 0.0,
                strength: 100.0,
                last_hit: Timer::from_seconds(1.0, true),
                in_contact: false,
            },
            ScreenTag,
        ));
}
