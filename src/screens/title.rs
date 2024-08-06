//! The title screen that appears when the game starts.

use bevy::prelude::*;

use super::Screen;
use crate::{spawn::prelude::*, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_title_screen);
}

fn spawn_title_screen(mut commands: Commands) {
    commands.spawn_with(title_screen);
}

fn title_screen(id: Entity, world: &mut World) {
    let enter_playing = world.register_system(enter_playing);
    let enter_credits = world.register_system(enter_credits);
    #[cfg(not(target_family = "wasm"))]
    let exit_app = world.register_system(exit_app);

    world
        .entity_mut(id)
        .add(widget::ui_root)
        .insert((Name::new("Title screen"), StateScoped(Screen::Title)))
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
