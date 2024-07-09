//! Handle player input and translate it into velocity.
//! Note that the approach used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/pull/14223).

use std::time::Duration;

use bevy::prelude::*;

use super::{audio::sfx::Sfx, spawn::player::Player, GameSystem};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        handle_player_movement_input.in_set(GameSystem::Movement),
    );
}

/// Since Bevy's default 2D camera setup is scaled such that
/// one unit is one pixel, you can think of this as
/// "How many pixels per second should the player move?"
/// Note that physics engines may use different unit/pixel ratios.
const MOVEMENT_SPEED: f32 = 420.0;

/// Time between walk sound effects.
const STEP_SFX_INTERVAL: Duration = Duration::from_millis(250);

/// Handle keyboard input to move the player.
fn handle_player_movement_input(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Sprite), With<Player>>,
    mut last_sfx: Local<Duration>,
    mut commands: Commands,
) {
    let mut intent = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
        intent.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
        intent.y -= 1.0;
    }
    if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
        intent.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
        intent.x += 1.0;
    }
    // Need to normalize and scale because otherwise
    // diagonal movement would be faster than horizontal or vertical movement.
    let intent = intent.normalize_or_zero();
    let target_velocity = intent * MOVEMENT_SPEED;

    for (mut transform, mut sprite) in &mut player_query {
        transform.translation += target_velocity * time.delta_seconds();
        if intent.x != 0.0 {
            sprite.flip_x = intent.x < 0.0;
        }
    }

    // If the player is moving, play a step sound effect.
    let now = time.elapsed();
    if intent != Vec3::ZERO && *last_sfx + STEP_SFX_INTERVAL < now {
        *last_sfx = now;
        commands.trigger(Sfx::Step);
    }
}
