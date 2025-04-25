use bevy::audio::Volume;
use bevy::ecs::component::{ComponentId, HookContext};
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MusicVolumeFactor>();
    app.register_type::<MusicVolumeFactor>();
    app.add_systems(
        PostUpdate,
        (
            update_unscaled_music_volume,
            scale_music_volume.run_if(resource_changed::<MusicVolumeFactor>),
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
#[component(on_add = init_unscaled_music_volume)]
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
pub struct MusicVolumeFactor(pub Volume);

#[derive(Component, Debug, Reflect, Deref, DerefMut, Default)]
#[reflect(Component)]
pub struct UnscaledVolume(pub Volume);

fn update_unscaled_music_volume(
    mut sinks: Query<(&AudioSink, &mut UnscaledVolume), (Changed<AudioSink>, With<Music>)>,
) {
    for (sink, mut unscaled_volume) in &mut sinks {
        unscaled_volume.0 = sink.volume();
    }
}

fn init_unscaled_music_volume(mut world: DeferredWorld, ctx: HookContext) {
    let entity = ctx.entity;
    let Some(volume) = world.get::<AudioSink>(entity).map(|sink| sink.volume()) else {
        error!("Music marker was added to an entity without an AudioSink");
        return;
    };
    {
        // UnscaledMusicVolume is required, so we can safely unwrap
        let mut unscaled_volume = world.get_mut::<UnscaledVolume>(entity).unwrap();
        unscaled_volume.0 = volume;
    }
    let scale = world.resource::<MusicVolumeFactor>().0;
    let mut sink = world.get_mut::<AudioSink>(entity).unwrap();
    sink.set_volume(scale * volume);
}

fn scale_music_volume(
    music_volume_factor: Res<MusicVolumeFactor>,
    mut sinks: Query<(&mut AudioSink, &UnscaledVolume), With<Music>>,
) {
    for (mut sink, unscaled_volume) in &mut sinks {
        sink.set_volume(music_volume_factor.0 * unscaled_volume.0);
    }
}
