//! Handle player input and translate it into movement through a character controller.
//! A character controller is the collection of systems that govern the movement of characters.
//!
//! In our case, the character controller has the following logic:
//! - Set the movement intent based on directional input.
//! - Apply movement based on the intent.
//! - Wrap the character within the window.
//!
//! Note that the implementation used here is simple for demonstration purposes.
//! If you want to move the player in a smoother way,
//! consider using a [fixed timestep](https://github.com/bevyengine/bevy/blob/main/examples/movement/physics_in_fixed_timestep.rs).

use bevy::{prelude::*, window::PrimaryWindow};

use crate::AppSet;

pub(super) fn plugin(app: &mut App) {
    // Record directional input as movement controls.
    app.register_type::<MovementController>();
    app.add_systems(
        Update,
        record_movement_controller.in_set(AppSet::RecordInput),
    );

    // Apply movement based on controls.
    app.register_type::<(Movement, ScreenWrap)>();
    app.add_systems(
        Update,
        (apply_movement, wrap_within_window)
            .chain()
            .in_set(AppSet::Update),
    );
}

/// This is the input for our character controller,
/// which in this case is only the direction the character wants to move in.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController {
    pub intent: Vec2,
}

fn record_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
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

    // Normalize so that diagonal movement has the same speed as
    // horizontal and vertical movement.
    // Skip this if we want to support fractional input from analog sticks.
    let intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.intent = intent;
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Movement {
    /// Since Bevy's default 2D camera setup is scaled such that
    /// one unit is one pixel, you can think of this as
    /// "How many pixels per second should the player move?"
    /// Note that physics engines may use different unit/pixel ratios.
    pub speed: f32,
}

fn apply_movement(
    time: Res<Time>,
    mut movement_query: Query<(&MovementController, &Movement, &mut Transform)>,
) {
    for (controller, movement, mut transform) in &mut movement_query {
        let velocity = movement.speed * controller.intent;
        transform.translation += velocity.extend(0.0) * time.delta_seconds();
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScreenWrap;

fn wrap_within_window(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut wrap_query: Query<&mut Transform, With<ScreenWrap>>,
) {
    let size = window_query.single().size() + 256.0;
    let half_size = size / 2.0;
    for mut transform in &mut wrap_query {
        let position = transform.translation.xy();
        let wrapped = (position + half_size).rem_euclid(size) - half_size;
        transform.translation = wrapped.extend(transform.translation.z);
    }
}
