use bevy::audio::Volume;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<GlobalMusicVolumeScale>();
    app.register_type::<GlobalMusicVolumeScale>();
    app.add_systems(
        PostUpdate,
        (
            scale_mutated_music,
            update_music_for_new_global_scale.run_if(resource_changed::<GlobalMusicVolumeScale>),
        )
            .chain(),
    );
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it is in the
/// general "music" category (ex: global background music, soundtrack, etc).
///
/// This can then be used to query for and operate on sounds in that category. For example:
///
/// ```
/// use bevy::{audio::Volume, prelude::*};
/// use bevy_new_2d::audio::Music;
///
/// fn set_music_volume(mut sink_query: Query<&mut AudioSink, With<Music>>) {
///     for mut sink in &mut sink_query {
///         sink.set_volume(Volume::Linear(0.5));
///     }
/// }
/// ```
#[derive(Component, Debug, Default)]
#[require(UnscaledVolume)]
pub struct Music;

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it is in the
/// general "sound effect" category (ex: footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category. For example:
///
/// ```
/// use bevy::{audio::Volume, prelude::*};
/// use bevy_new_2d::audio::SoundEffect;
///
/// fn set_sound_effect_volume(mut sink_query: Query<&mut AudioSink, With<SoundEffect>>) {
///     for mut sink in &mut sink_query {
///         sink.set_volume(Volume::Linear(0.5));
///     }
/// }
/// ```
#[derive(Component, Debug, Default)]
pub struct SoundEffect;

#[derive(Resource, Debug, Reflect, Deref, DerefMut, Default)]
#[reflect(Resource)]
pub struct GlobalMusicVolumeScale(pub Volume);
#[derive(Component, Debug, Reflect, Deref, DerefMut, Default)]
#[reflect(Component)]
pub struct UnscaledVolume(pub Volume);

fn scale_mutated_music(
    mut sinks: Query<(Mut<AudioSink>, &mut UnscaledVolume), (Changed<AudioSink>, With<Music>)>,
    music_volume_scale: Res<GlobalMusicVolumeScale>,
) {
    for (mut sink, mut unscaled_volume) in &mut sinks {
        let sink = sink.bypass_change_detection();
        unscaled_volume.0 = sink.volume();
        sink.set_volume(music_volume_scale.0 * unscaled_volume.0);
    }
}

fn update_music_for_new_global_scale(
    music_volume_factor: Res<GlobalMusicVolumeScale>,
    mut sinks: Query<(&mut AudioSink, &UnscaledVolume), With<Music>>,
) {
    for (mut sink, unscaled_volume) in &mut sinks {
        let sink = sink.bypass_change_detection();
        sink.set_volume(music_volume_factor.0 * unscaled_volume.0);
    }
}
