//! The title screen that appears when the game starts.

use bevy::prelude::*;

use crate::{screens::Screen, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_title_screen);
}

fn spawn_title_screen(mut commands: Commands) {
    commands
        .spawn((widget::ui_root("Title Screen"), StateScoped(Screen::Title)))
        .with_children(|parent| {
            parent
                .spawn(widget::button("Play"))
                .observe(enter_gameplay_screen);
            parent
                .spawn(widget::button("Credits"))
                .observe(enter_credits_screen);

            #[cfg(not(target_family = "wasm"))]
            parent.spawn(widget::button("Exit")).observe(exit_app);
        });
}

fn enter_gameplay_screen(
    _: Trigger<Pointer<Released>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_screen.set(Screen::Gameplay);
}

fn enter_credits_screen(_: Trigger<Pointer<Released>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<Pointer<Released>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
