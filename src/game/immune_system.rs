use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{z_layers, ScreenTag};

#[derive(Component)]
pub struct ImmuneSystem {
    pub speed: f32,
}

pub fn setup(mut commands: Commands) {
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
            damping: RigidBodyDamping {
                linear_damping: 10.0,
                angular_damping: 1.0,
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(5.0, 5.0).into(),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(ImmuneSystem { speed: 100.0 })
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
    if keyboard_input.pressed(KeyCode::Right) {
        order.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        order.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        order.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        order.y -= 1.0;
    }
    if order != Vec2::ZERO {
        let (mut rb_position, mut rb_forces, immune_system) = immune_system.single_mut();
        let move_by = order.normalize() * time.delta_seconds() * immune_system.speed * 100000.0;
        let window = windows.get_primary().unwrap();
        let (width, height) = (window.width() * 0.985, window.height() * 0.975);
        rb_forces.force = move_by.into();
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
}
