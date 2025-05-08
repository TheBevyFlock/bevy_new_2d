//! The screen state for the main gameplay.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{demo::level::spawn_level, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Gameplay).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
