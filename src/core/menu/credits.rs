use bevy::prelude::*;

use super::MenuState;
use crate::ui::*;

pub(super) fn plugin(app: &mut App) {
    // Setup, update, teardown
    app.add_systems(OnEnter(MenuState::Credits), setup);
    app.add_systems(Update, update.run_if(in_state(MenuState::Credits)));
}

fn setup(mut commands: Commands) {
    let list = commands
        .my_root()
        .insert(StateScoped(MenuState::Credits))
        .id();

    commands.my_label("Alice").set_parent(list);
    commands.my_label("Bob").set_parent(list);

    commands.my_button("Back", UiAction).set_parent(list);
}

fn update(
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut interaction_query: ButtonQuery<UiAction>,
) {
    for (interaction, _) in &mut interaction_query {
        if let Interaction::Pressed = interaction {
            next_menu_state.set(MenuState::Main)
        }
    }
}

#[derive(Component, PartialEq, Eq)]
pub struct UiAction;
