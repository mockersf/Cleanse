use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    IntersectionEvent, IntoEntity, RigidBodyForcesComponent, RigidBodyPositionComponent,
};

use super::{immune_system::ImmuneSystem, pathogens::Pathogen, HostState};

#[derive(Component)]
pub struct WhiteCell {
    pub spawned_at: f32,
    pub strength: f32,
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

pub fn attack(
    mut commands: Commands,
    mut intersection_events: EventReader<IntersectionEvent>,
    white_cells: Query<&WhiteCell>,
    pathogens: Query<&Pathogen>,
    mut host_state: ResMut<HostState>,
) {
    let mut hit = vec![];
    let mut destroyed = 0;
    for event in intersection_events.iter() {
        if event.intersecting {
            let e1 = event.collider1.entity();
            let e2 = event.collider2.entity();
            let (white_cell, pathogen) = if let Ok(white_cell) = white_cells.get(e1) {
                if let Ok(pathogen) = pathogens.get(e2) {
                    ((e1, white_cell), (e2, pathogen))
                } else {
                    continue;
                }
            } else if let Ok(white_cell) = white_cells.get(e2) {
                if let Ok(pathogen) = pathogens.get(e1) {
                    ((e2, white_cell), (e1, pathogen))
                } else {
                    continue;
                }
            } else {
                continue;
            };
            if hit.contains(&pathogen.0) {
                continue;
            }
            hit.push(pathogen.0);
            if white_cell.1.strength > pathogen.1.strength {
                commands.entity(white_cell.0).despawn_recursive();
                commands.entity(pathogen.0).despawn_recursive();
                destroyed += 1;
            } else {
                commands.entity(white_cell.0).despawn_recursive();
            }
        }
    }
    host_state.exp += destroyed;
}
