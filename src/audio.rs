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

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Default)]
pub struct Music;

/// A music audio instance.
pub fn music(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::LOOP, Music)
}

/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Default)]
pub struct SoundEffect;

/// A sound effect audio instance.
pub fn sound_effect(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::DESPAWN, SoundEffect)
}

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
