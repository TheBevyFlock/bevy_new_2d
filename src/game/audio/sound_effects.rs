//! Functions and types for playing sound effects in the game.
//! The main thing of interest here is the extension trait `SoundEffectCommands`
//! and its `play_sound_effect` method.
//!
//! This pattern is adapted from the [Bevy example for sound effects](https://github.com/bevyengine/bevy/pull/14554).

use bevy::{ecs::world::Command, prelude::*};
use rand::seq::SliceRandom;

use crate::game::assets::SoundEffectHandles;

pub(super) fn plugin(_app: &mut App) {
    // No setup required for this plugin.
    // It's still good to have a function here so that we can add some setup later if needed.
}

impl SoundEffectHandles {
    /// Plays a random sound effect matching the given name.
    ///
    /// When defining the settings for this method, we almost always want to use [`PlaybackMode::Despawn`](bevy::audio::PlaybackMode).
    /// Every time a sound effect is played, a new entity is generated. Once the sound effect is complete,
    /// the entity should be cleaned up, rather than looping or sitting around uselessly.
    ///
    /// This method accepts any type which implements `AsRef<str>`.
    /// This allows us to pass in `&str`, `String`, or a custom type that can be converted to a string.
    ///
    /// These custom types can be useful for defining enums that represent specific sound effects.
    /// Generally speaking, enum values should be used to represent one-off or special-cased sound effects,
    /// while string keys are a better fit for sound effects corresponding to objects loaded from a data file.
    fn play(&mut self, name: impl AsRef<str>, world: &mut World, settings: PlaybackSettings) {
        let name = name.as_ref();
        if let Some(sfx_list) = self.get_mut(name) {
            // If we need precise control over the randomization order of our sound effects,
            // we can store the RNG as a resource and modify these functions to take it as an argument.
            let rng = &mut rand::thread_rng();
            let random_sfx = sfx_list.choose(rng).unwrap();

            // We don't need a (slightly) more expensive strong handle here (which are used to keep assets loaded in memory)
            // because a copy is always stored in the SoundEffects resource.
            let source = random_sfx.clone_weak();

            world.spawn(AudioBundle { source, settings });
        } else {
            warn!("Sound effect not found: {name}");
        }
    }
}

/// A custom command used to play sound effects.
struct PlaySoundEffect {
    name: String,
    settings: PlaybackSettings,
}

impl Command for PlaySoundEffect {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, mut sound_effects: Mut<SoundEffectHandles>| {
            sound_effects.play(self.name, world, self.settings);
        });
    }
}

/// An extension trait used to make it convenient to play sound effects via [`Commands`].
pub trait SoundEffectCommands {
    fn play_sound_effect_with_settings(
        &mut self,
        name: impl Into<String>,
        settings: PlaybackSettings,
    );

    fn play_sound_effect(&mut self, name: impl Into<String>) {
        self.play_sound_effect_with_settings(name, PlaybackSettings::DESPAWN);
    }
}

impl SoundEffectCommands for Commands<'_, '_> {
    // By accepting an `Into<String>` here, we can be flexible about what we want to accept:
    // &str literals are better for prototyping and data-driven sound effects,
    // but enums are nicer for special-cased effects
    fn play_sound_effect_with_settings(
        &mut self,
        name: impl Into<String>,
        settings: PlaybackSettings,
    ) {
        let name = name.into();
        self.add(PlaySoundEffect { name, settings });
    }
}
