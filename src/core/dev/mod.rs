use crate::AppState;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Print state transitions in dev builds
    app.add_systems(Update, bevy::dev_tools::states::log_transitions::<AppState>);
}
