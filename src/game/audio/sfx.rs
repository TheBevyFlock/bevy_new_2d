use bevy::{audio::PlaybackMode, prelude::*};
use rand::seq::SliceRandom;

use crate::game::assets::{AssetMap, SfxKey};

pub(super) fn play_sfx(
    trigger: Trigger<Sfx>,
    mut commands: Commands,
    sfx_map: Res<AssetMap<SfxKey>>,
) {
    let event = trigger.event();
    let source = match event {
        Sfx::ButtonHover => &sfx_map[&SfxKey::ButtonHover],
        Sfx::ButtonPress => &sfx_map[&SfxKey::ButtonPress],
        Sfx::Step => random_step(&sfx_map),
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

fn random_step(sfx_map: &AssetMap<SfxKey>) -> &Handle<AudioSource> {
    [
        &sfx_map[&SfxKey::Step1],
        &sfx_map[&SfxKey::Step2],
        &sfx_map[&SfxKey::Step3],
        &sfx_map[&SfxKey::Step4],
    ]
    .choose(&mut rand::thread_rng())
    .unwrap()
}
