//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::Screen;
use crate::{assets::BgmHandles, audio::bgm::BgmCommands as _, demo::level::SpawnLevel};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn_level);
    app.add_systems(OnExit(Screen::Playing), stop_bgm);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_level(mut commands: Commands) {
    commands.add(SpawnLevel);
    commands.play_bgm(BgmHandles::PATH_GAMEPLAY);
}

fn stop_bgm(mut commands: Commands) {
    commands.stop_bgm();
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
