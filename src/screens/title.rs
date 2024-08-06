//! The title screen that appears when the game starts.

use bevy::prelude::*;

use super::Screen;
use crate::ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), show_title_screen);
}

fn show_title_screen(mut commands: Commands) {
    let enter_playing = commands.register_one_shot_system(enter_playing);
    let enter_credits = commands.register_one_shot_system(enter_credits);
    #[cfg(not(target_family = "wasm"))]
    let exit_app = commands.register_one_shot_system(exit_app);

    commands
        .ui_root()
        .insert(StateScoped(Screen::Title))
        .with_children(|children| {
            children.button("Play", enter_playing);
            children.button("Credits", enter_credits);

            #[cfg(not(target_family = "wasm"))]
            children.button("Exit", exit_app);
        });
}

fn enter_playing(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Playing);
}

fn enter_credits(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}
