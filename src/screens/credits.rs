//! A credits screen that can be accessed from the title screen.

use bevy::prelude::*;

use super::Screen;
use crate::{
    assets::SoundtrackHandles, audio::soundtrack::SoundtrackCommands as _, spawn::prelude::*,
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Credits),
        (spawn_credits_screen, play_credits_soundtrack),
    );
    app.add_systems(OnExit(Screen::Credits), stop_soundtrack);
}

fn spawn_credits_screen(mut commands: Commands) {
    commands.spawn_fn(credits_screen);
}

fn credits_screen(In(id): In<Entity>, mut commands: Commands) {
    let enter_title = commands.register_one_shot_system(enter_title);

    commands.entity(id).add_fn(widget::ui_root)
        .insert((Name::new("Credits screen"), StateScoped(Screen::Credits)))
        .with_children(|children| {
            children.header("Made by");
            children.label("Joe Shmoe - Implemented aligator wrestling AI");
            children.label("Jane Doe - Made the music for the alien invasion");

            children.header("Assets");
            children.label("Bevy logo - All rights reserved by the Bevy Foundation. Permission granted for splash screen use when unmodified.");
            children.label("Ducky sprite - CC0 by Caz Creates Games");
            children.label("Music - CC BY 3.0 by Kevin MacLeod");

            children.button("Back", enter_title);
        });
}

fn play_credits_soundtrack(mut commands: Commands) {
    commands.play_soundtrack(SoundtrackHandles::KEY_CREDITS);
}

fn stop_soundtrack(mut commands: Commands) {
    commands.stop_current_soundtrack();
}

fn enter_title(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
