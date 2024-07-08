//! Handle player input and translate it into velocity.

use bevy::prelude::*;

use super::physics::Velocity;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, handle_input);
}

/// Handle keyboard input to move the player.
fn handle_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Velocity>) {
    /// Since Bevy's default 2D camera setup is scaled such that
    /// one unit is one pixel, you can think of this as
    /// "How many pixels per second should the player move?"
    /// Note that physics engines may use different unit/pixel ratios.
    const SPEED: f32 = 210.0;
    for mut velocity in query.iter_mut() {
        velocity.0 = Vec3::ZERO;

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

        // Need to normalize and scale because otherwise
        // diagonal movement would be faster than horizontal or vertical movement.
        velocity.0 = velocity.normalize_or_zero() * SPEED;
    }
}
