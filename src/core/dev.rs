use bevy::{dev_tools::states::log_transitions, prelude::*};

use super::{menu::MenuState, RunningState};
use crate::AppState;

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_systems(
        Update,
        (
            log_transitions::<AppState>,
            log_transitions::<RunningState>,
            log_transitions::<MenuState>,
        )
            .chain(),
    );
}
