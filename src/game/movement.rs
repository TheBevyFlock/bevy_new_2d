use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app
        // `FixedUpdate` runs before `Update`, so the physics simulation is advanced before the
        // player's visual representation is updated.
        .add_systems(FixedUpdate, advance_physics)
        .add_systems(Update, (update_displayed_transform, handle_input).chain());
}

/// How many units per second the player should move.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub(crate) struct Velocity(pub(crate) Vec3);

/// The actual position of the player in the physics simulation.
/// This is separate from the `Transform`, which is merely a visual
/// representation.
///
/// If you want to make sure that this component is always initialized
/// with the same value as the `Transform`'s translation, you can
/// use a [component lifecycle hook](https://docs.rs/bevy/0.14.0/bevy/ecs/component/struct.ComponentHooks.html)
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub(crate) struct PhysicalTranslation(pub(crate) Vec3);

/// Handle keyboard input to move the player.
fn handle_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Velocity>) {
    /// Since Bevy's default 2D camera setup is scaled such that
    /// one unit is one pixel, you can think of this as
    /// "How many pixels per second should the player move?"
    const SPEED: f32 = 210.0;
    for mut velocity in query.iter_mut() {
        velocity.0 = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            velocity.y += SPEED;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            velocity.y -= SPEED;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            velocity.x -= SPEED;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            velocity.x += SPEED;
        }

        // Need to normalize and scale because otherwise
        // diagonal movement would be faster than horizontal or vertical movement.
        velocity.0 = velocity.normalize_or_zero() * SPEED;
    }
}

/// Advance the physics simulation by one fixed timestep. This may run zero or
/// multiple times per frame.
///
/// Note that since this runs in `FixedUpdate`, `Res<Time>` would be
/// `Res<Time<Fixed>>` automatically. We are being explicit here for clarity.
fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut PhysicalTranslation, &Velocity)>,
) {
    for (mut physical_translation, velocity) in query.iter_mut() {
        physical_translation.0 += velocity.0 * fixed_time.delta_seconds();
    }
}

fn update_displayed_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &PhysicalTranslation)>,
) {
    for (mut transform, physical_translation) in query.iter_mut() {
        let last_displayed_translation = transform.translation;
        // The overstep fraction is a value between 0 and 1 that tells us how far we are
        // between two fixed timesteps.
        let alpha = fixed_time.overstep_fraction();

        let next_displayed_translation =
            last_displayed_translation.lerp(physical_translation.0, alpha);

        transform.translation = next_displayed_translation;
    }
}
