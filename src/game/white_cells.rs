use bevy::prelude::*;
use bevy_rapier2d::prelude::{RigidBodyForcesComponent, RigidBodyPositionComponent};

use super::immune_system::ImmuneSystem;

#[derive(Component)]
pub struct WhiteCell {
    pub spawned_at: f32,
}

pub fn movements(
    immune_system: Query<&Transform, With<ImmuneSystem>>,
    mut pathogens: Query<(
        &RigidBodyPositionComponent,
        &mut RigidBodyForcesComponent,
        &WhiteCell,
    )>,
    time: Res<Time>,
) {
    let target = immune_system.single().translation.truncate();
    for (rb_position, mut rb_forces, white_cell) in pathogens.iter_mut() {
        let position: Vec2 = rb_position.position.translation.into();
        let target = target
            + Vec2::new(
                (time.seconds_since_startup() as f32 * 5.0 - white_cell.spawned_at).sin(),
                (time.seconds_since_startup() as f32 * 5.0 - white_cell.spawned_at).cos(),
            ) * 12.5;
        let order = target - position;
        if order.length_squared() < 150.0 {
            let move_by = order.clamp_length_max(2.0) * 10000.0;
            rb_forces.force = move_by.into();
        } else {
            let move_by = order * 10000.0;
            rb_forces.force = move_by.into();
        }
    }
}
