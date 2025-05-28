//! The screen state for the main gameplay.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{demo::level::spawn_level, menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_level);
    app.add_systems(OnExit(Screen::Gameplay), close_menu);

    // Toggle pause menu on key press.
    app.add_systems(
        Update,
        (
            open_pause_menu.run_if(
                in_state(Screen::Gameplay)
                    .and(in_state(Menu::None))
                    .and(input_just_pressed(KeyCode::Escape).or(input_just_pressed(KeyCode::KeyP))),
            ),
            close_menu.run_if(in_state(Screen::Gameplay).and(input_just_pressed(KeyCode::KeyP))),
        ),
    );
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
