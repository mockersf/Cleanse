use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GlobalState;

use super::{z_layers, HostState, ScreenTag};

#[derive(Component)]
pub struct ImmuneSystem {
    pub speed: f32,
    pub health: f32,
    pub original_health: f32,
}

impl ImmuneSystem {
    fn new(speed: f32, health: f32) -> ImmuneSystem {
        ImmuneSystem {
            speed,
            health,
            original_health: health,
        }
    }
}

pub fn setup(mut commands: Commands, global_state: Res<GlobalState>) {
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, z_layers::IMMUNE_SYSTEM),
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: Vec2::new(0.0, 0.0).into(),
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
            flags: ActiveEvents::CONTACT_EVENTS.into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(ImmuneSystem::new(
            80.0 + 5.0 * global_state.generation as f32,
            10.0 + global_state.generation as f32 / 2.0,
        ))
        .insert(ScreenTag);
}

pub fn movements(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut immune_system: Query<(
        &mut RigidBodyPositionComponent,
        &mut RigidBodyForcesComponent,
        &ImmuneSystem,
    )>,
    windows: Res<Windows>,
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
        let distance_to_zero = (position.distance(Vec2::ZERO) - 100.0).max(0.0);
        let move_by = order.normalize()
            * time.delta_seconds()
            * immune_system.speed
            * (1.0 - distance_to_zero / 750.0)
            * 100000.0;
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
    let distance_to_zero = (position.distance(Vec2::ZERO) - 100.0).max(0.0);
    immune_system.health -= distance_to_zero / 500.0 * time.delta_seconds() * host_state.sickness;
}
