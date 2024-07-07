use bevy::prelude::*;
use super::MenuState;
use crate::ui::*;

pub(super) fn plugin(app: &mut App) {
    // Setup(s), update(s), teardown(s)
    app.add_systems(OnEnter(MenuState::Credits), setup);
    app.add_systems(Update, update.run_if(in_state(MenuState::Credits)));
}

fn setup(mut commands: Commands) {
    let list = commands
        .my_root()
        .insert(StateScoped(MenuState::Credits))
        .id();

    commands.my_label("Alice - Foo").set_parent(list);
    commands.my_label("Bob - Bar").set_parent(list);

    commands
        .my_button("Back")
        .insert(UiAction::Back)
        .set_parent(list);
}

fn update(
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut interaction_query: ButtonInteractionQuery<UiAction>,
) {
    for (interaction, _) in &mut interaction_query {
        if matches!(interaction, Interaction::Pressed) {
            next_menu_state.set(MenuState::Main)
        }
    }
}

#[derive(Component, PartialEq, Eq)]
enum UiAction {
    Back,
}
