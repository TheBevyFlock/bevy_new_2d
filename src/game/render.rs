//! Handle how the physically simulated world should be rendered.

use bevy::prelude::*;

use super::physics::PhysicalTranslation;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_rendered_transform);
}

fn update_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &PhysicalTranslation)>,
) {
    for (mut transform, physical_translation) in query.iter_mut() {
        let last_rendered_translation = transform.translation;
        // The overstep fraction is a value between 0 and 1 that tells us how far we are
        // between two fixed timesteps.
        let alpha = fixed_time.overstep_fraction();

        let next_rendered_translation =
            last_rendered_translation.lerp(physical_translation.0, alpha);

        transform.translation = next_rendered_translation;
    }
}
