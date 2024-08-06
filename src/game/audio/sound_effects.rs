use bevy::{ecs::world::Command, prelude::*};
use rand::{distributions::Uniform, Rng as _};

use crate::game::assets::SoundEffectHandles;

pub(super) fn plugin(_app: &mut App) {
    // No setup required for this plugin.
    // It's still good to have a function here so that you can add some setup later if needed.
}

impl SoundEffectHandles {
    /// Plays a random sound effect matching the given name.
    ///
    /// When defining the settings for this method, you almost always want to use [`PlaybackMode::Despawn`](bevy::audio::PlaybackMode).
    /// Every time a sound effect is played, a new entity is generated. Once the sound effect is complete,
    /// the entity should be cleaned up, rather than looping or sitting around uselessly.
    ///
    /// This method accepts any type which implements `AsRef<str>`.
    /// This allows you to pass in `&str`, `String`, or a custom type that can be converted to a string.
    ///
    /// These custom types can be useful for defining enums that represent specific sound effects.
    /// Generally speaking, enum values should be used to represent one-off or special-cased sound effects,
    /// while string keys are a better fit for sound effects corresponding to objects loaded from a data file.
    fn play(&mut self, name: impl AsRef<str>, world: &mut World, settings: PlaybackSettings) {
        let name = name.as_ref();
        if let Some(sfx_list) = self.get_mut(name) {
            // If you need precise control over the randomization order of your sound effects,
            // store the RNG as a resource and modify these functions to take it as an argument.
            let rng = &mut rand::thread_rng();

            let index = rng.sample(Uniform::from(0..sfx_list.len()));
            // We don't need a (slightly) more expensive strong handle here (which are used to keep assets loaded in memory)
            // because a copy is always stored in the SoundEffects resource.
            let source = sfx_list[index].clone_weak();

            world.spawn(AudioBundle {
                source,
                // We want the sound effect to play once and then despawn.
                settings,
            });
        } else {
            warn!("Sound effect not found: {}", name);
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
        // Access both the world and the resource we need from it using resource_scope
        // which temporarily removes the SoundEffects resource from the world
        world.resource_scope(|world, mut sound_effects: Mut<SoundEffectHandles>| {
            sound_effects.play(self.name, world, self.settings);
        });
    }
}

/// An "extension trait" used to make it convenient to play sound effects via [`Commands`].
///
/// This technique allows us to implement methods for types that we don't own,
/// which can be used as long as the trait is in scope.
pub trait SoundEffectCommands {
    fn play_sound_effect_with_settings(
        &mut self,
        name: impl Into<String>,
        settings: PlaybackSettings,
    );

    fn play_sound_effect(&mut self, name: impl Into<String>) {
        // This default method implementation saves work for types implementing this trait;
        // if not overwritten, the trait's default method will be used here, forwarding to the
        // more customizable method
        self.play_sound_effect_with_settings(name, PlaybackSettings::DESPAWN);
    }
}

impl<'w, 's> SoundEffectCommands for Commands<'w, 's> {
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
