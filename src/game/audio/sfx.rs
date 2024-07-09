use bevy::{audio::PlaybackMode, prelude::*};
use rand::prelude::SliceRandom;

pub(super) fn play_sfx(
    trigger: Trigger<Sfx>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let event = trigger.event();
    let path = match event {
        Sfx::ButtonHover => "audio/sfx/button_hover.ogg",
        Sfx::ButtonPress => "audio/sfx/button_press.ogg",
        Sfx::Step => random_step(),
    };
    let source = asset_server.load::<AudioSource>(path);
    let settings = PlaybackSettings {
        mode: PlaybackMode::Despawn,
        ..default()
    };
    commands.spawn(AudioSourceBundle { source, settings });
}

/// Play an a single sound effect.
#[derive(Event)]
pub enum Sfx {
    ButtonHover,
    ButtonPress,
    Step,
}

fn random_step() -> &'static str {
    [
        "audio/sfx/step1.ogg",
        "audio/sfx/step2.ogg",
        "audio/sfx/step3.ogg",
        "audio/sfx/step4.ogg",
    ]
    .choose(&mut rand::thread_rng())
    .unwrap()
}
