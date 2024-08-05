//! The title screen that appears when the game starts.

use bevy::prelude::*;

use super::Screen;
use crate::ui::{interaction::OnPress, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), show_title_screen);
}

fn show_title_screen(mut commands: Commands) {
    let on_play = commands.register_one_shot_system(on_play);
    let on_credits = commands.register_one_shot_system(on_credits);
    let on_exit = commands.register_one_shot_system(on_exit);

    commands
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children.button("Play").insert(OnPress(on_play));
            children.button("Credits").insert(OnPress(on_credits));

            #[cfg(not(target_family = "wasm"))]
            children.button("Exit").insert(OnPress(on_exit));
        });
}

fn on_play(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Playing);
}

fn on_credits(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn on_exit(mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}
