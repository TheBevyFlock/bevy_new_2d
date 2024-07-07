mod credits;

use bevy::prelude::*;

use super::RunningState;
use crate::ui::*;

pub(super) fn plugin(app: &mut App) {
    // State setup
    app.add_sub_state::<MenuState>();
    // Add state scoped entities for UI cleanup
    app.enable_state_scoped_entities::<MenuState>();
    // Print state transitions in debug builds
    #[cfg(feature = "dev")]
    app.add_systems(
        Update,
        bevy::dev_tools::states::log_transitions::<MenuState>,
    );

    // Setup(s), update(s), teardown(s)
    app.add_systems(OnEnter(MenuState::Main), setup);
    app.add_systems(Update, update.run_if(in_state(MenuState::Main)));

    // Sub plugins
    app.add_plugins(credits::plugin);
}

#[derive(SubStates, Debug, PartialEq, Hash, Eq, Clone, Default)]
#[source(RunningState = RunningState::Menu)]
enum MenuState {
    #[default]
    Main,
    Credits,
}

fn setup(mut commands: Commands) {
    let list = commands.my_root().insert(StateScoped(MenuState::Main)).id();

    commands
        .my_button("Play")
        .insert(UiAction::Play)
        .set_parent(list);
    commands
        .my_button("Credits")
        .insert(UiAction::Credits)
        .set_parent(list);
}

fn update(
    mut next_core_state: ResMut<NextState<RunningState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut interactions_query: ButtonInteractionQuery<UiAction>,
) {
    for (interaction, button) in &mut interactions_query {
        if matches!(interaction, Interaction::Pressed) {
            match button {
                UiAction::Play => next_core_state.set(RunningState::Game),
                UiAction::Credits => next_menu_state.set(MenuState::Credits),
            }
        }
    }
}

#[derive(Component, PartialEq, Eq)]
enum UiAction {
    Play,
    Credits,
}
