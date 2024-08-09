//! Functions and types for playing sound effects in the game.
//! The main point of interest here is the extension trait `SfxCommands`
//! and its `play_sfx` method.
//!
//! This method accepts any type which implements `Into<String>`.
//! This allows us to pass in `&str`, `String`, or a custom type that can be
//! converted to a string.
//!
//! These custom types can be useful for defining enums that represent specific
//! sound effects. Generally speaking, enum values should be used to represent
//! one-off or special-cased sound effects, while string keys are a better fit
//! for sound effects corresponding to objects loaded from a data file.
//!
//! This pattern is taken from the [Bevy example for sound effects](https://github.com/bevyengine/bevy/pull/14554).

use bevy::{ecs::world::Command, prelude::*};
use rand::seq::SliceRandom;

use crate::assets::SfxHandles;

pub(super) fn plugin(_app: &mut App) {
    // No setup required for this plugin.
    // It's still good to have a function here so that we can add some setup
    // later if needed.
}

impl SfxHandles {
    /// Plays a random sound effect matching the given name.
    ///
    /// When defining the settings for this method, we almost always want to use
    /// [`PlaybackMode::Despawn`](bevy::audio::PlaybackMode). Every time a
    /// sound effect is played, a new entity is generated. Once the sound effect
    /// is complete, the entity should be cleaned up, rather than looping or
    /// sitting around uselessly.
    fn play(&mut self, name: impl AsRef<str>, world: &mut World, settings: PlaybackSettings) {
        let name = name.as_ref();
        if let Some(sfx_list) = self.get_mut(name) {
            // If we need precise control over the randomization order of our sound effects,
            // we can store the RNG as a resource and modify these functions to take it as
            // an argument.
            let rng = &mut rand::thread_rng();
            let random_sfx = sfx_list.choose(rng).unwrap();

            // We don't need a (slightly) more expensive strong handle here (which is used
            // to keep an asset loaded in memory), because a copy is always
            // stored in the `SfxHandles` resource.
            let source = random_sfx.clone_weak();

            world.spawn(AudioBundle { source, settings });
        } else {
            warn!("Sound effect not found: {name}");
        }
    }
}

/// A custom command used to play sound effects.
struct PlaySfx {
    name: String,
    settings: PlaybackSettings,
}

impl Command for PlaySfx {
    fn apply(self, world: &mut World) {
        // If you need more complex behavior, use `world.run_system_once_with`,
        // as demonstrated with `PlayBgm`.
        world.resource_scope(|world, mut sfx: Mut<SfxHandles>| {
            sfx.play(self.name, world, self.settings);
        });
    }
}

/// An extension trait with convenience methods for sound effect commands.
pub trait SfxCommands {
    fn play_sfx_with_settings(&mut self, name: impl Into<String>, settings: PlaybackSettings);

    fn play_sfx(&mut self, name: impl Into<String>) {
        self.play_sfx_with_settings(name, PlaybackSettings::DESPAWN);
    }
}

impl SfxCommands for Commands<'_, '_> {
    // By accepting an `Into<String>` here, we can be flexible about what we want to
    // accept: &str literals are better for prototyping and data-driven sound
    // effects, but enums are nicer for special-cased effects
    fn play_sfx_with_settings(&mut self, name: impl Into<String>, settings: PlaybackSettings) {
        let name = name.into();
        self.add(PlaySfx { name, settings });
    }
}
