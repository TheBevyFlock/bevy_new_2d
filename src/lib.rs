mod bevy_setup;
mod core;
#[cfg(feature = "dev")]
mod dev;
mod ui;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // State setup
        app.init_state::<AppState>()
            // Add state scoped entities for UI cleanup
            .enable_state_scoped_entities::<AppState>()
            // Sub-plugins
            .add_plugins((bevy_setup::plugin, core::plugin, ui::plugin));

        #[cfg(feature = "dev")]
        app.add_plugins(dev::plugin);
    }
}

/// Root state of the application.
#[derive(States, Debug, PartialEq, Eq, Hash, Clone, Default)]
enum AppState {
    #[default]
    Loading,
    Ready,
}
