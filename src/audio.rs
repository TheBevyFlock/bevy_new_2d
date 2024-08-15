use bevy::prelude::*;

/// An organizational marker component that should be added to a spawned [`AudioBundle`] if it is in the
/// general "music" category (ex: global background music, soundtrack, etc).
///
/// This can then be used to query for and operate on sounds in that category. For example:
///
/// ```
/// use bevy::prelude::*;
/// use bevy_quickstart::audio::Music;
///
/// fn set_music_volume(sinks: Query<&AudioSink, With<Music>>) {
///     for sink in &sinks {
///         sink.set_volume(0.5);
///     }
/// }
/// ```
#[derive(Component, Default)]
pub struct Music;

/// An organizational marker component that should be added to a spawned [`AudioBundle`] if it is in the
/// general "sound effect" category (ex: footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category. For example:
///
/// ```
/// use bevy::prelude::*;
/// use bevy_quickstart::audio::SoundEffect;
///
/// fn set_sound_effect_volume(sinks: Query<&AudioSink, With<SoundEffect>>) {
///     for sink in &sinks {
///         sink.set_volume(0.5);
///     }
/// }
/// ```
#[derive(Component, Default)]
pub struct SoundEffect;
