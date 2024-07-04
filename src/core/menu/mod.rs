mod credits;

use bevy::prelude::*;

use super::CoreState;
use crate::ui::*;

pub(super) fn plugin(app: &mut App) {
    // Setup state
    app.add_sub_state::<MenuState>();
    app.enable_state_scoped_entities::<MenuState>();

    // Setup, update, teardown
    app.add_systems(OnEnter(MenuState::Main), setup);
    app.add_systems(Update, update.run_if(in_state(MenuState::Main)));

    // Sub states
    app.add_plugins(credits::plugin);
}

#[derive(SubStates, Debug, PartialEq, Hash, Eq, Clone, Default)]
#[source(CoreState = CoreState::Menu)]
enum MenuState {
    #[default]
    Main,
    Credits,
}

fn setup(mut commands: Commands) {
    let list = commands.my_root().insert(StateScoped(MenuState::Main)).id();

    commands.my_button("Play", UiAction::Play).set_parent(list);
    commands
        .my_button("Credits", UiAction::Credits)
        .set_parent(list);
}

fn update(
    mut next_core_state: ResMut<NextState<CoreState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut interactions_query: ButtonQuery<UiAction>,
) {
    for (interaction, button) in &mut interactions_query {
        if let Interaction::Pressed = interaction {
            match button {
                UiAction::Play => next_core_state.set(CoreState::Game),
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
