//! A credits screen that can be accessed from the title screen.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::Screen;
use crate::util::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Credits), enter_credits);

    app.add_systems(
        Update,
        handle_credits_action.run_if(in_state(Screen::Credits)),
    );

    app.register_type::<CreditsAction>();
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Component, Debug, PartialEq, Hash, Serialize, Deserialize)]
enum CreditsAction {
    Back,
}

fn enter_credits(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::Credits))
        .with_children(|children| {
            children.label("Alice - Foo");
            children.label("Bob - Bar");

            children.button("Back").insert(CreditsAction::Back);
        });
}

fn handle_credits_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: InteractionQuery<&CreditsAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                CreditsAction::Back => next_screen.set(Screen::Title),
            }
        }
    }
}
