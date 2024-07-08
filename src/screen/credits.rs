//! A credits screen that can be accessed from the title screen.

use bevy::prelude::*;

use super::Screen;
use crate::util::ui::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Credits), enter_credits);

    app.add_systems(
        Update,
        handle_credits_action.run_if(in_state(Screen::Credits)),
    );
}

#[derive(Component, PartialEq, Eq)]
enum CreditsAction {
    Back,
}

fn enter_credits(mut commands: Commands) {
    commands.my_root(|_| {});
    commands
        .my_root(|children| {
            children.my_label("Alice - Foo");
            children.my_label("Bob - Bar");
            
            children.my_button("Back").insert(CreditsAction::Back);
        })
        .insert(StateScoped(Screen::Credits));
}

fn handle_credits_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: ButtonInteractionQuery<CreditsAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                CreditsAction::Back => next_screen.set(Screen::Title),
            }
        }
    }
}
