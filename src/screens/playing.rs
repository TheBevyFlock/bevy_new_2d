//! A playing screen where the game is actually played.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::Screen;
use crate::{
    assets::SoundtrackHandles, audio::soundtrack::SoundtrackCommands as _, demo::level::level,
    spawn::prelude::*, theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Playing),
        (spawn_playing_screen, spawn_level, play_gameplay_soundtrack),
    );
    app.add_systems(OnExit(Screen::Playing), stop_soundtrack);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_playing_screen(mut commands: Commands) {
    commands.spawn_fn(playing_screen);
}

fn playing_screen(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .add_fn(widget::ui_root)
        .insert((Name::new("Playing screen"), StateScoped(Screen::Playing)))
        .with_children(|_children| {
            // Spawn playing screen UI here (e.g. a HUD).
        });
}

fn spawn_level(mut commands: Commands) {
    commands
        .spawn_fn(level)
        .insert(StateScoped(Screen::Playing));
}

fn play_gameplay_soundtrack(mut commands: Commands) {
    commands.play_soundtrack(SoundtrackHandles::KEY_GAMEPLAY);
}

fn stop_soundtrack(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entities instead.
    commands.stop_current_soundtrack();
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
