use bevy::prelude::*;
use bevy_egui::egui::lerp;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use strum::IntoEnumIterator;

use crate::{
    assets::InGameAssets,
    progress::{Effect, Progress},
    GlobalState,
};

use super::{white_cells::WhiteCell, z_layers, HostState, ScreenTag};

#[derive(Component)]
pub struct ImmuneSystem {
    pub speed: f32,
    pub health: f32,
    pub original_health: f32,
    pub attack_spawn_rate: f32,
}

impl ImmuneSystem {
    fn new(speed: f32, health: f32, attack_spawn_rate: f32) -> ImmuneSystem {
        ImmuneSystem {
            speed,
            health,
            original_health: health,
            attack_spawn_rate,
        }
    }
}

pub fn setup(mut commands: Commands, global_state: Res<GlobalState>, assets: Res<InGameAssets>) {
    let mut speed = 70.0 + 4.0 * global_state.generation as f32;
    let mut health = 10.0 + global_state.generation as f32 / 2.0 + global_state.progress / 700.0;
    let mut attack = global_state.generation as f32 / 180.0 + global_state.progress / 5000.0;
    let mut effect = Effect::default();
    for progress in Progress::iter() {
        if global_state.has(&progress) {
            effect.apply(progress);
        }
    }
    speed += effect.speed;
    health += effect.health;
    attack += effect.attack;

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, z_layers::IMMUNE_SYSTEM),
            texture: assets.immune_system.clone_weak(),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: Vec2::new(0.0, 0.0).into(),
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            damping: RigidBodyDamping {
                linear_damping: 15.0,
                angular_damping: 10.0,
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::ball(5.0).into(),
            flags: ActiveEvents::CONTACT_EVENTS.into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(ImmuneSystem::new(speed, health, attack))
        .insert(ScreenTag);
}

pub fn movements(
    keyboard_input: Res<Input<KeyCode>>,
    mut immune_system: Query<(
        &mut RigidBodyPositionComponent,
        &mut RigidBodyForcesComponent,
        &ImmuneSystem,
    )>,
    windows: Res<Windows>,
    host_state: Res<HostState>,
    global_state: Res<GlobalState>,
) {
    let mut order = Vec2::ZERO;
    if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
        order.x += 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
        order.x -= 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
        order.y += 1.0;
    }
    if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
        order.y -= 1.0;
    }

    let (mut rb_position, mut rb_forces, immune_system) = immune_system.single_mut();
    if order != Vec2::ZERO {
        let position: Vec2 = rb_position.position.translation.into();
        let distance_to_zero = (position.distance_squared(Vec2::ZERO) - 10_000.0).max(0.0);
        let move_by = order.normalize()
            * immune_system.speed
            * (1.0 - distance_to_zero / 500_000.0)
            * (1.0 / (host_state.age / global_state.expectancy.max(50.0)).max(1.0))
            * 1000.0;
        rb_forces.force = move_by.into();
    }
    let window = windows.get_primary().unwrap();
    let (width, height) = (window.width() * 0.985, window.height() * 0.975);
    rb_position.position.translation.x = rb_position
        .position
        .translation
        .x
        .clamp(-width / 2.0, width / 2.0);
    rb_position.position.translation.y = rb_position
        .position
        .translation
        .y
        .clamp(-height / 2.0, height / 2.0 * 0.925);
}

pub fn health(
    time: Res<Time>,
    mut immune_system: Query<(&RigidBodyPositionComponent, &mut ImmuneSystem)>,
    host_state: Res<HostState>,
) {
    let (rb_position, mut immune_system) = immune_system.single_mut();
    let position: Vec2 = rb_position.position.translation.into();
    let distance_to_zero =
        position.distance_squared(Vec2::ZERO) - (host_state.dilatation / 5.0).powi(2);
    immune_system.health -=
        distance_to_zero.max(0.0) / 250_000.0 * time.delta_seconds() * host_state.sickness * 1.5;
    immune_system.health = (immune_system.health
        + (distance_to_zero.min(0.0).abs() / (host_state.dilatation / 5.0).powi(2))
            * time.delta_seconds()
            * host_state.regen)
        .min(immune_system.original_health);
}

pub fn spawn_white_cell(
    mut commands: Commands,
    immune_system: Query<(&RigidBodyPositionComponent, &ImmuneSystem)>,
    time: Res<Time>,
    host: Res<HostState>,
    global: Res<GlobalState>,
    assets: Res<InGameAssets>,
) {
    let (position, immune_system) = immune_system.single();
    let rate = if host.age < (global.expectancy / 2.0).min(100.0) {
        lerp(0.0..=immune_system.attack_spawn_rate, host.age / 100.0)
    } else if host.age > (global.expectancy * 1.5).max(200.0) {
        lerp(
            immune_system.attack_spawn_rate..=(immune_system.attack_spawn_rate * 1.3),
            (host.age - global.expectancy.max(200.0)) / (300.0 - global.expectancy.max(200.0)),
        )
    } else {
        immune_system.attack_spawn_rate
    };
    if rand::thread_rng().gen_bool((rate * time.delta_seconds()).clamp(0.0, 1.0) as f64) {
        let mut velocity = RigidBodyVelocity::zero();
        velocity.angvel = 1.0;

        commands
            .spawn_bundle(SpriteBundle {
                transform: Transform::from_xyz(0.0, 0.0, z_layers::IMMUNE_SYSTEM),
                texture: assets.white_cell.clone_weak(),
                ..Default::default()
            })
            .insert_bundle(RigidBodyBundle {
                position: (**position).into(),
                damping: RigidBodyDamping {
                    linear_damping: 15.0,
                    angular_damping: 0.0,
                }
                .into(),
                velocity: velocity.into(),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                collider_type: ColliderType::Sensor.into(),
                shape: ColliderShape::ball(2.0).into(),
                flags: ActiveEvents::INTERSECTION_EVENTS.into(),
                ..Default::default()
            })
            .insert(RigidBodyPositionSync::Discrete)
            .insert(WhiteCell {
                spawned_at: time.seconds_since_startup() as f32,
                strength: 50.0,
            })
            .insert(ScreenTag);
    }
}
