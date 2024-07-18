use bevy::{audio::PlaybackMode, prelude::*};
use rand::seq::SliceRandom;

use crate::game::assets::{HandleMap, SfxKey};

pub(super) fn play_sfx(
    trigger: Trigger<Sfx>,
    mut commands: Commands,
    sfx_handles: Res<HandleMap<SfxKey>>,
) {
    let event = trigger.event();
    let source = match event {
        Sfx::ButtonHover => &sfx_handles[&SfxKey::ButtonHover],
        Sfx::ButtonPress => &sfx_handles[&SfxKey::ButtonPress],
        Sfx::Step => random_step(&sfx_handles),
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

fn random_step(sfx_handles: &HandleMap<SfxKey>) -> &Handle<AudioSource> {
    [
        &sfx_handles[&SfxKey::Step1],
        &sfx_handles[&SfxKey::Step2],
        &sfx_handles[&SfxKey::Step3],
        &sfx_handles[&SfxKey::Step4],
    ]
    .choose(&mut rand::thread_rng())
    .unwrap()
}
