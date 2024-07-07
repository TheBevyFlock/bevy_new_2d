mod loading;
mod ready;

pub use bevy::prelude::*;

use crate::ui;

pub(super) fn plugin(app: &mut App) {
    // State setup
    app.init_state::<AppState>();
    // Add state scoped entities for UI cleanup
    app.enable_state_scoped_entities::<AppState>();
    // Print state transitions in dev builds
    #[cfg(feature = "dev")]
    app.add_systems(Update, bevy::dev_tools::states::log_transitions::<AppState>);

    // Sub plugins
    app.add_plugins((loading::plugin, ready::plugin));

    // Other
    app.add_plugins(ui::plugin);

    // For a larger UI example visit: https://github.com/MiniaczQ/bevy-substate-project
}

/// Root state of the application.
#[derive(States, Debug, PartialEq, Eq, Hash, Clone, Default)]
enum AppState {
    #[default]
    Loading,
    Ready,
}
