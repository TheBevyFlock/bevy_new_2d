mod game;
mod menu;

pub use bevy::prelude::*;

use crate::ui;

pub fn plugin(app: &mut App) {
    // Setup state
    app.init_state::<CoreState>();
    // Add state scoped entities for UI cleanup
    app.enable_state_scoped_entities::<CoreState>();

    // Setup, update, teardown
    app.add_systems(Startup, setup);

    // Sub plugins
    app.add_plugins(ui::MouseHoverPlugin);
    app.add_plugins((menu::plugin, game::plugin));
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
