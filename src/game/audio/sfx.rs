use bevy::{audio::PlaybackMode, prelude::*};
use rand::prelude::SliceRandom;

use crate::game::assets::{SfxAsset, SfxAssets};

pub(super) fn play_sfx(trigger: Trigger<Sfx>, mut commands: Commands, sfxs: Res<SfxAssets>) {
    let event = trigger.event();
    let source = match event {
        Sfx::ButtonHover => &sfxs[&SfxAsset::ButtonHover],
        Sfx::ButtonPress => &sfxs[&SfxAsset::ButtonPress],
        Sfx::Step => random_step(&sfxs),
    }
    .clone_weak();
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
    [
        &sfxs[&SfxAsset::Step1],
        &sfxs[&SfxAsset::Step2],
        &sfxs[&SfxAsset::Step3],
        &sfxs[&SfxAsset::Step4],
    ]
    .choose(&mut rand::thread_rng())
    .unwrap()
}
