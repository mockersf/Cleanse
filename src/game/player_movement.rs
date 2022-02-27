use bevy::prelude::*;

use super::Player;

pub fn player_movements(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&mut Transform, &Player)>,
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
        let mut player = player.single_mut();
        let move_by = order.normalize().extend(0.0) * time.delta_seconds();
        player.0.translation += move_by * player.1.speed;
    }
}
