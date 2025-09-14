//! The main menu (seen on the title screen).

use bevy::{prelude::*, ui_widgets::Activate};

use crate::{asset_tracking::ResourceHandles, menus::Menu, screens::Screen, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands) {
    let bundle = (
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        #[cfg(not(target_family = "wasm"))]
        children![
            widget::button(
                "Play",
                commands.register_system(enter_loading_or_gameplay_screen)
            ),
            widget::button("Settings", commands.register_system(open_settings_menu)),
            widget::button("Credits", commands.register_system(open_credits_menu)),
            widget::button("Exit", commands.register_system(exit_app)),
        ],
        #[cfg(target_family = "wasm")]
        children![
            widget::button(
                "Play",
                commands.register_system(enter_loading_or_gameplay_screen)
            ),
            widget::button("Settings", commands.register_system(open_settings_menu)),
            widget::button("Credits", commands.register_system(open_credits_menu)),
        ],
    );
    commands.spawn(bundle);
}

fn enter_loading_or_gameplay_screen(
    _: In<Activate>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn open_settings_menu(_: In<Activate>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

fn open_credits_menu(_: In<Activate>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: In<Activate>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
