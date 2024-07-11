use bevy::{audio::PlaybackMode, prelude::*};
use rand::prelude::SliceRandom;

use crate::game::assets::SfxAssets;

pub(super) fn play_sfx(trigger: Trigger<Sfx>, mut commands: Commands, sfxs: Res<SfxAssets>) {
    let event = trigger.event();
    let source = match event {
        Sfx::ButtonHover => sfxs.button_hover.clone(),
        Sfx::ButtonPress => sfxs.button_press.clone(),
        Sfx::Step => random_step(&sfxs).clone(),
    };
    let settings = PlaybackSettings {
        mode: PlaybackMode::Despawn,
        ..default()
    };
    commands.spawn(AudioSourceBundle { source, settings });
}

/// Play a single sound effect.
#[derive(Event)]
pub enum Sfx {
    ButtonHover,
    ButtonPress,
    Step,
}

fn random_step(sfxs: &SfxAssets) -> &Handle<AudioSource> {
    [&sfxs.step1, &sfxs.step2, &sfxs.step3, &sfxs.step4]
        .choose(&mut rand::thread_rng())
        .unwrap()
}
