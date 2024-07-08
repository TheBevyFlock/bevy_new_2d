use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Component)]
struct Player;

#[derive(Debug, Clone, Copy, PartialEq, Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            velocity.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            velocity.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            velocity.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            velocity.x += 1.0;
        }
    }
}

fn dampen_velocity(mut query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in query.iter_mut() {
        velocity.0 *= 0.9;
    }
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Player>>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += Vec3::new(velocity.0.x, velocity.0.y, 0.0) * time.delta_seconds();
    }
}
