use bevy::{audio::PlaybackMode, prelude::*};

use crate::game::assets::{HandleMap, SoundtrackKey};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<IsSoundtrack>();
    app.observe(play_soundtrack);
}

fn play_soundtrack(
    trigger: Trigger<PlaySoundtrack>,
    mut commands: Commands,
    soundtrack_handles: Res<HandleMap<SoundtrackKey>>,
    soundtrack_query: Query<Entity, With<IsSoundtrack>>,
) {
    for entity in &soundtrack_query {
        commands.entity(entity).despawn_recursive();
    }

    commands.spawn((
        AudioSourceBundle {
            source: soundtrack_handles[&match trigger.event() {
                PlaySoundtrack::Disable => return,
                PlaySoundtrack::Credits => SoundtrackKey::Credits,
                PlaySoundtrack::Gameplay => SoundtrackKey::Gameplay,
            }]
                .clone_weak(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        IsSoundtrack,
    ));
}

/// Trigger this event to play or disable the soundtrack.
/// Playing a new soundtrack will overwrite the previous one.
/// Soundtracks will loop.
#[derive(Event)]
pub enum PlaySoundtrack {
    Disable,
    Credits,
    Gameplay,
}

/// Marker component for the soundtrack entity so we can find it later.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct IsSoundtrack;
