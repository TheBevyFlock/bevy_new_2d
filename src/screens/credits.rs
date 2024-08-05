//! A credits screen that can be accessed from the title screen.

use bevy::prelude::*;

use super::Screen;
use crate::{
    game::{assets::SoundtrackHandles, audio::soundtrack::PlaySoundtrack},
    ui::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Credits), show_credits_screen);
    app.add_systems(OnExit(Screen::Credits), disable_soundtrack);
}

fn show_credits_screen(mut commands: Commands) {
    let enter_title = commands.register_one_shot_system(enter_title);

    commands
        .ui_root()
        .insert(StateScoped(Screen::Credits))
        .with_children(|children| {
            children.header("Made by");
            children.label("Joe Shmoe - Implemented aligator wrestling AI");
            children.label("Jane Doe - Made the music for the alien invasion");

            children.header("Assets");
            children.label("Bevy logo - All rights reserved by the Bevy Foundation. Permission granted for splash screen use when unmodified.");
            children.label("Ducky sprite - CC0 by Caz Creates Games");
            children.label("Music - CC BY 3.0 by Kevin MacLeod");

            children.button("Back").insert(OnPress(enter_title));
        });

    commands.trigger(PlaySoundtrack::Key(SoundtrackHandles::CREDITS.to_string()));
}

fn disable_soundtrack(mut commands: Commands) {
    commands.trigger(PlaySoundtrack::Disable);
}

fn enter_title(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
