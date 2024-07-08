//! Handle how the physically simulated world should be rendered.

use bevy::prelude::*;

use super::physics::{PhysicalTransform, PreviousPhysicalTransform};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_rendered_transform);
}

/// Interpolate between the previous and current physical translation to get a
/// smooth visual representation of the player. This is a tradeoff, as it will
/// also make visual representation lag slightly behind the actual physics simulation.
fn update_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &PhysicalTransform,
        &PreviousPhysicalTransform,
    )>,
) {
    for (mut transform, current_physical_transform, previous_physical_transform) in query.iter_mut()
    {
        let previous = previous_physical_transform.translation;
        let current = current_physical_transform.translation;
        // The overstep fraction is a value between 0 and 1 that tells us how far we are
        // between two fixed timesteps.
        let alpha = fixed_time.overstep_fraction();

        let rendered_translation = previous.lerp(current, alpha);
        transform.translation = rendered_translation;
    }
}
