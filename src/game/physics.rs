//! Run a very simple physics simulation.

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // `FixedUpdate` runs before `Update`, so the physics simulation is advanced
    // before the player's visual representation is updated.
    app.add_systems(FixedUpdate, advance_physics);
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
