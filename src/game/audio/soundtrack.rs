use bevy::{audio::PlaybackMode, prelude::*};

use crate::game::assets::{SoundtrackAsset, SoundtrackAssets};

pub(super) fn play_soundtrack(
    trigger: Trigger<Soundtrack>,
    mut commands: Commands,
    soundtracks: Res<SoundtrackAssets>,
    query: Query<Entity, With<SoundtrackMarker>>,
) {
    let event = trigger.event();
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    let source = match event {
        Soundtrack::Disable => {
            return;
        }
        Soundtrack::Credits => &soundtracks[&SoundtrackAsset::Credits],
        Soundtrack::Gameplay => &soundtracks[&SoundtrackAsset::Gameplay],
    }
    .clone_weak();

    let settings = PlaybackSettings {
        mode: PlaybackMode::Loop,
        ..default()
    };
    commands.spawn((AudioSourceBundle { source, settings }, SoundtrackMarker));
}

/// We mark our soundtrack entity so we can find it later.
#[derive(Component)]
pub(super) struct SoundtrackMarker;

/// Play or disable the soundtrack.
/// Playing a new soundtrack will overwrite the previous one.
/// Soundtracks will loop.
#[derive(Event)]
pub enum Soundtrack {
    Credits,
    Gameplay,
    Disable,
}
