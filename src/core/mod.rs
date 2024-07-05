mod game;
mod menu;

pub use bevy::prelude::*;

use crate::ui;

pub(super) fn plugin(app: &mut App) {
    // State setup
    app.init_state::<CoreState>();
    // Add state scoped entities for UI cleanup
    app.enable_state_scoped_entities::<CoreState>();
    // Print state transitions in dev builds
    #[cfg(feature = "dev")]
    app.add_systems(
        Update,
        bevy::dev_tools::states::log_transitions::<CoreState>,
    );

    // Setup, update, teardown
    app.add_systems(Startup, setup);

    // Sub plugins
    app.add_plugins((menu::plugin, game::plugin));

    // Other
    app.add_plugins(ui::plugin);

    // For a larger UI example visit: https://github.com/MiniaczQ/bevy-substate-project
}

/// Root state of the entire game.
#[derive(States, Debug, PartialEq, Hash, Eq, Clone, Default)]
enum CoreState {
    #[default]
    Menu,
    Game,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
