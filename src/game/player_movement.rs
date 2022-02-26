use bevy::prelude::*;

use super::Player;

pub fn player_movements(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let mut order = Vec2::ZERO;
    let speed = 50.0;
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
        let move_by = order.normalize().extend(0.0) * time.delta_seconds() * speed;
        player.single_mut().translation += move_by;
    }
}

pub fn camera_follow_player(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player = player.single().translation.truncate();
    let mut camera_position = camera.single_mut();
    camera_position.translation.x = player.x;
    camera_position.translation.y = player.y;
}
