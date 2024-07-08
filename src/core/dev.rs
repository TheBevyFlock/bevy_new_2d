use bevy::{dev_tools::states::log_transitions, prelude::*};

use crate::screen::Screen;

use super::booting::Booting;

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_systems(
        Update,
        (log_transitions::<Booting>, log_transitions::<Screen>).chain(),
    );
}
