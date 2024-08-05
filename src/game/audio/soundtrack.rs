use bevy::{audio::PlaybackMode, prelude::*};

use crate::game::assets::SoundtrackHandles;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<IsSoundtrack>();
    app.observe(play_soundtrack);
}

fn play_soundtrack(
    trigger: Trigger<PlaySoundtrack>,
    mut commands: Commands,
    soundtrack_handles: Res<SoundtrackHandles>,
    soundtrack_query: Query<Entity, With<IsSoundtrack>>,
) {
    for entity in &soundtrack_query {
        commands.entity(entity).despawn_recursive();
    }

    let soundtrack_key = match trigger.event() {
        PlaySoundtrack::Key(key) => key,
        PlaySoundtrack::Disable => return,
    };
    commands.spawn((
        AudioSourceBundle {
            source: soundtrack_handles[soundtrack_key].clone_weak(),
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
    Key(String),
    Disable,
}

/// Marker component for the soundtrack entity so we can find it later.
#[derive(Component, Reflect)]
#[reflect(Component)]
struct IsSoundtrack;
