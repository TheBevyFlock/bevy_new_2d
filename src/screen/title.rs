use bevy::prelude::*;

use super::Screen;
use crate::util::ui::*;

pub(super) fn plugin(app: &mut App) {
    // Screen setup and teardown.
    app.add_systems(OnEnter(Screen::Title), enter_title)
        .add_systems(OnExit(Screen::Title), exit_title);

    app.add_systems(Update, handle_title_action.run_if(in_state(Screen::Title)));
}

#[derive(Component, PartialEq, Eq)]
enum TitleAction {
    Play,
    Credits,
}

fn enter_title(mut commands: Commands) {
    let list = commands.my_root().insert(StateScoped(Screen::Title)).id();

    commands
        .my_button("Play")
        .insert(TitleAction::Play)
        .set_parent(list);
    commands
        .my_button("Credits")
        .insert(TitleAction::Credits)
        .set_parent(list);
}

fn exit_title() {}

fn handle_title_action(
    mut next_screen: ResMut<NextState<Screen>>,
    mut button_query: ButtonInteractionQuery<TitleAction>,
) {
    for (interaction, action) in &mut button_query {
        if matches!(interaction, Interaction::Pressed) {
            match action {
                TitleAction::Play => next_screen.set(Screen::Playing),
                TitleAction::Credits => next_screen.set(Screen::Credits),
            }
        }
    }
}