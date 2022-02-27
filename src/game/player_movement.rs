use bevy::prelude::*;

use super::ImmuneSystem;

pub fn player_movements(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player: Query<(&mut Transform, &ImmuneSystem)>,
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
        let mut player = player.single_mut();
        let move_by = order.normalize().extend(0.0) * time.delta_seconds();
        player.0.translation += move_by * player.1.speed;
        let window = windows.get_primary().unwrap();
        let (width, height) = (window.width() * 0.975, window.height() * 0.975);
        player.0.translation.x = player.0.translation.x.clamp(-width / 2.0, width / 2.0);
        player.0.translation.y = player.0.translation.y.clamp(-height / 2.0, height / 2.0);
    }
}
