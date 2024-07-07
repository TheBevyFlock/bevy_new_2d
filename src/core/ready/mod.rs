mod game;
mod menu;

use bevy::prelude::*;

use crate::AppState;

pub(super) fn plugin(app: &mut App) {
    // State setup
    app.add_sub_state::<RunningState>();
    // Add state scoped entities for UI cleanup
    app.enable_state_scoped_entities::<RunningState>();
    // Print state transitions in dev builds
    #[cfg(feature = "dev")]
    app.add_systems(
        Update,
        bevy::dev_tools::states::log_transitions::<RunningState>,
    );

    // Setup(s), update(s), teardown(s)
    app.add_systems(
        OnEnter(AppState::Ready),
        (make_window_visible, setup_camera),
    );

    // Sub plugins
    app.add_plugins((menu::plugin, game::plugin));
}

// TODO: name needs bikeshedding
#[derive(SubStates, Debug, PartialEq, Eq, Hash, Clone, Default)]
#[source(AppState = AppState::Ready)]
enum RunningState {
    #[default]
    Menu,
    Game,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Based on: <https://github.com/bevyengine/bevy/blob/v0.14.0/examples/window/window_settings.rs#L56>
fn make_window_visible(mut window: Query<&mut Window>) {
    window.single_mut().visible = true;
}
