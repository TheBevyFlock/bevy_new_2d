//! The pause menu.

use bevy::{input::common_conditions::input_just_pressed, prelude::*, ui_widgets::Activate};

use crate::{menus::Menu, screens::Screen, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_pause_menu(mut commands: Commands) {
    let bundle = (
        widget::ui_root("Pause Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Pause),
        children![
            widget::header("Game paused"),
            widget::button("Continue", commands.register_system(close_menu)),
            widget::button("Settings", commands.register_system(open_settings_menu)),
            widget::button("Quit to title", commands.register_system(quit_to_title)),
        ],
    );
    commands.spawn(bundle);
}

fn open_settings_menu(_: In<Activate>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn close_menu(_: In<Activate>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn quit_to_title(_: In<Activate>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
