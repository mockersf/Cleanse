use std::time::Duration;

use bevy::prelude::*;
use bevy_easings::{Ease, EaseFunction, EasingType};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::assets::InGameAssets;

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
    assets: Res<InGameAssets>,
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
        let mut velocity = RigidBodyVelocity::zero();
        velocity.angvel = rng.gen_range(-0.5..0.5);
        commands
            .spawn_bundle(PathogenBundle {
                sprite: SpriteBundle {
                    transform: Transform {
                        translation: position.extend(z_layers::PATHOGEN),
                        scale: Vec2::ZERO.extend(1.0),
                        rotation: Quat::IDENTITY,
                    },
                    sprite: Sprite {
                        color: Color::WHITE,
                        flip_x: rng.gen_bool(0.5),
                        flip_y: rng.gen_bool(0.5),
                        custom_size: None,
                    },
                    texture: assets.bacteria.clone_weak(),
                    ..Default::default()
                },
                rigid_body: RigidBodyBundle {
                    position: position.into(),
                    damping: RigidBodyDamping {
                        linear_damping: 15.0,
                        angular_damping: 0.0,
                    }
                    .into(),
                    velocity: velocity.into(),
                    ..Default::default()
                },
                collider: ColliderBundle {
                    shape: ColliderShape::ball(8.0).into(),
                    flags: ColliderFlags {
                        solver_groups: InteractionGroups::new(1, 1),
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                },
                position_sync: RigidBodyPositionSync::Discrete,
                pathogen_spec: Bacteria,
                pathogen: Pathogen {
                    speed: 50.0,
                    strength: 10.0,
                    last_hit: Timer::from_seconds(1.0, true),
                    in_contact: false,
                },
                tag: ScreenTag,
            })
            .insert(
                Transform {
                    translation: position.extend(z_layers::PATHOGEN),
                    scale: Vec2::ZERO.extend(1.0),
                    rotation: Quat::IDENTITY,
                }
                .ease_to(
                    Transform {
                        translation: position.extend(z_layers::PATHOGEN),
                        scale: Vec3::ONE,
                        rotation: Quat::IDENTITY,
                    },
                    EaseFunction::CubicOut,
                    EasingType::Once {
                        duration: Duration::from_millis(4000),
                    },
                ),
            );
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
        let mut velocity = RigidBodyVelocity::zero();
        velocity.angvel = rng.gen_range(-1.5..1.5);
        commands
            .spawn_bundle(PathogenBundle {
                sprite: SpriteBundle {
                    transform: Transform {
                        translation: position.extend(z_layers::PATHOGEN),
                        scale: Vec2::ZERO.extend(1.0),
                        rotation: Quat::IDENTITY,
                    },
                    sprite: Sprite {
                        color: Color::WHITE,
                        flip_x: rng.gen_bool(0.5),
                        flip_y: rng.gen_bool(0.5),
                        custom_size: None,
                    },
                    texture: assets.virus.clone_weak(),
                    ..Default::default()
                },
                rigid_body: RigidBodyBundle {
                    position: position.into(),
                    damping: RigidBodyDamping {
                        linear_damping: 15.0,
                        angular_damping: 0.0,
                    }
                    .into(),
                    velocity: velocity.into(),
                    ..Default::default()
                },
                collider: ColliderBundle {
                    shape: ColliderShape::ball(5.0).into(),
                    flags: ColliderFlags {
                        solver_groups: InteractionGroups::new(1, 1),
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                },
                position_sync: RigidBodyPositionSync::Discrete,
                pathogen_spec: Bacteria,
                pathogen: Pathogen {
                    speed: 75.0,
                    strength: 2.0,
                    last_hit: Timer::from_seconds(1.0, true),
                    in_contact: false,
                },
                tag: ScreenTag,
            })
            .insert(
                Transform {
                    translation: position.extend(z_layers::PATHOGEN),
                    scale: Vec2::ZERO.extend(1.0),
                    rotation: Quat::IDENTITY,
                }
                .ease_to(
                    Transform {
                        translation: position.extend(z_layers::PATHOGEN),
                        scale: Vec3::ONE,
                        rotation: Quat::IDENTITY,
                    },
                    EaseFunction::CubicOut,
                    EasingType::Once {
                        duration: Duration::from_millis(4000),
                    },
                ),
            );
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
        spawn_cancer_cell(&mut commands, position, 0.12, assets.cancer.clone_weak());
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
    assets: Res<InGameAssets>,
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
            spawn_cancer_cell(&mut commands, position, 0.035, assets.cancer.clone_weak());
        }
    }
}

fn spawn_cancer_cell(
    commands: &mut Commands,
    position: Vec2,
    replication: f32,
    texture: Handle<Image>,
) {
    let mut rng = rand::thread_rng();
    commands.spawn_bundle(PathogenBundle {
        sprite: SpriteBundle {
            transform: Transform::from_translation(position.extend(z_layers::CANCER)),
            sprite: Sprite {
                color: Color::WHITE,
                flip_x: rng.gen_bool(0.5),
                flip_y: rng.gen_bool(0.5),
                custom_size: None,
            },
            texture,
            ..Default::default()
        },
        rigid_body: RigidBodyBundle {
            position: position.into(),
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            damping: RigidBodyDamping {
                linear_damping: 200.0,
                angular_damping: 200.0,
            }
            .into(),
            ..Default::default()
        },
        collider: ColliderBundle {
            mass_properties: ColliderMassProps::Density(100.0).into(),
            shape: ColliderShape::ball(8.0).into(),
            flags: ColliderFlags {
                solver_groups: InteractionGroups::new(2, 2),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        },
        position_sync: RigidBodyPositionSync::Discrete,
        pathogen_spec: Cancer { replication },
        pathogen: Pathogen {
            speed: -800.0,
            strength: 1000.0,
            last_hit: Timer::from_seconds(1.0, true),
            in_contact: false,
        },
        tag: ScreenTag,
    });
}

#[derive(Bundle)]
struct PathogenBundle<T: 'static + Sync + Send + Component> {
    #[bundle]
    sprite: SpriteBundle,
    #[bundle]
    rigid_body: RigidBodyBundle,
    #[bundle]
    collider: ColliderBundle,
    pathogen_spec: T,
    tag: ScreenTag,
    position_sync: RigidBodyPositionSync,
    pathogen: Pathogen,
}
